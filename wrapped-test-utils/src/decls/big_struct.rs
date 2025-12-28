macro_rules! big_struct {
    () => {
        pub fn big_struct () -> String { let n = 1_000 ; big_struct_n (n) }
    };
}

big_struct!();