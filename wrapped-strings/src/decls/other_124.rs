macro_rules! deps {
    () => {
        PCSTR!();
    };
}

macro_rules! other_124 {
    () => {
        deps!();
        unsafe extern "C" { fn strlen (s : PCSTR) -> usize ; }
    };
}

other_124!();