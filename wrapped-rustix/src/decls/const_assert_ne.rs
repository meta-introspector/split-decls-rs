macro_rules! const_assert_ne {
    () => {
        macro_rules ! const_assert_ne { ($ x : expr , $ y : expr) => { assert_ne ! ($ x , $ y) ; } ; }
    };
}

const_assert_ne!()