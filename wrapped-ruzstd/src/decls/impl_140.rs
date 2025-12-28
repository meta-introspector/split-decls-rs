macro_rules! deps {
    () => {
        Error!();
        Read!();
        DecodeBuffer!();
    };
}

macro_rules! impl_140 {
    () => {
        deps!();
        impl Read for DecodeBuffer { fn read (& mut self , target : & mut [u8]) -> Result < usize , Error > { let max_amount = self . can_drain_to_window_size () . unwrap_or (0) ; let amount = max_amount . min (target . len ()) ; let mut written = 0 ; self . drain_to (amount , | buf | { target [written ..] [.. buf . len ()] . copy_from_slice (buf) ; written += buf . len () ; (buf . len () , Ok (())) }) ? ; Ok (amount) } }
    };
}

impl_140!();