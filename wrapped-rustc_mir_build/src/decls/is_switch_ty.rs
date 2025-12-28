macro_rules! is_switch_ty {
    () => {
        fn is_switch_ty (ty : Ty < '_ >) -> bool { ty . is_integral () || ty . is_char () }
    };
}

is_switch_ty!();