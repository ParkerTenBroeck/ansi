use super::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "crepr", repr(C))]
pub enum KnownCSI<'a> {
    CursorUp(u16),
    CursorDown(u16),
    CursorLeft(u16),
    CursorRight(u16),

    CursorNextLine(u16),
    CursorPreviousLine(u16),

    CursorHorizontalAbsolute(u16),
    CursorTo {
        row: u16,
        col: u16,
    },
    HorizontalVerticalPosition {
        row: u16,
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
    /// CSI r ; c R
    DeviceStatusReport,
    SelectGraphicRendition(GraphicsRendition<'a>),

    SaveCurrentCursorPosition,
    RestoreCurrentCursorPosition,
    ShowCursor,
    HideCursor,

    EnableFocusReporting,
    DisableFocusReporting,

    EnableBracketPastingMode,
    DisableBracketPastingMode,

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

    CursorLineAbsolute(u16),

    Unknown(CSIParser<'a>),
    ReportedCursorPosition {
        row: u16,
        col: u16,
    },
    ReportCursorPosition,
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

impl<'a> CSIParser<'a> {
    #[cfg_attr(feature = "no_panic", no_panic::no_panic)]
    pub fn parse(&mut self) -> KnownCSI<'a> {
        let copy = *self;
        if let Some(p) = self.parse_() {
            p
        } else {
            KnownCSI::Unknown(copy)
        }
    }

    #[cfg_attr(feature = "no_panic", no_panic::no_panic)]
    fn parse_(&mut self) -> Option<KnownCSI<'a>> {
        let copy = *self;
        let csi = match (self.special_first(), self.final_identifier()) {
            (None, Some(b'A')) => KnownCSI::CursorUp(self.parse_params([1])?[0]),
            (None, Some(b'B')) => KnownCSI::CursorDown(self.parse_params([1])?[0]),
            (None, Some(b'C')) => KnownCSI::CursorRight(self.parse_params([1])?[0]),
            (None, Some(b'D')) => KnownCSI::CursorLeft(self.parse_params([1])?[0]),
            (None, Some(b'E')) => KnownCSI::CursorNextLine(self.parse_params([1])?[0]),
            (None, Some(b'F')) => KnownCSI::CursorPreviousLine(self.parse_params([1])?[0]),
            (None, Some(b'G')) => KnownCSI::CursorHorizontalAbsolute(self.parse_params([1])?[0]),
            (None, Some(b'H')) => {
                let [row, col] = self.parse_params([1, 1])?;
                KnownCSI::CursorTo { row, col }
            }
            (None, Some(b'J')) => match self.parse_params([u16::MAX])?[0] {
                u16::MAX => KnownCSI::EraseDisplay,
                0 => KnownCSI::EraseFromCursor,
                1 => KnownCSI::EraseToCursor,
                2 => KnownCSI::EraseScreen,
                3 => KnownCSI::EraseSavedLines,
                _ => None?,
            },
            (None, Some(b'K')) => match self.parse_params([0])?[0] {
                0 => KnownCSI::EraseFromCursorToEndOfLine,
                1 => KnownCSI::EraseStartOfLineToCursor,
                2 => KnownCSI::EraseLine,
                _ => None?,
            },
            (None, Some(b'L')) => KnownCSI::InsertLines(self.parse_params([1])?[0]),
            (None, Some(b'M')) => KnownCSI::DeleteLines(self.parse_params([1])?[0]),
            (None, Some(b'S')) => KnownCSI::ScrollUp(self.parse_params([1])?[0]),
            (None, Some(b'T')) => KnownCSI::ScrollDown(self.parse_params([1])?[0]),

            (None, Some(b'f')) => {
                let [row, col] = self.parse_params([1, 1])?;
                KnownCSI::HorizontalVerticalPosition { row, col }
            }
            (Some(b'?'), Some(b'h')) => match self.parse_params([0])?[0] {
                0 => KnownCSI::ScreenMode(ScreenMode::Monochrome40x25),
                1 => KnownCSI::ScreenMode(ScreenMode::Color40x25),
                2 => KnownCSI::ScreenMode(ScreenMode::Monochrome80x25),
                3 => KnownCSI::ScreenMode(ScreenMode::Color80x25),
                4 => KnownCSI::ScreenMode(ScreenMode::Graphics4Color320x200),
                5 => KnownCSI::ScreenMode(ScreenMode::GraphicsMonochrome320x200),
                6 => KnownCSI::ScreenMode(ScreenMode::GraphicsMonochrome640x200),
                7 => KnownCSI::ScreenMode(ScreenMode::EnableLineWrapping),
                12 => KnownCSI::ScreenMode(ScreenMode::StopBlinkingCursor),
                13 => KnownCSI::ScreenMode(ScreenMode::GraphicsColor320x200),
                14 => KnownCSI::ScreenMode(ScreenMode::Graphics16Color640x200),
                15 => KnownCSI::ScreenMode(ScreenMode::GraphicsMonochrome630x350),
                16 => KnownCSI::ScreenMode(ScreenMode::Graphics16Color640x350),
                17 => KnownCSI::ScreenMode(ScreenMode::GraphicsMonochrome640x480),
                18 => KnownCSI::ScreenMode(ScreenMode::Graphics16Color640x480),
                19 => KnownCSI::ScreenMode(ScreenMode::Graphics256Color320x200),
                25 => KnownCSI::ShowCursor,
                1004 => KnownCSI::EnableFocusReporting,
                1049 => KnownCSI::EnableAlternativeBuffer,
                2004 => KnownCSI::EnableBracketPastingMode,
                _ => None?,
            },
            (None, Some(b'i')) => match self.parse_params([0])?[0] {
                4 => KnownCSI::AuxPortOff,
                5 => KnownCSI::AuxPortOn,
                _ => None?,
            },
            (Some(b'?'), Some(b'l')) => match self.parse_params([0])?[0] {
                0 => KnownCSI::ResetScreenMode(ScreenMode::Monochrome40x25),
                1 => KnownCSI::ResetScreenMode(ScreenMode::Color40x25),
                2 => KnownCSI::ResetScreenMode(ScreenMode::Monochrome80x25),
                3 => KnownCSI::ResetScreenMode(ScreenMode::Color80x25),
                4 => KnownCSI::ResetScreenMode(ScreenMode::Graphics4Color320x200),
                5 => KnownCSI::ResetScreenMode(ScreenMode::GraphicsMonochrome320x200),
                6 => KnownCSI::ResetScreenMode(ScreenMode::GraphicsMonochrome640x200),
                7 => KnownCSI::ResetScreenMode(ScreenMode::EnableLineWrapping),
                12 => KnownCSI::ResetScreenMode(ScreenMode::StopBlinkingCursor),
                13 => KnownCSI::ResetScreenMode(ScreenMode::GraphicsColor320x200),
                14 => KnownCSI::ResetScreenMode(ScreenMode::Graphics16Color640x200),
                15 => KnownCSI::ResetScreenMode(ScreenMode::GraphicsMonochrome630x350),
                16 => KnownCSI::ResetScreenMode(ScreenMode::Graphics16Color640x350),
                17 => KnownCSI::ResetScreenMode(ScreenMode::GraphicsMonochrome640x480),
                18 => KnownCSI::ResetScreenMode(ScreenMode::Graphics16Color640x480),
                19 => KnownCSI::ResetScreenMode(ScreenMode::Graphics256Color320x200),
                25 => KnownCSI::HideCursor,
                1004 => KnownCSI::DisableFocusReporting,
                1049 => KnownCSI::DisableAlternativeBuffer,
                2004 => KnownCSI::DisableBracketPastingMode,
                _ => None?,
            },
            (None, Some(b'm')) => {
                return Some(KnownCSI::SelectGraphicRendition(GraphicsRendition(*self)));
            }
            (None, Some(b'n')) => match self.parse_params([0])?[0] {
                5 => KnownCSI::DeviceStatusReport,
                6 => KnownCSI::ReportCursorPosition,
                _ => None?,
            },
            (None, Some(b'r')) => {
                let [top, bottom] = self.parse_params([1, 1])?;
                KnownCSI::SetScrollingRegion { top, bottom }
            }

            (None, Some(b's')) => KnownCSI::SaveCurrentCursorPosition,
            (None, Some(b'u')) => KnownCSI::RestoreCurrentCursorPosition,

            _ => None?,
        };
        if self.empty() {
            Some(csi)
        } else {
            Some(KnownCSI::Unknown(copy))
        }
    }
}
