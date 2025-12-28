macro_rules! other_13 {
    () => {
        unsafe extern "C" { fn strlen (s : PCSTR) -> usize ; }
    };
}

other_13!()