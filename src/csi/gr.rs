use super::*;

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

    Unknown(CSIPart),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct GraphicsRendition<'a>(pub CSIParser<'a>);

impl<'a> core::fmt::Debug for GraphicsRendition<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_list().entries(*self).finish()
    }
}

impl<'a> GraphicsRendition<'a> {
    #[cfg_attr(feature = "no_panic", no_panic::no_panic)]
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
            let Some(long) = self.0.next() else {
                return Color::LongNotPresnet;
            };
            match long {
                CSIPart::SubParam(Some(2)) => {
                    let Some([r, g, b]) = self.0.parse_sub_params([0, 0, 0]) else {
                        return Color::MalformedRGB;
                    };
                    if let (Ok(r), Ok(g), Ok(b)) = (r.try_into(), g.try_into(), b.try_into()) {
                        return Color::RGB([r, g, b]);
                    } else {
                        return Color::MalformedVGA;
                    }
                }
                CSIPart::Param(Some(2)) => {
                    let Some([r, g, b]) = self.0.parse_params([0, 0, 0]) else {
                        return Color::MalformedRGB;
                    };
                    if let (Ok(r), Ok(g), Ok(b)) = (r.try_into(), g.try_into(), b.try_into()) {
                        return Color::RGB([r, g, b]);
                    } else {
                        return Color::MalformedVGA;
                    }
                }
                CSIPart::SubParam(Some(5)) => {
                    let Some([vga]) = self.0.parse_sub_params([0]) else {
                        return Color::MalformedVGA;
                    };
                    if let Ok(vga) = vga.try_into() {
                        return Color::VGA(vga);
                    } else {
                        return Color::MalformedVGA;
                    }
                }
                CSIPart::Param(Some(5)) => {
                    let Some([vga]) = self.0.parse_params([0]) else {
                        return Color::MalformedVGA;
                    };
                    if let Ok(vga) = vga.try_into() {
                        return Color::VGA(vga);
                    } else {
                        return Color::MalformedVGA;
                    }
                }
                CSIPart::Param(Some(other)) => return Color::InvalidLong(other),
                CSIPart::SubParam(Some(other)) => return Color::InvalidLong(other),
                _ => return Color::InvalidLong(0),
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

    #[cfg_attr(feature = "no_panic", no_panic::no_panic)]
    fn next(&mut self) -> Option<Self::Item> {
        match self.0.next()? {
            CSIPart::Param(None) => Some(SelectGraphic::Reset),
            CSIPart::Param(Some(0)) => Some(SelectGraphic::Reset),
            CSIPart::Param(Some(1)) => Some(SelectGraphic::Bold),
            CSIPart::Param(Some(2)) => Some(SelectGraphic::Faint),
            CSIPart::Param(Some(3)) => Some(SelectGraphic::Italic),
            CSIPart::Param(Some(4)) => Some(SelectGraphic::Underline),
            CSIPart::Param(Some(5)) => Some(SelectGraphic::SlowBlink),
            CSIPart::Param(Some(6)) => Some(SelectGraphic::RapidBlink),
            CSIPart::Param(Some(7)) => Some(SelectGraphic::InvertFgBg),
            CSIPart::Param(Some(8)) => Some(SelectGraphic::Conceal),
            CSIPart::Param(Some(9)) => Some(SelectGraphic::CrossedOut),
            CSIPart::Param(Some(10)) => Some(SelectGraphic::PrimaryFont),
            CSIPart::Param(Some(f @ 11..=19)) => Some(SelectGraphic::AlternativeFont(f as u8 - 11)),
            CSIPart::Param(Some(20)) => Some(SelectGraphic::Fraktur),
            CSIPart::Param(Some(21)) => Some(SelectGraphic::DoublyUnderlined),
            CSIPart::Param(Some(22)) => Some(SelectGraphic::NormalIntensity),
            CSIPart::Param(Some(23)) => Some(SelectGraphic::NeitherItalicNorBackletter),
            CSIPart::Param(Some(24)) => Some(SelectGraphic::NotUnderlined),
            CSIPart::Param(Some(25)) => Some(SelectGraphic::NotBlinking),
            CSIPart::Param(Some(26)) => Some(SelectGraphic::ProportionalSpacing),
            CSIPart::Param(Some(27)) => Some(SelectGraphic::NotInvertedFgBg),
            CSIPart::Param(Some(28)) => Some(SelectGraphic::Reveal),
            CSIPart::Param(Some(29)) => Some(SelectGraphic::NotCrossedOut),
            CSIPart::Param(Some(c @ (30..=39 | 90..=97))) => Some(SelectGraphic::Fg(
                self.parse_color(c, Some(39), Some(30), Some(90), Some(38)),
            )),
            CSIPart::Param(Some(c @ (40..=49 | 100..=107))) => Some(SelectGraphic::Bg(
                self.parse_color(c, Some(49), Some(40), Some(100), Some(48)),
            )),
            CSIPart::Param(Some(50)) => Some(SelectGraphic::DisableProportionalSpacing),
            CSIPart::Param(Some(51)) => Some(SelectGraphic::Framed),
            CSIPart::Param(Some(52)) => Some(SelectGraphic::Encircled),
            CSIPart::Param(Some(53)) => Some(SelectGraphic::Overlined),
            CSIPart::Param(Some(54)) => Some(SelectGraphic::NeitherFramedNorEncircled),
            CSIPart::Param(Some(55)) => Some(SelectGraphic::NotOverlined),
            CSIPart::Param(Some(c @ 58..=59)) => Some(SelectGraphic::UnderlineColor(
                self.parse_color(c, Some(59), None, None, Some(58)),
            )),
            CSIPart::Param(Some(60)) => Some(SelectGraphic::IdeogramUnderline),
            CSIPart::Param(Some(61)) => Some(SelectGraphic::IdeogramDoubleUnderline),
            CSIPart::Param(Some(62)) => Some(SelectGraphic::IdeogramOverline),
            CSIPart::Param(Some(63)) => Some(SelectGraphic::IdeogramDoubleUnderline),
            CSIPart::Param(Some(64)) => Some(SelectGraphic::IdeogramStressMarking),
            CSIPart::Param(Some(65)) => Some(SelectGraphic::IdeogramAttributes),
            CSIPart::Param(Some(73)) => Some(SelectGraphic::Superscript),
            CSIPart::Param(Some(74)) => Some(SelectGraphic::Subscript),
            CSIPart::Param(Some(75)) => Some(SelectGraphic::NeitherSuperscriptNorSubScript),

            p => Some(SelectGraphic::Unknown(p)),
        }
    }
}
