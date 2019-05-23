// Candy Crash
type Num = &'static [u8];
type Item = &'static [u8];
type Ident = &'static [u8];
type Annotate = (&'static [u8]);

pub trait AST {
    fn nums(self) -> (&'static [u8], String);
    fn item(self) -> (&'static [u8]);
    fn crash<T>(self) -> T;
}

impl AST for &'static [u8] {
    fn nums(self) -> (&'static [u8], String) {
        let s = String::from_utf8(self.to_vec()).unwrap();
        let o = s.find('_');

        match o.is_none() {
            true => (self, "i32".to_string()),
            false => (&self[..o.unwrap()], s[(o.unwrap() + 1)..].to_string())
        }
    }

    fn item(self) -> Item {
        unimplemented!()
    }

    fn crash<T>(self) -> T {
        unimplemented!()
    }
}
