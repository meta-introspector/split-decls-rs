macro_rules! assert_eq_align {
    () => {
        macro_rules ! assert_eq_align { ($ x : ty , $ y : ty) => { assert_eq ! (core :: mem :: align_of ::<$ x > () , core :: mem :: align_of ::<$ y > ()) ; } ; }
    };
}

assert_eq_align!()