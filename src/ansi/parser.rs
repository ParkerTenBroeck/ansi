use crate::ansi::*;
use crate::csi::CSI;

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
enum State {
    #[default]
    Default,
    Escape,
    CsiStart,
    Csi,
    CsiIntermediate,

    String(StringKind),

    Nf(bool),
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum CsiStatus {
    Ok,
    Ignore,
    IntegerOverflow,
    SequenceTooLarge,
    IntermediateOverflow,
}

pub struct AnsiParser<const CSI_MAX_PARAMS: usize = 16, const BYTE_BUF_SIZE: usize = 8> {
    state: State,

    csi_param_count: u8,
    csi_params: [u16; CSI_MAX_PARAMS],
    csi_mod: CsiMod,
    csi_status: CsiStatus,

    byte_buffer_count: u8,
    byte_buffer: [u8; BYTE_BUF_SIZE],

    pub bit8_enabled: bool,
    pub del_special: bool,
    pub space_special: bool,

    pub csi_silent_integer_overflow: bool,
    pub csi_silent_sequence_overflow: bool,
    pub csi_silent_intermediate_overflow: bool,
    pub parse_csi: bool,
    pub csi_pass_through_c0: bool,

    pub nf_silent_sequence_overflow: bool,
}

impl<const CSI_MAX_PARAMS: usize, const BYTE_BUF_SIZE: usize> core::default::Default
    for AnsiParser<CSI_MAX_PARAMS, BYTE_BUF_SIZE>
{
    fn default() -> Self {
        Self::new()
    }
}

impl<const CSI_MAX_PARAMS: usize, const BYTE_BUF_SIZE: usize>
    AnsiParser<CSI_MAX_PARAMS, BYTE_BUF_SIZE>
{
    pub const fn new() -> Self {
        const {
            assert!(BYTE_BUF_SIZE < 256);
            assert!(CSI_MAX_PARAMS < 256);
        }
        Self {
            state: State::Default,
            csi_param_count: 0,
            // csi_param_count: 0,
            csi_params: [0; CSI_MAX_PARAMS],
            csi_mod: CsiMod::Standard,
            csi_status: CsiStatus::Ok,

            // byte buffer is used for csi intermediate characters and nF sequences
            byte_buffer_count: 0,
            byte_buffer: [0; BYTE_BUF_SIZE],

            bit8_enabled: false,
            del_special: true,
            space_special: true,

            csi_silent_integer_overflow: true,
            csi_silent_sequence_overflow: true,
            csi_silent_intermediate_overflow: true,
            parse_csi: true,
            csi_pass_through_c0: true,

            nf_silent_sequence_overflow: true,
        }
    }

    fn reset_byte_buffer(&mut self) {
        self.byte_buffer_count = 0;
    }

    fn current_byte_buffer(&self) -> &[u8] {
        &self.byte_buffer[..self.byte_buffer_count as usize]
    }

    fn insert_into_byte_buffer(&mut self, input: u8) -> Result<(), ()> {
        if let Some(e) = self.byte_buffer.get_mut(self.byte_buffer_count as usize) {
            *e = input;
            if let Some(r) = self.byte_buffer_count.checked_add(1) {
                self.byte_buffer_count = r;
                Ok(())
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }

    fn parse_safe_c0(&mut self, input: char) -> Out {
        Out::Ansi(Ansi::C0(match input as u32 {
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
            _ => unreachable!(),
        }))
    }

    pub fn next(&mut self, mut input: char) -> Out {
        loop {
            match input as u32 {
                24 | 26 => self.state = State::Default,
                27 => self.state = State::Default,
                0x80..=0x9F if self.bit8_enabled => {
                    self.state = State::Escape;
                    input = (input as u8 - 0x40) as char;
                }
                _ => {}
            }
            return match self.state {
                State::Default => Out::Ansi(Ansi::C0(match input as u32 {
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
                    _ => return Out::Data(input),
                })),
                State::Escape => match input as u32 {
                    0x20..=0x2F => {
                        self.reset_byte_buffer();
                        self.state = State::Nf(self.insert_into_byte_buffer(input as u8).is_err());
                        Out::None
                    }
                    0x30..=0x3F => {
                        // Fp
                        self.state = State::Default;
                        Out::Ansi(Ansi::C1(C1::Fp(match input {
                            '0' => Fp::UnknownX30,
                            '1' => Fp::UnknownX31,
                            '2' => Fp::UnknownX32,
                            '3' => Fp::UnknownX33,
                            '4' => Fp::UnknownX34,
                            '5' => Fp::UnknownX35,
                            '6' => Fp::DECFI,
                            '7' => Fp::DECSC,
                            '8' => Fp::DECRC,
                            '9' => Fp::UnknownX39,
                            ':' => Fp::UnknownX3A,
                            ';' => Fp::UnknownX3B,
                            '<' => Fp::UnknownX3C,
                            '=' => Fp::DECKPAM,
                            '>' => Fp::DECKPNM,
                            '?' => Fp::UnknownX3F,
                            _ => unreachable!(),
                        })))
                    }
                    0x40..=0x5F => {
                        // Fe
                        self.state = State::Default;
                        match input {
                            '@' => Out::Ansi(Ansi::C1(C1::Fe(Fe::PAD))),
                            'A' => Out::Ansi(Ansi::C1(C1::Fe(Fe::HOP))),
                            'B' => Out::Ansi(Ansi::C1(C1::Fe(Fe::BPH))),
                            'C' => Out::Ansi(Ansi::C1(C1::Fe(Fe::NBH))),
                            'D' => Out::Ansi(Ansi::C1(C1::Fe(Fe::IND))),
                            'E' => Out::Ansi(Ansi::C1(C1::Fe(Fe::NEL))),
                            'F' => Out::Ansi(Ansi::C1(C1::Fe(Fe::SSA))),
                            'G' => Out::Ansi(Ansi::C1(C1::Fe(Fe::ESA))),
                            'H' => Out::Ansi(Ansi::C1(C1::Fe(Fe::HTS))),
                            'I' => Out::Ansi(Ansi::C1(C1::Fe(Fe::HTJ))),
                            'J' => Out::Ansi(Ansi::C1(C1::Fe(Fe::VTS))),
                            'K' => Out::Ansi(Ansi::C1(C1::Fe(Fe::PLD))),
                            'L' => Out::Ansi(Ansi::C1(C1::Fe(Fe::PLU))),
                            'M' => Out::Ansi(Ansi::C1(C1::Fe(Fe::RI))),
                            'N' => Out::Ansi(Ansi::C1(C1::Fe(Fe::SS2))),
                            'O' => Out::Ansi(Ansi::C1(C1::Fe(Fe::SS3))),
                            'P' => {
                                self.state = State::String(StringKind::DeviceControl);
                                Out::Ansi(Ansi::C1(C1::Fe(Fe::DCS)))
                            }
                            'Q' => Out::Ansi(Ansi::C1(C1::Fe(Fe::PU1))),
                            'R' => Out::Ansi(Ansi::C1(C1::Fe(Fe::PU2))),
                            'S' => Out::Ansi(Ansi::C1(C1::Fe(Fe::STS))),
                            'T' => Out::Ansi(Ansi::C1(C1::Fe(Fe::CCH))),
                            'U' => Out::Ansi(Ansi::C1(C1::Fe(Fe::MW))),
                            'V' => Out::Ansi(Ansi::C1(C1::Fe(Fe::SPA))),
                            'W' => Out::Ansi(Ansi::C1(C1::Fe(Fe::EPA))),
                            'X' => {
                                self.state = State::String(StringKind::Regular);
                                Out::Ansi(Ansi::C1(C1::Fe(Fe::SOS)))
                            }
                            'Y' => Out::Ansi(Ansi::C1(C1::Fe(Fe::SGCI))),
                            'Z' => Out::Ansi(Ansi::C1(C1::Fe(Fe::SCI))),
                            '[' => {
                                self.state = State::CsiStart;
                                self.csi_status = CsiStatus::Ok;
                                self.reset_byte_buffer();
                                self.csi_param_count = 0;
                                self.csi_params[0] = 0;
                                self.csi_status = CsiStatus::Ok;
                                Out::None
                            }
                            '\\' => Out::Ansi(Ansi::C1(C1::Fe(Fe::ST))),
                            ']' => {
                                self.state = State::String(StringKind::Os);
                                Out::Ansi(Ansi::C1(C1::Fe(Fe::OSC)))
                            }
                            '^' => {
                                self.state = State::String(StringKind::Privacy);
                                Out::Ansi(Ansi::C1(C1::Fe(Fe::PM)))
                            }
                            '_' => {
                                self.state = State::String(StringKind::ApplicationProgramCommand);
                                Out::Ansi(Ansi::C1(C1::Fe(Fe::APC)))
                            }
                            _ => unreachable!(),
                        }
                    }
                    0x60..=0x7E => {
                        // Fs
                        self.state = State::Default;
                        Out::Ansi(Ansi::C1(C1::Fs(match input {
                            '`' => Fs::DMI,
                            'a' => Fs::INT,
                            'b' => Fs::EMI,
                            'c' => Fs::RIS,
                            'd' => Fs::CMD,
                            'e' => Fs::UnknownX65,
                            'f' => Fs::UnknownX66,
                            'g' => Fs::UnknownX67,
                            'h' => Fs::UnknownX68,
                            'i' => Fs::UnknownX69,
                            'j' => Fs::UnknownX6A,
                            'k' => Fs::UnknownX6B,
                            'l' => Fs::LCKMEM,
                            'm' => Fs::ULKMEM,
                            'n' => Fs::LS2,
                            'o' => Fs::LS3,
                            'p' => Fs::UnknownX70,
                            'q' => Fs::UnknownX71,
                            'r' => Fs::UnknownX72,
                            's' => Fs::UnknownX73,
                            't' => Fs::UnknownX74,
                            'u' => Fs::UnknownX75,
                            'v' => Fs::UnknownX76,
                            'w' => Fs::UnknownX77,
                            'x' => Fs::UnknownX78,
                            'y' => Fs::UnknownX79,
                            'z' => Fs::UnknownX7A,
                            '{' => Fs::UnknownX7B,
                            '|' => Fs::LS3R,
                            '}' => Fs::LS2R,
                            '~' => Fs::LS1R,
                            _ => unreachable!(),
                        })))
                    }
                    _ => {
                        self.state = State::Default;
                        Out::Ansi(Ansi::C1(C1::Invalid(input)))
                    }
                },
                State::Nf(err) => match input {
                    '\x20'..='\x2F' => {
                        self.state = State::Nf(self.insert_into_byte_buffer(input as u8).is_err());
                        Out::None
                    }
                    _ if err && !self.nf_silent_sequence_overflow => {
                        Out::Ansi(Ansi::C1(C1::nF(nF::SequenceTooLarge)))
                    }
                    _ => Out::Ansi(Ansi::C1(C1::nF(nF::Unknown(
                        self.current_byte_buffer(),
                        input,
                    )))),
                },
                State::CsiStart => match input {
                    '=' => {
                        self.state = State::Csi;
                        self.csi_mod = CsiMod::Equal;
                        Out::None
                    }
                    '?' => {
                        self.state = State::Csi;
                        self.csi_mod = CsiMod::Question;
                        Out::None
                    }
                    '<' => {
                        self.state = State::Csi;
                        self.csi_mod = CsiMod::Lt;
                        Out::None
                    }
                    '>' => {
                        self.state = State::Csi;
                        self.csi_mod = CsiMod::Standard;
                        Out::None
                    }
                    ':' => {
                        self.state = State::Csi;
                        self.csi_mod = CsiMod::Gt;
                        self.csi_status = CsiStatus::Ignore;
                        Out::None
                    }
                    _ => {
                        self.state = State::Csi;
                        self.csi_mod = CsiMod::Standard;
                        continue;
                    }
                },
                State::Csi | State::CsiIntermediate => match input {
                    '\x20'..='\x2f' => {
                        if self.insert_into_byte_buffer(input as u8).is_err()
                            && self.csi_status != CsiStatus::Ignore
                        {
                            self.csi_status = CsiStatus::IntermediateOverflow;
                        }

                        self.state = State::CsiIntermediate;
                        Out::None
                    }
                    '0'..='9' | '\x3a'..='\x3f' if self.state == State::CsiIntermediate => {
                        self.csi_status = CsiStatus::Ignore;
                        Out::None
                    }
                    d @ '0'..='9' => {
                        if self.csi_param_count == 0 {
                            self.csi_param_count = 1;
                        }
                        if let Some(v) = self.csi_params.get_mut(self.csi_param_count as usize - 1)
                        {
                            if let Some(nv) = v
                                .checked_mul(10)
                                .and_then(|v| v.checked_add((d as u8 - b'0') as u16))
                            {
                                *v = nv;
                            } else {
                                *v = v.wrapping_mul(10).wrapping_add((d as u8 - b'0') as u16);
                                if self.csi_status != CsiStatus::Ignore {
                                    self.csi_status = CsiStatus::IntegerOverflow;
                                }
                            }
                        } else if self.csi_status != CsiStatus::Ignore {
                            self.csi_status = CsiStatus::SequenceTooLarge;
                        }
                        Out::None
                    }
                    '\x3a' | '\x3c'..='\x3f' => {
                        self.csi_status = CsiStatus::Ignore;
                        Out::None
                    }
                    ';' => {
                        if let Some(next) = self.csi_param_count.checked_add(1) {
                            self.csi_param_count = next;
                            if let Some(next) =
                                self.csi_params.get_mut(self.csi_param_count as usize)
                            {
                                *next = 0;
                            } else if self.csi_status != CsiStatus::Ignore {
                                self.csi_status = CsiStatus::SequenceTooLarge;
                            }
                        } else if self.csi_status != CsiStatus::Ignore {
                            self.csi_status = CsiStatus::SequenceTooLarge;
                        }
                        Out::None
                    }
                    '\x7f' => Out::None,

                    c @ '\x40'..='\x7E' => {
                        self.state = State::Default;
                        match self.csi_status {
                            CsiStatus::Ignore => Out::None,
                            CsiStatus::IntegerOverflow if !self.csi_silent_integer_overflow => {
                                Out::Ansi(Ansi::C1(C1::Fe(Fe::CSI(CSI::IntegerOverflow))))
                            }
                            CsiStatus::SequenceTooLarge if !self.csi_silent_sequence_overflow => {
                                Out::Ansi(Ansi::C1(C1::Fe(Fe::CSI(CSI::SequenceTooLarge))))
                            }
                            CsiStatus::IntermediateOverflow
                                if !self.csi_silent_intermediate_overflow =>
                            {
                                Out::Ansi(Ansi::C1(C1::Fe(Fe::CSI(CSI::IntermediateOverflow))))
                            }
                            _ if self.parse_csi => self.parse_csi(c),
                            _ => Out::Ansi(Ansi::C1(C1::Fe(Fe::CSI(CSI::Unknown {
                                sequence: &self.csi_params[..self.csi_param_count as usize],
                                intermediate: self.current_byte_buffer(),
                                // modifier: self.csi_mod,
                                end: c,
                            })))),
                        }
                    }
                    '\x00'..='\x17' | '\x19' | '\x1C'..='\x1F' => self.parse_safe_c0(input),
                    _ => {
                        self.csi_status = CsiStatus::Ignore;
                        Out::None
                    }
                },
                State::String(kind) => match input {
                    '\x00'..='\x17' | '\x19' | '\x1C'..='\x1F' => Out::None,
                    c => match kind {
                        StringKind::DeviceControl => Out::Ansi(Ansi::C1(C1::Fe(Fe::DCSData(c)))),
                        StringKind::Regular => Out::Ansi(Ansi::C1(C1::Fe(Fe::SData(c)))),
                        StringKind::Os => Out::Ansi(Ansi::C1(C1::Fe(Fe::OSData(c)))),
                        StringKind::Privacy => Out::Ansi(Ansi::C1(C1::Fe(Fe::PMData(c)))),
                        StringKind::ApplicationProgramCommand => {
                            Out::Ansi(Ansi::C1(C1::Fe(Fe::APCData(c))))
                        }
                    },
                },
            };
        }
    }

    fn parse_csi(&mut self, c: char) -> Out {
        Out::Ansi(Ansi::C1(C1::Fe(Fe::CSI(CSI::parse(
            self.csi_mod,
            &self.csi_params[..self.csi_param_count as usize],
            self.current_byte_buffer(),
            c,
        )))))
    }
}
