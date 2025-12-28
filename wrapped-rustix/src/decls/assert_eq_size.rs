macro_rules! assert_eq_size {
    () => {
        macro_rules ! assert_eq_size { ($ x : ty , $ y : ty) => { assert_eq ! (core :: mem :: size_of ::<$ x > () , core :: mem :: size_of ::<$ y > ()) ; } ; }
    };
}

assert_eq_size!()