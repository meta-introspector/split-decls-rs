macro_rules! deps {
    () => {
        Action!();
        Status!();
        XzDecoder!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        impl < W : Write > Write for XzDecoder < W > { fn write (& mut self , data : & [u8]) -> io :: Result < usize > { loop { self . dump () ? ; let before = self . total_in () ; let res = self . data . process_vec (data , & mut self . buf , Action :: Run) ? ; let written = (self . total_in () - before) as usize ; if written > 0 || data . len () == 0 || res == Status :: StreamEnd { return Ok (written) ; } } } fn flush (& mut self) -> io :: Result < () > { self . dump () ? ; self . obj . as_mut () . unwrap () . flush () } }
    };
}

impl_68!();