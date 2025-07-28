use super::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ParsedCSI<'a> {
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
    SelectGraphicRendition(GraphicsRendition<'a>),

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

    Unknown(CSIParser<'a>),
    ReportedCursorPosition {
        row: u16,
        col: u16,
    },
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
    pub fn parse(&mut self) -> ParsedCSI<'a> {
        let copy = *self;
        if let Some(p) = self.parse_() {
            p
        } else {
            ParsedCSI::Unknown(copy)
        }
    }

    fn parse_(&mut self) -> Option<ParsedCSI<'a>> {
        Some(match (self.special_first(), self.final_identifier()) {
            (None, Some(b'f')) => {
                let [line, col] = self.parse_params([1, 1])?;
                ParsedCSI::HorizontalVerticalPosition {
                    line: line,
                    col: col,
                }
            }

            (None, Some(b'm')) => ParsedCSI::SelectGraphicRendition(GraphicsRendition(*self)),
            _ => None?,
        })
    }
}
