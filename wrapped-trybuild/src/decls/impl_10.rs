macro_rules! deps {
    () => {
        Result!();
        Term!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl Write for Term { fn write (& mut self , mut buf : & [u8]) -> Result < usize > { if self . spec . is_none () { return self . stream . write (buf) ; } let len = buf . len () ; while ! buf . is_empty () { if self . start_of_line { let _ = self . stream . set_color (& self . spec) ; } match buf . iter () . position (| byte | * byte == b'\n') { Some (line_len) => { self . stream . write_all (& buf [.. line_len + 1]) ? ; self . start_of_line = true ; buf = & buf [line_len + 1 ..] ; } None => { self . stream . write_all (buf) ? ; self . start_of_line = false ; break ; } } } Ok (len) } fn flush (& mut self) -> Result < () > { self . stream . flush () } }
    };
}

impl_10!()