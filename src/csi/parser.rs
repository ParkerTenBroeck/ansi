#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8, C)]
pub enum CSIPart {
    Param(Option<u16>),
    SubParam(Option<u16>),

    Question,
    Eq,
    Gt,
    Lt,

    Intermediate(u8),
    Final(u8),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum CSIParserState {
    Start,
    Middle,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(C)]
pub struct CSIParser<'a>(&'a [u8], CSIParserState);

impl<'a> CSIParser<'a> {
    #[cfg_attr(feature = "no_panic", no_panic::no_panic)]
    pub fn new(input: &'a [u8]) -> Self {
        Self(input, CSIParserState::Start)
    }

    #[cfg_attr(feature = "no_panic", no_panic::no_panic)]
    fn peek_first(&self) -> Option<u8> {
        self.0.first().copied()
    }

    #[cfg_attr(feature = "no_panic", no_panic::no_panic)]
    fn peek_last(&self) -> Option<u8> {
        self.0.last().copied()
    }

    #[cfg_attr(feature = "no_panic", no_panic::no_panic)]
    fn pop_front(&mut self) -> Option<u8> {
        match self.0 {
            [v, r @ ..] => {
                self.0 = r;
                Some(*v)
            }
            _ => None,
        }
    }

    #[cfg_attr(feature = "no_panic", no_panic::no_panic)]
    fn pop_back(&mut self) -> Option<u8> {
        match self.0 {
            [r @ .., v] => {
                self.0 = r;
                Some(*v)
            }
            _ => None,
        }
    }

    #[cfg_attr(feature = "no_panic", no_panic::no_panic)]
    pub fn special_first(&mut self) -> Option<u8> {
        if matches!(self.peek_first()?, b'?' | b'>' | b'<' | b'=') {
            return self.pop_front();
        }
        None
    }

    #[cfg_attr(feature = "no_panic", no_panic::no_panic)]
    pub fn final_identifier(&mut self) -> Option<u8> {
        if matches!(self.peek_last()?, 0x40..=0x7E) {
            return self.pop_back();
        }
        None
    }

    #[cfg_attr(feature = "no_panic", no_panic::no_panic)]
    pub fn parse_params<const N: usize>(&mut self, default: [u16; N]) -> Option<[u16; N]> {
        let mut result = [0; N];
        let mut fail = false;
        for i in 0..N {
            match self.peek() {
                Some(CSIPart::Param(p)) => {
                    result[i] = p.unwrap_or(default[i]);
                    self.next();
                }
                Some(CSIPart::SubParam(_)) => {
                    fail = true;
                    self.next();
                }
                Some(CSIPart::Intermediate(_) | CSIPart::Final(_)) | None => result[i] = default[i],
                _ => return None,
            }
        }
        if fail {
            return None;
        }
        Some(result)
    }

    #[cfg_attr(feature = "no_panic", no_panic::no_panic)]
    pub fn parse_sub_params<const N: usize>(&mut self, default: [u16; N]) -> Option<[u16; N]> {
        let mut result = [0; N];
        for i in 0..N {
            match self.peek() {
                Some(CSIPart::SubParam(p)) => {
                    result[i] = p.unwrap_or(default[i]);
                    self.next();
                }
                Some(CSIPart::Param(_) | CSIPart::Intermediate(_) | CSIPart::Final(_)) | None => {
                    result[i] = default[i]
                }
                _ => return None,
            }
        }
        Some(result)
    }

    #[cfg_attr(feature = "no_panic", no_panic::no_panic)]
    fn peek(&self) -> Option<CSIPart> {
        let mut copy = *self;
        copy.next()
    }

    pub fn empty(&self) -> bool {
        self.0.is_empty() || self.peek().is_none()
    }
}

impl<'a> Iterator for CSIParser<'a> {
    type Item = CSIPart;

    #[cfg_attr(feature = "no_panic", no_panic::no_panic)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.1 == CSIParserState::Start {
            if matches!(self.peek_first(), None | Some(0x20..=0x2F|0x40..=0x7E|b':'|b';')) {
                self.1 = CSIParserState::Middle;
                return Some(CSIPart::Param(None));
            } else if matches!(self.peek_first(), Some(b'0'..=b'9')) {
                self.1 = CSIParserState::Middle;
            }
        }
        let mut value = None;
        let sub;
        match self.pop_front() {
            Some(b'?') => return Some(CSIPart::Question),
            Some(b'=') => return Some(CSIPart::Eq),
            Some(b'>') => return Some(CSIPart::Gt),
            Some(b'<') => return Some(CSIPart::Lt),
            Some(v @ 0x20..=0x2F) => return Some(CSIPart::Intermediate(v)),
            Some(v @ 0x40..=0x7E) => return Some(CSIPart::Final(v)),
            Some(b':') => {
                sub = true;
            }
            Some(b';') => {
                sub = false;
            }
            Some(v @ b'0'..=b'9') => {
                sub = false;
                value = Some((v - b'0') as u16);
            }
            _ => return None,
        }
        while let Some(v @ b'0'..=b'9') = self.peek_first() {
            self.pop_front();
            let d = (v - b'0') as u16;
            if let Some(v) = value {
                value = Some(v.wrapping_mul(10).wrapping_add(d))
            } else {
                value = Some(d);
            }
        }
        if sub {
            Some(CSIPart::SubParam(value))
        } else {
            Some(CSIPart::Param(value))
        }
    }
}
