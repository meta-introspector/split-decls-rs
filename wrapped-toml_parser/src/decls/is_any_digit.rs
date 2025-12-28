macro_rules! deps {
    () => {
        ScalarKind!();
    };
}

macro_rules! is_any_digit {
    () => {
        deps!();
        fn is_any_digit (b : u8 , kind : ScalarKind) -> bool { if kind == ScalarKind :: Float { is_dec_integer_digit (b) } else { is_any_integer_digit (b) } }
    };
}

is_any_digit!();