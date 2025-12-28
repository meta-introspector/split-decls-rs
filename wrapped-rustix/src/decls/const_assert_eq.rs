macro_rules! const_assert_eq {
    () => {
        macro_rules ! const_assert_eq { ($ x : expr , $ y : expr) => { assert_eq ! ($ x , $ y) ; } ; }
    };
}

const_assert_eq!()