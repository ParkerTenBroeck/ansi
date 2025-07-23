use crate::CsiMod;

use super::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct GraphicsRendition<'a>(&'a [u16]);

impl<'a> core::fmt::Debug for GraphicsRendition<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_list().entries(*self).finish()
    }
}

impl<'a> GraphicsRendition<'a> {
    fn parse_color(
        &mut self,
        val: u16,
        default: Option<u16>,
        reg_start: Option<u16>,
        bright_start: Option<u16>,
        long: Option<u16>,
    ) -> Color {
        if Some(val) == default {
            return Color::Default;
        }
        if Some(val) == long {
            let Some((long, rest)) = self.0.split_first() else {
                return Color::LongNotPresnet;
            };
            self.0 = rest;
            match *long {
                2 => {
                    let Some((rgb, rest)) = self.0.split_first_chunk::<3>() else {
                        return Color::MalformedRGB;
                    };
                    self.0 = rest;
                    if let (Ok(r), Ok(g), Ok(b)) =
                        (rgb[0].try_into(), rgb[1].try_into(), rgb[2].try_into())
                    {
                        return Color::RGB([r, g, b]);
                    } else {
                        return Color::MalformedVGA;
                    }
                }
                5 => {
                    let Some((vga, rest)) = self.0.split_first() else {
                        return Color::MalformedVGA;
                    };
                    self.0 = rest;
                    if let Ok(vga) = (*vga).try_into() {
                        return Color::VGA(vga);
                    } else {
                        return Color::MalformedVGA;
                    }
                }
                other => return Color::InvalidLong(other),
            }
        }
        if let Some(start) = reg_start {
            match val.wrapping_sub(start) {
                0 => return Color::Black,
                1 => return Color::Red,
                2 => return Color::Green,
                3 => return Color::Yellow,
                4 => return Color::Blue,
                5 => return Color::Magenta,
                6 => return Color::Cyan,
                7 => return Color::White,
                _ => {}
            }
        }
        if let Some(start) = bright_start {
            match val.wrapping_sub(start) {
                0 => return Color::BrightBlack,
                1 => return Color::BrightRed,
                2 => return Color::BrightGreen,
                3 => return Color::BrightYellow,
                4 => return Color::BrightBlue,
                5 => return Color::BrightMagenta,
                6 => return Color::BrightCyan,
                7 => return Color::BrightWhite,
                _ => {}
            }
        }
        Color::Invalid(val)
    }
}

impl<'a> Iterator for GraphicsRendition<'a> {
    type Item = SelectGraphic;

    fn next(&mut self) -> Option<Self::Item> {
        let (first, rest) = self.0.split_first()?;
        self.0 = rest;
        match *first {
            0 => Some(SelectGraphic::Reset),
            1 => Some(SelectGraphic::Bold),
            2 => Some(SelectGraphic::Faint),
            3 => Some(SelectGraphic::Italic),
            4 => Some(SelectGraphic::Underline),
            5 => Some(SelectGraphic::SlowBlink),
            6 => Some(SelectGraphic::RapidBlink),
            7 => Some(SelectGraphic::InvertFgBg),
            8 => Some(SelectGraphic::Conceal),
            9 => Some(SelectGraphic::CrossedOut),
            10 => Some(SelectGraphic::PrimaryFont),
            f @ 11..=19 => Some(SelectGraphic::AlternativeFont(f as u8 - 11)),
            20 => Some(SelectGraphic::Fraktur),
            21 => Some(SelectGraphic::DoublyUnderlined),
            22 => Some(SelectGraphic::NormalIntensity),
            23 => Some(SelectGraphic::NeitherItalicNorBackletter),
            24 => Some(SelectGraphic::NotUnderlined),
            25 => Some(SelectGraphic::NotBlinking),
            26 => Some(SelectGraphic::ProportionalSpacing),
            27 => Some(SelectGraphic::NotInvertedFgBg),
            28 => Some(SelectGraphic::Reveal),
            29 => Some(SelectGraphic::NotCrossedOut),
            c @ (30..=39 | 90..=97) => Some(SelectGraphic::Fg(self.parse_color(
                c,
                Some(39),
                Some(30),
                Some(90),
                Some(38),
            ))),
            c @ (40..=49 | 100..=107) => Some(SelectGraphic::Bg(self.parse_color(
                c,
                Some(49),
                Some(40),
                Some(100),
                Some(48),
            ))),
            50 => Some(SelectGraphic::DisableProportionalSpacing),
            51 => Some(SelectGraphic::Framed),
            52 => Some(SelectGraphic::Encircled),
            53 => Some(SelectGraphic::Overlined),
            54 => Some(SelectGraphic::NeitherFramedNorEncircled),
            55 => Some(SelectGraphic::NotOverlined),
            c @ 58..=59 => Some(SelectGraphic::UnderlineColor(self.parse_color(
                c,
                Some(59),
                None,
                None,
                Some(58),
            ))),
            60 => Some(SelectGraphic::IdeogramUnderline),
            61 => Some(SelectGraphic::IdeogramDoubleUnderline),
            62 => Some(SelectGraphic::IdeogramOverline),
            63 => Some(SelectGraphic::IdeogramDoubleUnderline),
            64 => Some(SelectGraphic::IdeogramStressMarking),
            65 => Some(SelectGraphic::IdeogramAttributes),
            73 => Some(SelectGraphic::Superscript),
            74 => Some(SelectGraphic::Subscript),
            75 => Some(SelectGraphic::NeitherSuperscriptNorSubScript),

            _ => Some(SelectGraphic::Unknown(*first)),
        }
    }
}

impl<'a> CSI<'a> {
    pub fn parse(csi_mod: CsiMod, params: &'a [u16], intermediates: &'a [u8], last: char) -> Self {
        match (csi_mod, params, intermediates, last) {
            (CsiMod::Standard, [line, col], [], 'f') => CSI::HorizontalVerticalPosition {
                line: *line,
                col: *col,
            },
            (CsiMod::Standard, [], [], 'f') => CSI::CursorTo { line: 1, col: 1 },

            (CsiMod::Standard, [line, col], [], 'H') => CSI::CursorTo {
                line: *line,
                col: *col,
            },
            (CsiMod::Standard, [], [], 'H') => CSI::CursorTo { line: 1, col: 1 },

            (CsiMod::Standard, [amount], [], 'A') => CSI::CursorUp(*amount),
            (CsiMod::Standard, [], [], 'A') => CSI::CursorUp(1),

            (CsiMod::Standard, [amount], [], 'B') => CSI::CursorDown(*amount),
            (CsiMod::Standard, [], [], 'B') => CSI::CursorDown(1),

            (CsiMod::Standard, [amount], [], 'C') => CSI::CursorRight(*amount),
            (CsiMod::Standard, [], [], 'C') => CSI::CursorRight(1),

            (CsiMod::Standard, [amount], [], 'D') => CSI::CursorLeft(*amount),
            (CsiMod::Standard, [], [], 'D') => CSI::CursorLeft(1),

            (CsiMod::Standard, [amount], [], 'E') => CSI::CursorNextLine(*amount),
            (CsiMod::Standard, [], [], 'E') => CSI::CursorNextLine(1),

            (CsiMod::Standard, [amount], [], 'F') => CSI::CursorPreviousLine(*amount),
            (CsiMod::Standard, [], [], 'F') => CSI::CursorPreviousLine(1),

            (CsiMod::Standard, [col], [], 'G') => CSI::CursorHorizontalAbsolute(*col),
            (CsiMod::Standard, [], [], 'G') => CSI::CursorHorizontalAbsolute(1),

            (CsiMod::Standard, [line], [], 'd') => CSI::CursorLineAbsolute(*line),
            (CsiMod::Standard, [], [], 'd') => CSI::CursorLineAbsolute(1),

            (CsiMod::Standard, [], [], 'J') => CSI::EraseDisplay,
            (CsiMod::Standard, [0], [], 'J') => CSI::EraseFromCursor,
            (CsiMod::Standard, [1], [], 'J') => CSI::EraseToCursor,
            (CsiMod::Standard, [2], [], 'J') => CSI::EraseScreen,
            (CsiMod::Standard, [3], [], 'J') => CSI::EraseSavedLines,

            (CsiMod::Standard, [], [], 'K') => CSI::EraseFromCursorToEndOfLine,
            (CsiMod::Standard, [0], [], 'K') => CSI::EraseFromCursorToEndOfLine,
            (CsiMod::Standard, [1], [], 'K') => CSI::EraseFromCursorToEndOfLine,
            (CsiMod::Standard, [2], [], 'K') => CSI::EraseLine,

            (CsiMod::Standard, [], [], 'L') => CSI::InsertLines(1),
            (CsiMod::Standard, [lines], [], 'L') => CSI::InsertLines(*lines),

            (CsiMod::Standard, [], [], 'M') => CSI::DeleteLines(1),
            (CsiMod::Standard, [lines], [], 'M') => CSI::DeleteLines(*lines),

            (CsiMod::Standard, [5], [], 'i') => CSI::AuxPortOn,
            (CsiMod::Standard, [4], [], 'i') => CSI::AuxPortOff,
            (CsiMod::Standard, [5], [], 'n') => CSI::DeviceStatusReport,
            (CsiMod::Standard, [6], [], 'n') => CSI::ReportCursorPosition,
            (CsiMod::Standard, [r, c], [], 'n') => CSI::ReportedCursorPosition { row: *r, col: *c },

            (CsiMod::Standard, [], [], 's') => CSI::SaveCurrentCursorPosition,
            (CsiMod::Standard, [], [], 'u') => CSI::RestoreCurrentCursorPosition,

            (CsiMod::Question, [25], [], 'h') => CSI::ShowCursor,
            (CsiMod::Question, [25], [], 'l') => CSI::HideCursor,

            (CsiMod::Question, [1004], [], 'h') => CSI::EnableFocusReporting,
            (CsiMod::Question, [1004], [], 'l') => CSI::DisableFocusReporting,

            (CsiMod::Question, [47], [], 'h') => CSI::RestoreScreen,
            (CsiMod::Question, [47], [], 'l') => CSI::SaveScreen,

            (CsiMod::Question, [1049], [], 'h') => CSI::EnableAlternativeBuffer,
            (CsiMod::Question, [1049], [], 'l') => CSI::DisableAlternativeBuffer,

            (CsiMod::Standard, [], [], 'm') => CSI::SelectGraphicRendition(GraphicsRendition(&[0])),
            (CsiMod::Standard, gr, [], 'm') => CSI::SelectGraphicRendition(GraphicsRendition(gr)),

            (CsiMod::Standard, [top, bottom], [], 'r') => CSI::SetScrollingRegion {
                top: *top,
                bottom: *bottom,
            },

            (CsiMod::Question, [0], [], 'h') => CSI::ScreenMode(ScreenMode::Monochrome40x25),
            (CsiMod::Question, [1], [], 'h') => CSI::ScreenMode(ScreenMode::Color40x25),
            (CsiMod::Question, [2], [], 'h') => CSI::ScreenMode(ScreenMode::Monochrome80x25),
            (CsiMod::Question, [3], [], 'h') => CSI::ScreenMode(ScreenMode::Color80x25),
            (CsiMod::Question, [4], [], 'h') => CSI::ScreenMode(ScreenMode::Graphics4Color320x200),
            (CsiMod::Question, [5], [], 'h') => {
                CSI::ScreenMode(ScreenMode::GraphicsMonochrome320x200)
            }
            (CsiMod::Question, [6], [], 'h') => {
                CSI::ScreenMode(ScreenMode::GraphicsMonochrome640x200)
            }
            (CsiMod::Question, [7], [], 'h') => CSI::ScreenMode(ScreenMode::EnableLineWrapping),
            (CsiMod::Question, [13], [], 'h') => CSI::ScreenMode(ScreenMode::GraphicsColor320x200),
            (CsiMod::Question, [14], [], 'h') => {
                CSI::ScreenMode(ScreenMode::Graphics16Color640x200)
            }
            (CsiMod::Question, [15], [], 'h') => {
                CSI::ScreenMode(ScreenMode::GraphicsMonochrome630x350)
            }
            (CsiMod::Question, [16], [], 'h') => {
                CSI::ScreenMode(ScreenMode::Graphics16Color640x350)
            }
            (CsiMod::Question, [17], [], 'h') => {
                CSI::ScreenMode(ScreenMode::GraphicsMonochrome640x480)
            }
            (CsiMod::Question, [18], [], 'h') => {
                CSI::ScreenMode(ScreenMode::Graphics16Color640x480)
            }
            (CsiMod::Question, [19], [], 'h') => {
                CSI::ScreenMode(ScreenMode::Graphics256Color320x200)
            }

            (CsiMod::Question, [0], [], 'l') => CSI::ResetScreenMode(ScreenMode::Monochrome40x25),
            (CsiMod::Question, [1], [], 'l') => CSI::ResetScreenMode(ScreenMode::Color40x25),
            (CsiMod::Question, [2], [], 'l') => CSI::ResetScreenMode(ScreenMode::Monochrome80x25),
            (CsiMod::Question, [3], [], 'l') => CSI::ResetScreenMode(ScreenMode::Color80x25),
            (CsiMod::Question, [4], [], 'l') => {
                CSI::ResetScreenMode(ScreenMode::Graphics4Color320x200)
            }
            (CsiMod::Question, [5], [], 'l') => {
                CSI::ResetScreenMode(ScreenMode::GraphicsMonochrome320x200)
            }
            (CsiMod::Question, [6], [], 'l') => {
                CSI::ResetScreenMode(ScreenMode::GraphicsMonochrome640x200)
            }
            (CsiMod::Question, [7], [], 'l') => {
                CSI::ResetScreenMode(ScreenMode::EnableLineWrapping)
            }
            (CsiMod::Question, [12], [], 'l') => {
                CSI::ResetScreenMode(ScreenMode::StopBlinkingCursor)
            }
            (CsiMod::Question, [13], [], 'l') => {
                CSI::ResetScreenMode(ScreenMode::GraphicsColor320x200)
            }
            (CsiMod::Question, [14], [], 'l') => {
                CSI::ResetScreenMode(ScreenMode::Graphics16Color640x200)
            }
            (CsiMod::Question, [15], [], 'l') => {
                CSI::ResetScreenMode(ScreenMode::GraphicsMonochrome630x350)
            }
            (CsiMod::Question, [16], [], 'l') => {
                CSI::ResetScreenMode(ScreenMode::Graphics16Color640x350)
            }
            (CsiMod::Question, [17], [], 'l') => {
                CSI::ResetScreenMode(ScreenMode::GraphicsMonochrome640x480)
            }
            (CsiMod::Question, [18], [], 'l') => {
                CSI::ResetScreenMode(ScreenMode::Graphics16Color640x480)
            }
            (CsiMod::Question, [19], [], 'l') => {
                CSI::ResetScreenMode(ScreenMode::Graphics256Color320x200)
            }

            (_, sequence, intermediate, end) => CSI::Unknown {
                sequence,
                intermediate,
                // modifier,
                end,
            },
        }
    }
}
