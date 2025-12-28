macro_rules! verbose_disabled {
    () => {
        # [cfg (feature = "std")] # [test] fn verbose_disabled () { use crate :: VERBOSE ; assert_eq ! (VERBOSE , false) ; }
    };
}

verbose_disabled!()