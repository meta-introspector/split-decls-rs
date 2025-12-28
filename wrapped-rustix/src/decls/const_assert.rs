macro_rules! const_assert {
    () => {
        macro_rules ! const_assert { ($ x : expr) => { assert ! ($ x) ; } ; }
    };
}

const_assert!()