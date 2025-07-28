use super::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CSI<'a> {
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
    pub fn parse(&mut self) -> CSI<'a> {
        let copy = *self;
        if let Some(p) = self.parse_() {
            p
        } else {
            CSI::Unknown(copy)
        }
    }

    #[cfg_attr(feature = "no_panic", no_panic::no_panic)]
    fn parse_(&mut self) -> Option<CSI<'a>> {
        let copy = *self;
        let csi = match (self.special_first(), self.final_identifier()) {
            (None, Some(b'A')) => CSI::CursorUp(self.parse_params([1])?[0]),
            (None, Some(b'B')) => CSI::CursorDown(self.parse_params([1])?[0]),
            (None, Some(b'C')) => CSI::CursorRight(self.parse_params([1])?[0]),
            (None, Some(b'D')) => CSI::CursorLeft(self.parse_params([1])?[0]),
            (None, Some(b'E')) => CSI::CursorNextLine(self.parse_params([1])?[0]),
            (None, Some(b'F')) => CSI::CursorPreviousLine(self.parse_params([1])?[0]),
            (None, Some(b'G')) => CSI::CursorHorizontalAbsolute(self.parse_params([1])?[0]),
            (None, Some(b'H')) => {
                let [row, col] = self.parse_params([1, 1])?;
                CSI::CursorTo { row, col }
            }
            (None, Some(b'J')) => match self.parse_params([u16::MAX])?[0] {
                u16::MAX => CSI::EraseDisplay,
                0 => CSI::EraseFromCursor,
                1 => CSI::EraseToCursor,
                2 => CSI::EraseScreen,
                3 => CSI::EraseSavedLines,
                _ => None?,
            },
            (None, Some(b'K')) => match self.parse_params([0])?[0] {
                0 => CSI::EraseFromCursorToEndOfLine,
                1 => CSI::EraseStartOfLineToCursor,
                2 => CSI::EraseLine,
                _ => None?,
            },
            (None, Some(b'L')) => CSI::InsertLines(self.parse_params([1])?[0]),
            (None, Some(b'M')) => CSI::DeleteLines(self.parse_params([1])?[0]),
            (None, Some(b'S')) => CSI::ScrollUp(self.parse_params([1])?[0]),
            (None, Some(b'T')) => CSI::ScrollDown(self.parse_params([1])?[0]),

            (None, Some(b'f')) => {
                let [row, col] = self.parse_params([1, 1])?;
                CSI::HorizontalVerticalPosition { row, col }
            }
            (Some(b'?'), Some(b'h')) => match self.parse_params([0])?[0] {
                0 => CSI::ScreenMode(ScreenMode::Monochrome40x25),
                1 => CSI::ScreenMode(ScreenMode::Color40x25),
                2 => CSI::ScreenMode(ScreenMode::Monochrome80x25),
                3 => CSI::ScreenMode(ScreenMode::Color80x25),
                4 => CSI::ScreenMode(ScreenMode::Graphics4Color320x200),
                5 => CSI::ScreenMode(ScreenMode::GraphicsMonochrome320x200),
                6 => CSI::ScreenMode(ScreenMode::GraphicsMonochrome640x200),
                7 => CSI::ScreenMode(ScreenMode::EnableLineWrapping),
                12 => CSI::ScreenMode(ScreenMode::StopBlinkingCursor),
                13 => CSI::ScreenMode(ScreenMode::GraphicsColor320x200),
                14 => CSI::ScreenMode(ScreenMode::Graphics16Color640x200),
                15 => CSI::ScreenMode(ScreenMode::GraphicsMonochrome630x350),
                16 => CSI::ScreenMode(ScreenMode::Graphics16Color640x350),
                17 => CSI::ScreenMode(ScreenMode::GraphicsMonochrome640x480),
                18 => CSI::ScreenMode(ScreenMode::Graphics16Color640x480),
                19 => CSI::ScreenMode(ScreenMode::Graphics256Color320x200),
                25 => CSI::ShowCursor,
                1004 => CSI::EnableFocusReporting,
                1049 => CSI::EnableAlternativeBuffer,
                2004 => CSI::EnableBracketPastingMode,
                _ => None?,
            },
            (None, Some(b'i')) => match self.parse_params([0])?[0] {
                4 => CSI::AuxPortOff,
                5 => CSI::AuxPortOn,
                _ => None?,
            },
            (Some(b'?'), Some(b'l')) => match self.parse_params([0])?[0] {
                0 => CSI::ResetScreenMode(ScreenMode::Monochrome40x25),
                1 => CSI::ResetScreenMode(ScreenMode::Color40x25),
                2 => CSI::ResetScreenMode(ScreenMode::Monochrome80x25),
                3 => CSI::ResetScreenMode(ScreenMode::Color80x25),
                4 => CSI::ResetScreenMode(ScreenMode::Graphics4Color320x200),
                5 => CSI::ResetScreenMode(ScreenMode::GraphicsMonochrome320x200),
                6 => CSI::ResetScreenMode(ScreenMode::GraphicsMonochrome640x200),
                7 => CSI::ResetScreenMode(ScreenMode::EnableLineWrapping),
                12 => CSI::ResetScreenMode(ScreenMode::StopBlinkingCursor),
                13 => CSI::ResetScreenMode(ScreenMode::GraphicsColor320x200),
                14 => CSI::ResetScreenMode(ScreenMode::Graphics16Color640x200),
                15 => CSI::ResetScreenMode(ScreenMode::GraphicsMonochrome630x350),
                16 => CSI::ResetScreenMode(ScreenMode::Graphics16Color640x350),
                17 => CSI::ResetScreenMode(ScreenMode::GraphicsMonochrome640x480),
                18 => CSI::ResetScreenMode(ScreenMode::Graphics16Color640x480),
                19 => CSI::ResetScreenMode(ScreenMode::Graphics256Color320x200),
                25 => CSI::HideCursor,
                1004 => CSI::DisableFocusReporting,
                1049 => CSI::DisableAlternativeBuffer,
                2004 => CSI::DisableBracketPastingMode,
                _ => None?,
            },
            (None, Some(b'm')) => {
                return Some(CSI::SelectGraphicRendition(GraphicsRendition(*self)));
            }
            (None, Some(b'n')) => match self.parse_params([0])?[0] {
                5 => CSI::DeviceStatusReport,
                6 => CSI::ReportCursorPosition,
                _ => None?,
            },
            (None, Some(b'r')) => {
                let [top, bottom] = self.parse_params([1, 1])?;
                CSI::SetScrollingRegion { top, bottom }
            }

            (None, Some(b's')) => CSI::SaveCurrentCursorPosition,
            (None, Some(b'u')) => CSI::RestoreCurrentCursorPosition,

            _ => None?,
        };
        if self.empty() {
            Some(csi)
        } else {
            return Some(CSI::Unknown(copy));
        }
    }
}
