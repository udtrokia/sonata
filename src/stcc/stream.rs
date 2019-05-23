/// # Stream
/// at   => point the end of predication
/// cons => convert stream to cons
pub trait Stream<T> {
    fn at<F: Fn(u8) -> bool>(self, begin: usize, predicate: F)  -> usize;
    fn cons(self) -> (&'static [T], usize);
}

impl Stream<u8> for &'static [u8] {
    fn at<F: Fn(u8) -> bool>(self, begin: usize, predicate: F)  -> usize {
        self[begin..].iter().enumerate()
            .find(|(_, &x)| predicate(x))
            .map(|(i, _)| begin + i)
            .unwrap_or(self.len())
    }
    
    fn cons(self) -> (&'static [u8], usize) {
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

                (&self[(begin - 1)..end], end)
            },
            b'"' => {
                let begin = pos;
                let end = self.at(pos, |x| x == b'"');
                let _end = self.at(end, |x| !x.is_ascii_whitespace());

                (&self[(begin - 1)..end], _end)
            },
            b';' => {
                let end = self.at(pos, |x| x == b'\n');
                (&self[(pos - 1)..end], end)
            },
            _ => {
                let begin = pos;
                let end = self.at(pos, |x| x.is_ascii_whitespace());
                let _end = self.at(end, |x| !x.is_ascii_whitespace());

                (&self[(begin - 1)..end], _end)
            }
        }
    }
}
