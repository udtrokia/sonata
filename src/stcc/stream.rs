use crate::stcc::tyck::Atom;
/// # Stream
/// + at   => point the end of predication
/// + cons => convert stream to cons
pub(super) trait StreamTrait<T> {
    fn at<F: Fn(u8) -> bool>(self, begin: usize, predicate: F)  -> usize;
    fn cons(self) -> (&'static [T], &'static [T]);
}

impl StreamTrait<u8> for &'static [u8] {
    fn at<F: Fn(u8) -> bool>(self, begin: usize, predicate: F)  -> usize {
        self[begin..].iter().enumerate()
            .find(|(_, &x)| predicate(x))
            .map(|(i, _)| begin + i)
            .unwrap_or(self.len())
    }
    
    fn cons(self) -> (Atom, Atom) {
        if self.len() == 0 {
            return (b"", b"");
        }
        
        let mut pos = 1;
        match self[0] {
            b'(' => {
                let begin = pos;
                let mut next = self.at(pos, |x| x == b'(') + 1;
                let mut end = self.at(pos, |x| x == b')') + 1;
                while end > next {
                    pos = end;
                    next = self.at(pos, |x| x == b'(') + 1;
                    end = self.at(pos, |x| x == b')') + 1;
                }

                let mut _end = end;
                _end = self.at(end, |x| !x.is_ascii_whitespace());
                
                (&self[begin..(end - 1)], &self[_end..])
                // self[begin..(end - 1)].cons()
            },
            b'"' => {
                let begin = pos;
                let end = self.at(pos, |x| x == b'"');
                let _end = self.at(end, |x| !x.is_ascii_whitespace());

                (&self[(begin - 1)..end], &self[_end..])
            },
            b';' => {
                let end = self.at(pos, |x| x == b'\n');
                (&self[(pos - 1)..end], &self[end..])
            },
            x if x.is_ascii_whitespace() => {
                let end = self.at(pos, |x| !x.is_ascii_whitespace());
                self[end..].cons()
            },
            _ => {
                let begin = pos;
                let end = self.at(pos, |x| x.is_ascii_whitespace());
                let mut _end = self.at(end, |x| !x.is_ascii_whitespace());

                (&self[(begin - 1)..end], &self[_end..])
            }
        }
    }
}
