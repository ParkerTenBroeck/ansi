use crate::csi::CSIResult;
use crate::{CSIParser, ansi::*};

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
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
enum StringKind {
    DeviceControl,
    Regular,
    Privacy,
    ApplicationProgramCommand,
    Os,
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
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

pub type SizedAnsiParser<const BUF_CAP: usize = 256> = AnsiParser<[u8; BUF_CAP]>;
pub type UnsizedAnsiParser = AnsiParser<[u8]>;

#[repr(C)]
#[derive(Debug)]
pub struct AnsiParser<T: ?Sized> {
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
    immediate_count: usize,

    state: State,
    utf8_state: u8,
    codepoint: u32,

    buffer_count: usize,
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
            immediate_count: 0,

            state: State::Ground,
            utf8_state: 0,
            codepoint: 0,

            buffer_count: 0,
            buffer: [0; BYTE_BUF_SIZE],
        }
    }

    #[inline(always)]
    pub fn reset(&mut self) {
        let tc: &mut UnsizedAnsiParser = self;
        tc.reset();
    }

    #[inline(always)]
    pub fn next(&mut self, input: u8) -> Out {
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
        self.state = State::Ground;
        self.buffer_count = 0;
    }

    #[cfg_attr(feature = "no_panic", no_panic::no_panic)]
    fn reset_byte_buffer(&mut self) {
        self.buffer_count = 0;
    }

    #[cfg_attr(feature = "no_panic", no_panic::no_panic)]
    fn current_byte_buffer(&self) -> &[u8] {
        if self.buffer_count > self.buffer.len() {
            &self.buffer[..]
        } else {
            &self.buffer[..self.buffer_count]
        }
    }

    #[cfg_attr(feature = "no_panic", no_panic::no_panic)]
    fn insert_into_byte_buffer(&mut self, input: u8) -> Result<(), ()> {
        if let Some(e) = self.buffer.get_mut(self.buffer_count) {
            *e = input;
            if let Some(r) = self.buffer_count.checked_add(1) {
                self.buffer_count = r;
                Ok(())
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }

    #[cfg_attr(feature = "no_panic", no_panic::no_panic)]
    fn parse_safe_c0(&mut self, input: u8) -> Out {
        Out::Ansi(Ansi::C0(match input {
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
        }))
    }

    #[cfg_attr(feature = "no_panic", no_panic::no_panic)]
    fn next_utf8(&mut self, input: u8) -> Utf8Result {
        match self.state {
            State::Ground if self.utf8 => {}
            State::String(_) if self.utf8_strings => {}
            _ => return Utf8Result::Pass,
        }
        if self.utf8_state != 0 {
            if input & 0b11000000 == 0b10000000 {
                self.codepoint = self.codepoint.wrapping_shl(6);
                self.codepoint |= input as u32 & !0b11000000;
                self.utf8_state -= 1;

                if self.utf8_state == 0 {
                    if let Some(char) = char::from_u32(self.codepoint) {
                        return Utf8Result::Produce(char);
                    } else {
                        return Utf8Result::InvalidCodepoint(self.codepoint);
                    }
                } else {
                    return Utf8Result::Consume;
                }
            } else {
                self.utf8_state = 0;
                return Utf8Result::InvalidSequence;
            }
        } else {
            for i in 2..=6 {
                let mask = 0xFF >> (i + 1);
                let eq = (!mask) << 1;
                if input & !mask == eq {
                    self.codepoint = (input & mask) as u32;
                    self.utf8_state = i - 1;
                    return Utf8Result::Consume;
                }
            }
        }
        Utf8Result::Pass
    }

    #[cfg_attr(feature = "no_panic", no_panic::no_panic)]
    fn push_p(&mut self, input: u8) {
        if self.insert_into_byte_buffer(input).is_err() && !self.csi_silent_sequence_overflow {
            self.state = State::CsiIgnore(IgnoreKind::SequenceOverflow);
        }
    }

    #[cfg_attr(feature = "no_panic", no_panic::no_panic)]
    fn shift_csi(&mut self, input: u8) {
        let mut position = self
            .buffer
            .len()
            .saturating_sub(self.immediate_count)
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
        if self.immediate_count == self.max_immediate_count {
            if !self.csi_silent_intermediate_overflow {
                self.state = State::CsiIgnore(IgnoreKind::ImmediateOverflow);
            }
            return;
        }
        if self.insert_into_byte_buffer(input).is_err() {
            if !self.csi_silent_sequence_overflow {
                self.state = State::CsiIgnore(IgnoreKind::SequenceOverflow);
                return;
            }
            self.shift_csi(input);
        }
        self.immediate_count = self.immediate_count.wrapping_add(1);
    }

    #[cfg_attr(feature = "no_panic", no_panic::no_panic)]
    fn push_f(&mut self, input: u8) -> Out {
        if self.insert_into_byte_buffer(input).is_err() {
            if !self.csi_silent_sequence_overflow {
                return Out::Ansi(Ansi::C1(C1::Fe(Fe::CSI(CSIResult::SequenceTooLarge))));
            }
            self.shift_csi(input);
        }

        Out::Ansi(Ansi::C1(C1::Fe(Fe::CSI(CSIResult::Sequence(
            CSIParser::new(self.current_byte_buffer()),
        )))))
    }

    #[cfg_attr(feature = "no_panic", no_panic::no_panic)]
    pub fn next(&mut self, mut input: u8) -> Out {
        if self.utf8 | self.utf8_strings {
            match self.next_utf8(input) {
                Utf8Result::Produce(char) => match self.state {
                    State::String(str) => match str {
                        StringKind::DeviceControl => return Out::DCSData(char),
                        StringKind::Regular => return Out::SData(char),
                        StringKind::Privacy => return Out::PMData(char),
                        StringKind::ApplicationProgramCommand => return Out::APCData(char),
                        StringKind::Os => return Out::OSData(char),
                    },
                    _ => return Out::Data(char),
                },
                Utf8Result::Consume => return Out::None,
                Utf8Result::InvalidCodepoint(code) => return Out::InvalidCodepoint(code),
                Utf8Result::InvalidSequence => return Out::InvalidUtf8Sequence,
                Utf8Result::Pass => {}
            }
        }
        match input as u32 {
            24 | 26 => self.state = State::Ground,
            27 => self.state = State::Ground,
            0x80..=0x9F if self.bit8_enabled => {
                self.state = State::Escape;
                input -= 0x40;
            }
            _ => {}
        }
        match self.state {
            State::Ground => Out::Ansi(Ansi::C0(match input {
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
                    self.state = State::Escape;
                    return Out::None;
                }
                28 => C0::FS,
                29 => C0::GS,
                30 => C0::RS,
                31 => C0::US,
                32 if self.space_special => C0::SP,
                127 if self.del_special => C0::DEL,
                _ => return Out::Data(input as char),
            })),
            State::Escape => match input {
                0x20..=0x2F => {
                    self.reset_byte_buffer();
                    self.state = State::Nf(self.insert_into_byte_buffer(input).is_err());
                    Out::None
                }
                0x30..=0x3F => {
                    // Fp
                    self.state = State::Ground;
                    Out::Ansi(Ansi::C1(C1::Fp(match input {
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
                    })))
                }
                0x40..=0x5F => {
                    // Fe
                    self.state = State::Ground;
                    match input {
                        b'@' => Out::Ansi(Ansi::C1(C1::Fe(Fe::PAD))),
                        b'A' => Out::Ansi(Ansi::C1(C1::Fe(Fe::HOP))),
                        b'B' => Out::Ansi(Ansi::C1(C1::Fe(Fe::BPH))),
                        b'C' => Out::Ansi(Ansi::C1(C1::Fe(Fe::NBH))),
                        b'D' => Out::Ansi(Ansi::C1(C1::Fe(Fe::IND))),
                        b'E' => Out::Ansi(Ansi::C1(C1::Fe(Fe::NEL))),
                        b'F' => Out::Ansi(Ansi::C1(C1::Fe(Fe::SSA))),
                        b'G' => Out::Ansi(Ansi::C1(C1::Fe(Fe::ESA))),
                        b'H' => Out::Ansi(Ansi::C1(C1::Fe(Fe::HTS))),
                        b'I' => Out::Ansi(Ansi::C1(C1::Fe(Fe::HTJ))),
                        b'J' => Out::Ansi(Ansi::C1(C1::Fe(Fe::VTS))),
                        b'K' => Out::Ansi(Ansi::C1(C1::Fe(Fe::PLD))),
                        b'L' => Out::Ansi(Ansi::C1(C1::Fe(Fe::PLU))),
                        b'M' => Out::Ansi(Ansi::C1(C1::Fe(Fe::RI))),
                        b'N' => Out::Ansi(Ansi::C1(C1::Fe(Fe::SS2))),
                        b'O' => Out::Ansi(Ansi::C1(C1::Fe(Fe::SS3))),
                        b'P' => {
                            self.state = State::String(StringKind::DeviceControl);
                            Out::Ansi(Ansi::C1(C1::Fe(Fe::DCS)))
                        }
                        b'Q' => Out::Ansi(Ansi::C1(C1::Fe(Fe::PU1))),
                        b'R' => Out::Ansi(Ansi::C1(C1::Fe(Fe::PU2))),
                        b'S' => Out::Ansi(Ansi::C1(C1::Fe(Fe::STS))),
                        b'T' => Out::Ansi(Ansi::C1(C1::Fe(Fe::CCH))),
                        b'U' => Out::Ansi(Ansi::C1(C1::Fe(Fe::MW))),
                        b'V' => Out::Ansi(Ansi::C1(C1::Fe(Fe::SPA))),
                        b'W' => Out::Ansi(Ansi::C1(C1::Fe(Fe::EPA))),
                        b'X' => {
                            self.state = State::String(StringKind::Regular);
                            Out::Ansi(Ansi::C1(C1::Fe(Fe::SOS)))
                        }
                        b'Y' => Out::Ansi(Ansi::C1(C1::Fe(Fe::SGCI))),
                        b'Z' => Out::Ansi(Ansi::C1(C1::Fe(Fe::SCI))),
                        b'[' => {
                            self.state = State::CsiP;
                            self.reset_byte_buffer();
                            Out::None
                        }
                        b'\\' => Out::Ansi(Ansi::C1(C1::Fe(Fe::ST))),
                        b']' => {
                            self.state = State::String(StringKind::Os);
                            Out::Ansi(Ansi::C1(C1::Fe(Fe::OSC)))
                        }
                        b'^' => {
                            self.state = State::String(StringKind::Privacy);
                            Out::Ansi(Ansi::C1(C1::Fe(Fe::PM)))
                        }
                        b'_' => {
                            self.state = State::String(StringKind::ApplicationProgramCommand);
                            Out::Ansi(Ansi::C1(C1::Fe(Fe::APC)))
                        }
                        _ => unreachable!(),
                    }
                }
                0x60..=0x7E => {
                    // Fs
                    self.state = State::Ground;
                    Out::Ansi(Ansi::C1(C1::Fs(match input {
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
                    })))
                }
                _ => {
                    self.state = State::Ground;
                    Out::Ansi(Ansi::C1(C1::Invalid(input)))
                }
            },
            State::Nf(err) => match input {
                0x20..=0x2f => {
                    self.state = State::Nf(self.insert_into_byte_buffer(input).is_err());
                    Out::None
                }
                0x30..=0x7E => {
                    self.state = State::Ground;
                    if err
                        && self.insert_into_byte_buffer(input).is_err()
                        && !self.nf_silent_sequence_overflow
                    {
                        Out::Ansi(Ansi::C1(C1::nF(nF::SequenceTooLarge)))
                    } else {
                        Out::Ansi(Ansi::C1(C1::nF(nF::Unknown(self.current_byte_buffer()))))
                    }
                }
                _ => {
                    self.state = State::Ground;
                    Out::Ansi(Ansi::C1(C1::nF(nF::InvalidSequence)))
                }
            },
            State::CsiP => match input {
                0x30..=0x3F => {
                    self.push_p(input);
                    Out::None
                }
                0x20..=0x2F => {
                    self.immediate_count = 0;
                    self.state = State::CsiI;
                    self.push_i(input);
                    Out::None
                }
                0x40..=0x7E => {
                    self.state = State::Ground;
                    self.push_f(input)
                }
                _ => {
                    self.state = State::CsiIgnore(IgnoreKind::Regular);
                    Out::None
                }
            },
            State::CsiI => match input {
                0x30..=0x3F => {
                    self.state = State::CsiIgnore(IgnoreKind::Invalid);
                    Out::None
                }
                0x20..=0x2F => {
                    self.push_i(input);
                    Out::None
                }
                0x40..=0x7E => {
                    self.state = State::Ground;
                    self.push_f(input)
                }
                _ => {
                    self.state = State::CsiIgnore(IgnoreKind::Regular);
                    Out::None
                }
            },
            State::CsiIgnore(kind) => match input {
                0x40..=0x7E => {
                    self.state = State::Ground;
                    match kind {
                        IgnoreKind::Regular => Out::None,
                        IgnoreKind::SequenceOverflow => {
                            Out::Ansi(Ansi::C1(C1::Fe(Fe::CSI(CSIResult::SequenceTooLarge))))
                        }
                        IgnoreKind::ImmediateOverflow => {
                            Out::Ansi(Ansi::C1(C1::Fe(Fe::CSI(CSIResult::IntermediateOverflow))))
                        }
                        IgnoreKind::Invalid => Out::None,
                    }
                }
                _ => Out::None,
            },
            State::String(kind) => match input {
                0x00..=0x17 | 0x19 | 0x1C..=0x1F => {
                    if self.string_pass_through_c0 {
                        self.parse_safe_c0(input)
                    } else {
                        Out::None
                    }
                }
                c => match kind {
                    StringKind::DeviceControl => Out::DCSData(c as char),
                    StringKind::Regular => Out::SData(c as char),
                    StringKind::Os => Out::OSData(c as char),
                    StringKind::Privacy => Out::PMData(c as char),
                    StringKind::ApplicationProgramCommand => Out::APCData(c as char),
                },
            },
        }
    }
}
