mod parser;

pub use parser::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Number {
    PublicNone,
    Public(u16),
    PrivateNone,
    Private(u16),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CSI<'a> {
    Public(&'a [u8]),
    Private(&'a [u8]),

    SequenceTooLarge,
    IntermediateOverflow,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CSI_<'a> {
    CursorUp(u16),
    CursorDown(u16),
    CursorLeft(u16),
    CursorRight(u16),

    CursorNextLine(u16),
    CursorPreviousLine(u16),

    CursorHorizontalAbsolute(u16),
    CursorTo {
        line: u16,
        col: u16,
    },
    HorizontalVerticalPosition {
        line: u16,
        col: u16,
    },
    CursorPosition,

    EraseDisplay,
    EraseFromCursor,
    EraseToCursor,
    EraseScreen,

    EraseSavedLines,
    EraseFromCursorToEndOfLine,
    EraseStartOfLineToCursor,
    EraseLine,

    ScrollUp(u16),
    ScrollDown(u16),
    AuxPortOn,
    AuxPortOff,
    DeviceStatusReport,
    SelectGraphicRendition(parser::GraphicsRendition<'a>),

    SaveCurrentCursorPosition,
    RestoreCurrentCursorPosition,
    ShowCursor,
    HideCursor,

    EnableFocusReporting,
    DisableFocusReporting,

    RestoreScreen,
    SaveScreen,

    EnableAlternativeBuffer,
    DisableAlternativeBuffer,
    ScreenMode(ScreenMode),
    ResetScreenMode(ScreenMode),
    SetScrollingRegion {
        top: u16,
        bottom: u16,
    },
    DeleteLines(u16),
    InsertLines(u16),
    /// CSI r ; c R
    ReportCursorPosition,
    CursorLineAbsolute(u16),

    Unknown {
        sequence: &'a [u16],
        intermediate: &'a [u8],
        // modifier: CsiMod,
        end: char,
    },
    ReportedCursorPosition {
        row: u16,
        col: u16,
    },
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Color {
    Default,

    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,

    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,

    VGA(u8),
    RGB([u8; 3]),

    NotPresent,
    Invalid(u16),
    LongNotPresnet,
    InvalidLong(u16),
    MalformedVGA,
    MalformedRGB,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum ScreenMode {
    Monochrome40x25 = 0,
    Color40x25 = 1,
    Monochrome80x25 = 2,
    Color80x25 = 3,
    Graphics4Color320x200 = 4,
    GraphicsMonochrome320x200 = 5,
    GraphicsMonochrome640x200 = 6,
    EnableLineWrapping = 7,
    StopBlinkingCursor = 12,
    GraphicsColor320x200 = 13,
    Graphics16Color640x200 = 14,
    GraphicsMonochrome630x350 = 15,
    Graphics16Color640x350 = 16,
    GraphicsMonochrome640x480 = 17,
    Graphics16Color640x480 = 18,
    Graphics256Color320x200 = 19,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum SelectGraphic {
    Reset,
    Bold,
    Faint,
    Italic,
    Underline,
    SlowBlink,
    RapidBlink,
    InvertFgBg,
    Conceal,
    CrossedOut,
    PrimaryFont,
    AlternativeFont(u8),
    Fraktur,
    DoublyUnderlined,
    NormalIntensity,
    NeitherItalicNorBackletter,
    NotUnderlined,
    NotBlinking,
    ProportionalSpacing,
    NotInvertedFgBg,
    Reveal,
    NotCrossedOut,
    Fg(Color),
    Bg(Color),
    DisableProportionalSpacing,
    Framed,
    Encircled,
    Overlined,
    NeitherFramedNorEncircled,
    NotOverlined,
    UnderlineColor(Color),
    IdeogramUnderline,
    IdeogramDoubleUnderline,
    IdeogramOverline,
    IdeogramStressMarking,
    IdeogramAttributes,
    Superscript,
    Subscript,
    NeitherSuperscriptNorSubScript,

    Unknown(u16),
}
