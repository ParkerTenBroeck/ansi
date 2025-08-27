use super::*;


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "crepr", repr(C))]
pub struct RGB{
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGB{
    pub fn new(r: u8, g: u8, b: u8) -> Self{
        RGB { r, g, b }
    }

    pub fn shade(w: u8) -> Self{
        Self::new(w, w, w)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "crepr", repr(C))]
pub struct VGA(pub u8);

impl VGA{
    pub fn as_rgb(self) -> RGB{
        match self.0{
            0 => RGB::new(0, 0, 0),
            1 => RGB::new(170, 0, 0),
            2 => RGB::new(0, 170, 0),
            3 => RGB::new(170, 80, 0),
            4 => RGB::new(0, 0, 170),
            5 => RGB::new(170, 0, 170),
            6 => RGB::new(0, 170, 170),
            7 => RGB::new(192, 192, 192),
            8 => RGB::new(170, 170, 170),
            9 => RGB::new(255, 0, 0),
            10 => RGB::new(0, 255, 0),
            11 => RGB::new(255, 255, 0),
            12 => RGB::new(0, 0, 255),
            13 => RGB::new(255, 0, 170),
            14 => RGB::new(0, 255, 255),
            15 => RGB::new(255, 255, 255),

            v @ 16..=231 => {
                let v = v - 16;
                let lookup = [0x0, 0x5f, 0x87, 0xaf, 0xd7, 0xff];

                RGB::new(
                    lookup[(v as usize / (6 * 6)) % 6],
                    lookup[(v as usize / (6)) % 6],
                    lookup[v as usize % 6],
                )
            }
            v@ 232..=255 => RGB::shade((v - 232) * 10 + 8)
        }
    }

    pub fn as_color(self) -> Color{
        match self.0{
            0 => Color::Black,
            1 => Color::Red,
            2 => Color::Green,
            3 => Color::Yellow,
            4 => Color::Blue,
            5 => Color::Magenta,
            6 => Color::Cyan,
            7 => Color::White,
            8 => Color::BrightBlack,
            9 => Color::BrightRed,
            10 => Color::BrightGreen,
            11 => Color::BrightYellow,
            12 => Color::BrightBlue,
            13 => Color::BrightMagenta,
            14 => Color::BrightCyan,
            15 => Color::BrightWhite,

            _ => Color::RGB(self.as_rgb())
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "crepr", repr(C))]
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

    VGA(VGA),
    RGB(RGB),

    NotPresent,
    Invalid(u16),
    LongNotPresnet,
    InvalidLong(u16),
    MalformedVGA,
    MalformedRGB,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
#[cfg_attr(feature = "crepr", repr(C))]
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
#[cfg_attr(feature = "crepr", repr(C))]
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
                CSIPart::SubParam(crate::FfiOption::Some(2)) => {
                    let Some([r, g, b]) = self.0.parse_sub_params([0, 0, 0]) else {
                        return Color::MalformedRGB;
                    };
                    if let (Ok(r), Ok(g), Ok(b)) = (r.try_into(), g.try_into(), b.try_into()) {
                        return Color::RGB(RGB{r, g, b});
                    } else {
                        return Color::MalformedVGA;
                    }
                }
                CSIPart::Param(crate::FfiOption::Some(2)) => {
                    let Some([r, g, b]) = self.0.parse_params([0, 0, 0]) else {
                        return Color::MalformedRGB;
                    };
                    if let (Ok(r), Ok(g), Ok(b)) = (r.try_into(), g.try_into(), b.try_into()) {
                        return Color::RGB(RGB{r, g, b});
                    } else {
                        return Color::MalformedVGA;
                    }
                }
                CSIPart::SubParam(crate::FfiOption::Some(5)) => {
                    let Some([vga]) = self.0.parse_sub_params([0]) else {
                        return Color::MalformedVGA;
                    };
                    if let Ok(vga) = vga.try_into() {
                        return Color::VGA(VGA(vga));
                    } else {
                        return Color::MalformedVGA;
                    }
                }
                CSIPart::Param(crate::FfiOption::Some(5)) => {
                    let Some([vga]) = self.0.parse_params([0]) else {
                        return Color::MalformedVGA;
                    };
                    if let Ok(vga) = vga.try_into() {
                        return Color::VGA(VGA(vga));
                    } else {
                        return Color::MalformedVGA;
                    }
                }
                CSIPart::Param(crate::FfiOption::Some(other)) => return Color::InvalidLong(other),
                CSIPart::SubParam(crate::FfiOption::Some(other)) => return Color::InvalidLong(other),
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
            CSIPart::Param(crate::FfiOption::None) => Some(SelectGraphic::Reset),
            CSIPart::Param(crate::FfiOption::Some(0)) => Some(SelectGraphic::Reset),
            CSIPart::Param(crate::FfiOption::Some(1)) => Some(SelectGraphic::Bold),
            CSIPart::Param(crate::FfiOption::Some(2)) => Some(SelectGraphic::Faint),
            CSIPart::Param(crate::FfiOption::Some(3)) => Some(SelectGraphic::Italic),
            CSIPart::Param(crate::FfiOption::Some(4)) => Some(SelectGraphic::Underline),
            CSIPart::Param(crate::FfiOption::Some(5)) => Some(SelectGraphic::SlowBlink),
            CSIPart::Param(crate::FfiOption::Some(6)) => Some(SelectGraphic::RapidBlink),
            CSIPart::Param(crate::FfiOption::Some(7)) => Some(SelectGraphic::InvertFgBg),
            CSIPart::Param(crate::FfiOption::Some(8)) => Some(SelectGraphic::Conceal),
            CSIPart::Param(crate::FfiOption::Some(9)) => Some(SelectGraphic::CrossedOut),
            CSIPart::Param(crate::FfiOption::Some(10)) => Some(SelectGraphic::PrimaryFont),
            CSIPart::Param(crate::FfiOption::Some(f @ 11..=19)) => {
                Some(SelectGraphic::AlternativeFont(f as u8 - 11))
            }
            CSIPart::Param(crate::FfiOption::Some(20)) => Some(SelectGraphic::Fraktur),
            CSIPart::Param(crate::FfiOption::Some(21)) => Some(SelectGraphic::DoublyUnderlined),
            CSIPart::Param(crate::FfiOption::Some(22)) => Some(SelectGraphic::NormalIntensity),
            CSIPart::Param(crate::FfiOption::Some(23)) => {
                Some(SelectGraphic::NeitherItalicNorBackletter)
            }
            CSIPart::Param(crate::FfiOption::Some(24)) => Some(SelectGraphic::NotUnderlined),
            CSIPart::Param(crate::FfiOption::Some(25)) => Some(SelectGraphic::NotBlinking),
            CSIPart::Param(crate::FfiOption::Some(26)) => Some(SelectGraphic::ProportionalSpacing),
            CSIPart::Param(crate::FfiOption::Some(27)) => Some(SelectGraphic::NotInvertedFgBg),
            CSIPart::Param(crate::FfiOption::Some(28)) => Some(SelectGraphic::Reveal),
            CSIPart::Param(crate::FfiOption::Some(29)) => Some(SelectGraphic::NotCrossedOut),
            CSIPart::Param(crate::FfiOption::Some(c @ (30..=39 | 90..=97))) => Some(
                SelectGraphic::Fg(self.parse_color(c, Some(39), Some(30), Some(90), Some(38))),
            ),
            CSIPart::Param(crate::FfiOption::Some(c @ (40..=49 | 100..=107))) => Some(
                SelectGraphic::Bg(self.parse_color(c, Some(49), Some(40), Some(100), Some(48))),
            ),
            CSIPart::Param(crate::FfiOption::Some(50)) => {
                Some(SelectGraphic::DisableProportionalSpacing)
            }
            CSIPart::Param(crate::FfiOption::Some(51)) => Some(SelectGraphic::Framed),
            CSIPart::Param(crate::FfiOption::Some(52)) => Some(SelectGraphic::Encircled),
            CSIPart::Param(crate::FfiOption::Some(53)) => Some(SelectGraphic::Overlined),
            CSIPart::Param(crate::FfiOption::Some(54)) => {
                Some(SelectGraphic::NeitherFramedNorEncircled)
            }
            CSIPart::Param(crate::FfiOption::Some(55)) => Some(SelectGraphic::NotOverlined),
            CSIPart::Param(crate::FfiOption::Some(c @ 58..=59)) => Some(
                SelectGraphic::UnderlineColor(self.parse_color(c, Some(59), None, None, Some(58))),
            ),
            CSIPart::Param(crate::FfiOption::Some(60)) => Some(SelectGraphic::IdeogramUnderline),
            CSIPart::Param(crate::FfiOption::Some(61)) => {
                Some(SelectGraphic::IdeogramDoubleUnderline)
            }
            CSIPart::Param(crate::FfiOption::Some(62)) => Some(SelectGraphic::IdeogramOverline),
            CSIPart::Param(crate::FfiOption::Some(63)) => {
                Some(SelectGraphic::IdeogramDoubleUnderline)
            }
            CSIPart::Param(crate::FfiOption::Some(64)) => Some(SelectGraphic::IdeogramStressMarking),
            CSIPart::Param(crate::FfiOption::Some(65)) => Some(SelectGraphic::IdeogramAttributes),
            CSIPart::Param(crate::FfiOption::Some(73)) => Some(SelectGraphic::Superscript),
            CSIPart::Param(crate::FfiOption::Some(74)) => Some(SelectGraphic::Subscript),
            CSIPart::Param(crate::FfiOption::Some(75)) => {
                Some(SelectGraphic::NeitherSuperscriptNorSubScript)
            }

            p => Some(SelectGraphic::Unknown(p)),
        }
    }
}
