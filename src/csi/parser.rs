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
    pub fn new(input: &'a [u8]) -> Self {
        Self(input, CSIParserState::Start)
    }

    fn peek_first(&self) -> Option<u8> {
        self.0.get(0).copied()
    }

    fn peek_last(&self) -> Option<u8> {
        self.0.get(self.0.len().wrapping_sub(1)).copied()
    }

    fn pop_front(&mut self) -> Option<u8> {
        match self.0 {
            [v, r @ ..] => {
                self.0 = r;
                Some(*v)
            }
            _ => None,
        }
    }

    fn pop_back(&mut self) -> Option<u8> {
        match self.0 {
            [r @ .., v] => {
                self.0 = r;
                Some(*v)
            }
            _ => None,
        }
    }

    pub fn special_first(&mut self) -> Option<u8> {
        if matches!(self.peek_first()?, b'?' | b'>' | b'<' | b'=') {
            return self.pop_front();
        }
        None
    }

    pub fn final_identifier(&mut self) -> Option<u8> {
        if matches!(self.peek_last()?, 0x40..=0x7E) {
            return self.pop_back();
        }
        None
    }

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

    fn peek(&mut self) -> Option<CSIPart> {
        let mut copy = *self;
        copy.next()
    }
}

impl<'a> Iterator for CSIParser<'a> {
    type Item = CSIPart;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.1 == CSIParserState::Start {
                self.1 = CSIParserState::Middle;
                if matches!(self.peek_first(), None | Some(0x20..=0x2F|0x40..=0x7E|b':'|b';')) {
                    return Some(CSIPart::Param(None));
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
                return Some(CSIPart::SubParam(value));
            } else {
                return Some(CSIPart::Param(value));
            }
        }
    }
}
