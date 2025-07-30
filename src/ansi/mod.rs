mod parser;
mod test;
pub use parser::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Out<'a> {
    Ansi(Ansi<'a>),
    Data(char),

    DCSData(char),
    SData(char),
    PMData(char),
    APCData(char),
    OSData(char),

    InvalidUtf8Sequence,
    InvalidCodepoint(u32),

    None,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Ansi<'a> {
    /// 0x00-0x31 + 0x32? + 0x7F?
    C0(C0),
    /// 0x1b or 8bit?
    C1(C1<'a>),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum C0 {
    /// Null
    NUL = 0,
    /// Start of Heading
    SOH = 1,
    /// Start of Text
    STX = 2,
    /// End of Text
    ETX = 3,
    /// End of Transmission
    EOT = 4,
    /// Enquiry (Terminal status / present)
    ENQ = 5,
    /// Acknowledge
    ACK = 6,
    /// Bell, Alert
    BEL = 7,
    /// Backspace
    BS = 8,
    /// Character/Horizontal Tabulation
    HT = 9,
    /// Line Feed
    LF = 10,
    /// Line/Vertical Tabulation
    VT = 11,
    /// Form Feed
    FF = 12,
    /// Carriage Return
    CR = 13,
    /// Shift Out (Switch to an alternative character set)
    SO = 14,
    /// Shift In (Switch to regular character set)
    SI = 15,
    /// Data Link Escape
    DLE = 16,
    /// Device Control One
    DC1 = 17,
    /// Device Control Two
    DC2 = 18,
    /// Drvice Control Three
    DC3 = 19,
    /// Device Control Four
    DC4 = 20,
    /// Negative Acknowledge
    NAK = 21,
    /// Synchronous Idle
    SYN = 22,
    /// End of Tranmission Block
    ETB = 23,
    /// Cancel
    CAN = 24,
    /// End of Medium
    EM = 25,
    /// Substitude
    SUB = 26,
    /// File Separator
    FS = 28,
    /// Group Separator
    GS = 29,
    /// Record Separator
    RS = 30,
    /// Unit Separator
    US = 31,
    /// Space
    SP = 32,
    /// Delete
    DEL = 127,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum C1<'a> {
    /// 0x20-0x2F,
    nF(nF<'a>),
    /// 0x30-0x3F,
    Fp(Fp),
    /// 0x40-0x5F,
    Fe(Fe<'a>),
    /// 0x60-0x7E,
    Fs(Fs),
    /// Not nF, Fp, Fe, Fs
    Invalid(u8),
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
/// A sequence starting with 0x1b with a character in the range 0x20-0x2F following
pub enum nF<'a> {
    Unknown(&'a [u8]),
    SequenceTooLarge,
    InvalidSequence,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
/// A sequence starting with 0x1b with a character in the range 0x30-0x3F following
pub enum Fp {
    UnknownX30 = b'0',
    UnknownX31 = b'1',
    UnknownX32 = b'2',
    UnknownX33 = b'3',
    UnknownX34 = b'4',
    UnknownX35 = b'5',
    /// Back Index
    DECFI = b'6',
    /// Save Cursor
    DECSC = b'7',
    /// Restore Cursor
    DECRC = b'8',
    UnknownX39 = b'9',
    UnknownX3A = b':',
    UnknownX3B = b';',
    UnknownX3C = b'<',
    /// Forward Index
    DECKPAM = b'=',
    /// Application Keypad
    DECKPNM = b'>',
    UnknownX3F = b'?',
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
/// A sequence starting with 0x1b with a character in the range 0x40-0x5F following
#[repr(u8)]
pub enum Fe<'a> {
    /// '@' Padding Character
    PAD = b'@',
    /// 'A' High Octet Preset
    HOP = b'A',
    /// ''B Break Permitted Here
    BPH = b'B',
    /// 'C' No Break Here
    NBH = b'C',
    /// 'D' Index
    IND = b'D',
    /// 'E' Next Line
    NEL = b'E',
    /// 'F' Start of Selected Area
    SSA = b'F',
    /// 'G' End of Selected Area
    ESA = b'G',
    /// 'H' Character Tabulation Set/Horizontal Tabulation Set
    HTS = b'H',
    /// 'I' Character Tabulation With Justification/Horizontal Tabulation With Justification
    HTJ = b'I',
    /// 'J' Line Tabulation Set/Vertical Tabulation Set
    VTS = b'J',
    /// 'K' Partial Line Forward/Partial Line Down
    PLD = b'K',
    /// 'L' Partial Line Backward/Partial Line Up
    PLU = b'L',
    /// 'M' Reverse Line Feed/Reverse Index
    RI = b'M',
    /// 'N' Single-Shift 2
    SS2 = b'N',
    /// 'O' Single-Shift 3
    SS3 = b'O',
    /// 'P' Device Control String
    DCS = b'P',
    /// 'Q' Private Use 1
    PU1 = b'Q',
    /// 'R' Private Use 2
    PU2 = b'R',
    /// 'S' Set Transmit State
    STS = b'S',
    /// 'T' Cancel character
    CCH = b'T',
    /// 'U' Message Waiting
    MW = b'U',
    /// 'V' Start of Protected Area
    SPA = b'V',
    /// 'W' End of Protected Area
    EPA = b'W',
    /// 'X' Start of String
    SOS = b'X',
    /// 'Y' Single Graphic Character Introducer
    SGCI = b'Y',
    /// 'Z' Single Character Introducer
    SCI = b'Z',
    /// '[' Control Sequence Introducer [CSI]
    CSI(crate::csi::CSIResult<'a>) = b'[',
    /// '\' String Terminator
    ST = b'\\',
    /// ']' Operating System Command
    OSC = b']',
    /// '^' Privacy Message
    PM = b'^',
    /// '_' Application Program Command
    APC = b'_',
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
/// A sequence starting with 0x1b with a character in the range 0x60-0x7E following
pub enum Fs {
    /// Disable manual input
    DMI = b'`',
    /// Interrupt
    INT = b'a',
    /// Enable manual input
    EMI = b'b',
    /// Reset to initial state
    RIS = b'c',
    /// Coding method delimiter
    CMD = b'd',

    UnknownX65 = b'e',
    UnknownX66 = b'f',
    UnknownX67 = b'g',
    UnknownX68 = b'h',
    UnknownX69 = b'i',
    UnknownX6A = b'j',
    UnknownX6B = b'k',

    /// Memory Lock (Locks memory above the cursor)
    LCKMEM = b'l',
    /// Memory Unlock
    ULKMEM = b'm',
    /// Locking shift two
    LS2 = b'n',
    /// Locking shift three
    LS3 = b'o',

    UnknownX70 = b'p',
    UnknownX71 = b'q',
    UnknownX72 = b'r',
    UnknownX73 = b's',
    UnknownX74 = b't',
    UnknownX75 = b'u',
    UnknownX76 = b'v',
    UnknownX77 = b'w',
    UnknownX78 = b'x',
    UnknownX79 = b'y',
    UnknownX7A = b'z',
    UnknownX7B = b'{',

    /// Locking shift three right
    LS3R = b'|',
    /// Locking shift two right
    LS2R = b'}',
    /// Locking shift one right
    LS1R = b'~',
}
