use crate::ansi::*;

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum CsiMod {
    #[default]
    Standard,
    Equal,
    Question,
    Unknown(u8),
    Lt,
    Gt,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
enum StringKind {
    DeviceControl,
    Regular,
    Privacy,
    ApplicationProgramCommand,
    Os,
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
enum IgnoreKind {
    #[default]
    Regular,
    SequenceOverflow,
    ImmediateOverflow,
    Invalid,
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
enum State {
    #[default]
    Ground = 0,
    Escape,

    CsiP,
    CsiI,
    CsiIgnore(IgnoreKind),

    String(StringKind),

    Nf(bool),
}

pub type SizedAnsiParser<const BUF_CAP: usize> = AnsiParser<[u8; BUF_CAP]>;

pub type UnsizedAnsiParser = AnsiParser<[u8]>;

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "crepr", repr(C))]
pub struct Config {
    pub bit8_enabled: bool,
    pub del_special: bool,
    pub space_special: bool,

    pub csi_silent_sequence_overflow: bool,
    pub csi_silent_intermediate_overflow: bool,
    pub csi_pass_through_c0: bool,

    pub nf_silent_sequence_overflow: bool,

    pub utf8: bool,

    pub string_pass_through_c0: bool,
    pub utf8_strings: bool,

    pub max_immediate_count: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    pub const fn new() -> Self {
        Self {
            bit8_enabled: false,
            del_special: true,
            space_special: true,

            csi_silent_sequence_overflow: true,
            csi_silent_intermediate_overflow: true,
            csi_pass_through_c0: true,
            string_pass_through_c0: true,

            nf_silent_sequence_overflow: true,
            utf8: true,
            utf8_strings: true,

            max_immediate_count: 4,
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "crepr", repr(C))]
pub struct ParserState {
    immediate_count: usize,

    state: State,
    utf8_state: u8,
    codepoint: u32,

    buffer_count: usize,
}

impl Default for ParserState {
    fn default() -> Self {
        Self::new()
    }
}

impl ParserState {
    pub const fn new() -> Self {
        Self {
            immediate_count: 0,

            state: State::Ground,
            utf8_state: 0,
            codepoint: 0,

            buffer_count: 0,
        }
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "crepr", repr(C))]
pub struct AnsiParser<T: ?Sized> {
    pub cfg: Config,
    state: ParserState,
    buffer: T,
}

impl<const BYTE_BUF_SIZE: usize> core::default::Default for SizedAnsiParser<BYTE_BUF_SIZE> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const BYTE_BUF_SIZE: usize> SizedAnsiParser<BYTE_BUF_SIZE> {
    pub const fn new() -> Self {
        Self {
            cfg: Config::new(),
            state: ParserState::new(),
            buffer: [0; BYTE_BUF_SIZE],
        }
    }

    #[inline(always)]
    pub fn reset(&mut self) {
        let tc: &mut UnsizedAnsiParser = self;
        tc.reset();
    }

    #[inline(always)]
    pub fn next(&mut self, input: u8) -> Out<'_> {
        let tc: &mut UnsizedAnsiParser = self;
        tc.next(input)
    }
}

enum Utf8Result {
    Produce(char),
    Consume,
    Pass,
    InvalidSequence,
    InvalidCodepoint(u32),
}

impl UnsizedAnsiParser {
    #[cfg_attr(feature = "no_panic", no_panic::no_panic)]
    pub fn reset(&mut self) {
        self.state.state = State::Ground;
        self.state.buffer_count = 0;
    }

    #[cfg_attr(feature = "no_panic", no_panic::no_panic)]
    fn reset_byte_buffer(&mut self) {
        self.state.buffer_count = 0;
    }

    #[cfg_attr(feature = "no_panic", no_panic::no_panic)]
    fn current_byte_buffer(&self) -> &[u8] {
        if self.state.buffer_count > self.buffer.len() {
            &self.buffer[..]
        } else {
            &self.buffer[..self.state.buffer_count]
        }
    }

    #[cfg_attr(feature = "no_panic", no_panic::no_panic)]
    fn insert_into_byte_buffer(&mut self, input: u8) -> bool {
        if let Some(e) = self.buffer.get_mut(self.state.buffer_count) {
            *e = input;
            if let Some(r) = self.state.buffer_count.checked_add(1) {
                self.state.buffer_count = r;
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    #[cfg_attr(feature = "no_panic", no_panic::no_panic)]
    fn parse_safe_c0(&mut self, input: u8) -> Out<'_> {
        Out::C0(match input {
            0 => C0::NUL,
            1 => C0::SOH,
            2 => C0::STX,
            3 => C0::ETX,
            4 => C0::EOT,
            5 => C0::ENQ,
            6 => C0::ACK,
            7 => C0::BEL,
            8 => C0::BS,
            9 => C0::HT,
            10 => C0::LF,
            11 => C0::VT,
            12 => C0::FF,
            13 => C0::CR,
            14 => C0::SO,
            15 => C0::SI,
            16 => C0::DLE,
            17 => C0::DC1,
            18 => C0::DC2,
            19 => C0::DC3,
            20 => C0::DC4,
            21 => C0::NAK,
            22 => C0::SI,
            23 => C0::ETB,
            // 24 => C0::CAN,
            25 => C0::EM,
            // 26 => C0::SUB,
            // 27 => ESC
            28 => C0::FS,
            29 => C0::GS,
            30 => C0::RS,
            31 => C0::US,
            _ => return Out::None,
        })
    }

    #[cfg_attr(feature = "no_panic", no_panic::no_panic)]
    fn next_utf8(&mut self, input: u8) -> Utf8Result {
        match self.state.state {
            State::Ground if self.cfg.utf8 => {}
            State::String(_) if self.cfg.utf8_strings => {}
            _ => return Utf8Result::Pass,
        }
        if self.state.utf8_state != 0 {
            if input & 0b11000000 == 0b10000000 {
                self.state.codepoint = self.state.codepoint.wrapping_shl(6);
                self.state.codepoint |= input as u32 & !0b11000000;
                self.state.utf8_state -= 1;

                if self.state.utf8_state == 0 {
                    if let Some(char) = char::from_u32(self.state.codepoint) {
                        return Utf8Result::Produce(char);
                    } else {
                        return Utf8Result::InvalidCodepoint(self.state.codepoint);
                    }
                } else {
                    return Utf8Result::Consume;
                }
            } else {
                self.state.utf8_state = 0;
                return Utf8Result::InvalidSequence;
            }
        } else {
            for i in 2..=6 {
                let mask = 0xFF >> (i + 1);
                let eq = (!mask) << 1;
                if input & !mask == eq {
                    self.state.codepoint = (input & mask) as u32;
                    self.state.utf8_state = i - 1;
                    return Utf8Result::Consume;
                }
            }
        }
        Utf8Result::Pass
    }

    #[cfg_attr(feature = "no_panic", no_panic::no_panic)]
    fn push_p(&mut self, input: u8) {
        if !self.insert_into_byte_buffer(input) && !self.cfg.csi_silent_sequence_overflow {
            self.state.state = State::CsiIgnore(IgnoreKind::SequenceOverflow);
        }
    }

    #[cfg_attr(feature = "no_panic", no_panic::no_panic)]
    fn shift_csi(&mut self, input: u8) {
        let mut position = self
            .buffer
            .len()
            .saturating_sub(self.state.immediate_count)
            .saturating_sub(1);
        if position == 0 && matches!(self.buffer.first(), Some(b'?' | b'<' | b'>' | b'=')) {
            position += 1;
        }
        if !matches!(self.buffer.get(position), Some(0x20..=0x2F)) {
            while let (Some(v), Some(p)) = (
                self.buffer.get(position.wrapping_add(1)).copied(),
                self.buffer.get_mut(position),
            ) {
                *p = v;
                position += 1;
            }
        }

        if let Some(last) = self.buffer.last_mut() {
            *last = input
        }
    }

    #[cfg_attr(feature = "no_panic", no_panic::no_panic)]
    fn push_i(&mut self, input: u8) {
        if self.state.immediate_count == self.cfg.max_immediate_count {
            if !self.cfg.csi_silent_intermediate_overflow {
                self.state.state = State::CsiIgnore(IgnoreKind::ImmediateOverflow);
            }
            return;
        }
        if !self.insert_into_byte_buffer(input) {
            if !self.cfg.csi_silent_sequence_overflow {
                self.state.state = State::CsiIgnore(IgnoreKind::SequenceOverflow);
                return;
            }
            self.shift_csi(input);
        }
        self.state.immediate_count = self.state.immediate_count.wrapping_add(1);
    }

    #[cfg_attr(feature = "no_panic", no_panic::no_panic)]
    fn push_f(&mut self, input: u8) -> Out<'_> {
        if !self.insert_into_byte_buffer(input) {
            if !self.cfg.csi_silent_sequence_overflow {
                return Out::CSISequenceTooLarge;
            }
            self.shift_csi(input);
        }

        #[allow(clippy::useless_conversion)]
        Out::CSI(crate::CSI(self.current_byte_buffer().into()))
    }

    #[cfg_attr(feature = "no_panic", no_panic::no_panic)]
    pub fn next(&mut self, mut input: u8) -> Out<'_> {
        if self.cfg.utf8 | self.cfg.utf8_strings {
            match self.next_utf8(input) {
                Utf8Result::Produce(char) => match self.state.state {
                    State::String(str) => match str {
                        StringKind::DeviceControl => return Out::DCSData(char as crate::Mchar),
                        StringKind::Regular => return Out::SData(char as crate::Mchar),
                        StringKind::Privacy => return Out::PMData(char as crate::Mchar),
                        StringKind::ApplicationProgramCommand => {
                            return Out::APCData(char as crate::Mchar);
                        }
                        StringKind::Os => return Out::OSData(char as crate::Mchar),
                    },
                    _ => return Out::Data(char as crate::Mchar),
                },
                Utf8Result::Consume => return Out::None,
                Utf8Result::InvalidCodepoint(code) => return Out::InvalidCodepoint(code),
                Utf8Result::InvalidSequence => return Out::InvalidUtf8Sequence,
                Utf8Result::Pass => {}
            }
        }
        match input as u32 {
            24 | 26 => self.state.state = State::Ground,
            27 => self.state.state = State::Ground,
            0x80..=0x9F if self.cfg.bit8_enabled => {
                self.state.state = State::Escape;
                input -= 0x40;
            }
            _ => {}
        }
        match self.state.state {
            State::Ground => Out::C0(match input {
                0 => C0::NUL,
                1 => C0::SOH,
                2 => C0::STX,
                3 => C0::ETX,
                4 => C0::EOT,
                5 => C0::ENQ,
                6 => C0::ACK,
                7 => C0::BEL,
                8 => C0::BS,
                9 => C0::HT,
                10 => C0::LF,
                11 => C0::VT,
                12 => C0::FF,
                13 => C0::CR,
                14 => C0::SO,
                15 => C0::SI,
                16 => C0::DLE,
                17 => C0::DC1,
                18 => C0::DC2,
                19 => C0::DC3,
                20 => C0::DC4,
                21 => C0::NAK,
                22 => C0::SI,
                23 => C0::ETB,
                24 => C0::CAN,
                25 => C0::EM,
                26 => C0::SUB,
                27 => {
                    self.state.state = State::Escape;
                    return Out::None;
                }
                28 => C0::FS,
                29 => C0::GS,
                30 => C0::RS,
                31 => C0::US,
                32 if self.cfg.space_special => return Out::SP,
                127 if self.cfg.del_special => return Out::DEL,
                _ => return Out::Data(input as crate::Mchar),
            }),
            State::Escape => match input {
                0x20..=0x2F => {
                    self.reset_byte_buffer();
                    self.state.state = State::Nf(!self.insert_into_byte_buffer(input));
                    Out::None
                }
                0x30..=0x3F => {
                    // Fp
                    self.state.state = State::Ground;
                    Out::Fp(match input {
                        b'0' => Fp::UnknownX30,
                        b'1' => Fp::UnknownX31,
                        b'2' => Fp::UnknownX32,
                        b'3' => Fp::UnknownX33,
                        b'4' => Fp::UnknownX34,
                        b'5' => Fp::UnknownX35,
                        b'6' => Fp::DECFI,
                        b'7' => Fp::DECSC,
                        b'8' => Fp::DECRC,
                        b'9' => Fp::UnknownX39,
                        b':' => Fp::UnknownX3A,
                        b';' => Fp::UnknownX3B,
                        b'<' => Fp::UnknownX3C,
                        b'=' => Fp::DECKPAM,
                        b'>' => Fp::DECKPNM,
                        b'?' => Fp::UnknownX3F,
                        _ => unreachable!(),
                    })
                }
                0x40..=0x5F => {
                    // Fe
                    self.state.state = State::Ground;
                    Out::C1(match input {
                        b'@' => C1::PAD,
                        b'A' => C1::HOP,
                        b'B' => C1::BPH,
                        b'C' => C1::NBH,
                        b'D' => C1::IND,
                        b'E' => C1::NEL,
                        b'F' => C1::SSA,
                        b'G' => C1::ESA,
                        b'H' => C1::HTS,
                        b'I' => C1::HTJ,
                        b'J' => C1::VTS,
                        b'K' => C1::PLD,
                        b'L' => C1::PLU,
                        b'M' => C1::RI,
                        b'N' => C1::SS2,
                        b'O' => C1::SS3,
                        b'P' => {
                            self.state.state = State::String(StringKind::DeviceControl);
                            C1::DCS
                        }
                        b'Q' => C1::PU1,
                        b'R' => C1::PU2,
                        b'S' => C1::STS,
                        b'T' => C1::CCH,
                        b'U' => C1::MW,
                        b'V' => C1::SPA,
                        b'W' => C1::EPA,
                        b'X' => {
                            self.state.state = State::String(StringKind::Regular);
                            C1::SOS
                        }
                        b'Y' => C1::SGCI,
                        b'Z' => C1::SCI,
                        b'[' => {
                            self.state.state = State::CsiP;
                            self.reset_byte_buffer();
                            return Out::None;
                        }
                        b'\\' => C1::ST,
                        b']' => {
                            self.state.state = State::String(StringKind::Os);
                            C1::OSC
                        }
                        b'^' => {
                            self.state.state = State::String(StringKind::Privacy);
                            C1::PM
                        }
                        b'_' => {
                            self.state.state = State::String(StringKind::ApplicationProgramCommand);
                            C1::APC
                        }
                        _ => unreachable!(),
                    })
                }
                0x60..=0x7E => {
                    // Fs
                    self.state.state = State::Ground;
                    Out::Fs(match input {
                        b'`' => Fs::DMI,
                        b'a' => Fs::INT,
                        b'b' => Fs::EMI,
                        b'c' => Fs::RIS,
                        b'd' => Fs::CMD,
                        b'e' => Fs::UnknownX65,
                        b'f' => Fs::UnknownX66,
                        b'g' => Fs::UnknownX67,
                        b'h' => Fs::UnknownX68,
                        b'i' => Fs::UnknownX69,
                        b'j' => Fs::UnknownX6A,
                        b'k' => Fs::UnknownX6B,
                        b'l' => Fs::LCKMEM,
                        b'm' => Fs::ULKMEM,
                        b'n' => Fs::LS2,
                        b'o' => Fs::LS3,
                        b'p' => Fs::UnknownX70,
                        b'q' => Fs::UnknownX71,
                        b'r' => Fs::UnknownX72,
                        b's' => Fs::UnknownX73,
                        b't' => Fs::UnknownX74,
                        b'u' => Fs::UnknownX75,
                        b'v' => Fs::UnknownX76,
                        b'w' => Fs::UnknownX77,
                        b'x' => Fs::UnknownX78,
                        b'y' => Fs::UnknownX79,
                        b'z' => Fs::UnknownX7A,
                        b'{' => Fs::UnknownX7B,
                        b'|' => Fs::LS3R,
                        b'}' => Fs::LS2R,
                        b'~' => Fs::LS1R,
                        _ => unreachable!(),
                    })
                }
                _ => {
                    self.state.state = State::Ground;
                    Out::InvalidEscapeByte(input)
                }
            },
            State::Nf(err) => match input {
                0x20..=0x2f => {
                    self.state.state = State::Nf(!self.insert_into_byte_buffer(input));
                    Out::None
                }
                0x30..=0x7E => {
                    self.state.state = State::Ground;
                    if err
                        && !self.insert_into_byte_buffer(input)
                        && !self.cfg.nf_silent_sequence_overflow
                    {
                        Out::nFSequenceTooLarge
                    } else {
                        #[allow(clippy::useless_conversion)]
                        Out::nF(self.current_byte_buffer().into())
                    }
                }
                _ => {
                    self.state.state = State::Ground;
                    Out::nFInvalidSequence
                }
            },
            State::CsiP => match input {
                0x30..=0x3F => {
                    self.push_p(input);
                    Out::None
                }
                0x20..=0x2F => {
                    self.state.immediate_count = 0;
                    self.state.state = State::CsiI;
                    self.push_i(input);
                    Out::None
                }
                0x40..=0x7E => {
                    self.state.state = State::Ground;
                    self.push_f(input)
                }
                _ => {
                    self.state.state = State::CsiIgnore(IgnoreKind::Regular);
                    Out::None
                }
            },
            State::CsiI => match input {
                0x30..=0x3F => {
                    self.state.state = State::CsiIgnore(IgnoreKind::Invalid);
                    Out::None
                }
                0x20..=0x2F => {
                    self.push_i(input);
                    Out::None
                }
                0x40..=0x7E => {
                    self.state.state = State::Ground;
                    self.push_f(input)
                }
                _ => {
                    self.state.state = State::CsiIgnore(IgnoreKind::Regular);
                    Out::None
                }
            },
            State::CsiIgnore(kind) => match input {
                0x40..=0x7E => {
                    self.state.state = State::Ground;
                    match kind {
                        IgnoreKind::Regular => Out::None,
                        IgnoreKind::SequenceOverflow => Out::CSISequenceTooLarge,
                        IgnoreKind::ImmediateOverflow => Out::CSIIntermediateOverflow,
                        IgnoreKind::Invalid => Out::None,
                    }
                }
                _ => Out::None,
            },
            State::String(kind) => match input {
                0x00..=0x17 | 0x19 | 0x1C..=0x1F => {
                    if self.cfg.string_pass_through_c0 {
                        self.parse_safe_c0(input)
                    } else {
                        Out::None
                    }
                }
                c => match kind {
                    StringKind::DeviceControl => Out::DCSData(c as crate::Mchar),
                    StringKind::Regular => Out::SData(c as crate::Mchar),
                    StringKind::Os => Out::OSData(c as crate::Mchar),
                    StringKind::Privacy => Out::PMData(c as crate::Mchar),
                    StringKind::ApplicationProgramCommand => Out::APCData(c as crate::Mchar),
                },
            },
        }
    }
}
