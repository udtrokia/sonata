use crate::tyck::Num;

// Candy Crash
pub trait AST {
    fn num(self) -> Num;
    fn item(self) -> (&'static [u8]);
    fn crash<T>(self) -> T;
}

impl AST for &'static [u8] {
    fn num(self) -> Num {
        let s = self;
        let o = s.iter().enumerate()
            .find(|(_, x)| x == &&b'_')
            .unwrap_or((0, &b'0')).0;

        match o == 0 {
            true => (self, b"i32"),
            false => (&self[..o], &s[(o + 1)..])
        }
    }

    fn item(self) -> &'static [u8] {
        unimplemented!()
    }

    fn crash<T>(self) -> T {
        unimplemented!()
    }
}
