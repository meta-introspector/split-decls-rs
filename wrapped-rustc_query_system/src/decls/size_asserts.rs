macro_rules! deps {
    () => {
        DepNode!();
        DepKind!();
    };
}

macro_rules! size_asserts {
    () => {
        deps!();
        # [cfg (target_pointer_width = "64")] mod size_asserts { use rustc_data_structures :: static_assert_size ; use super :: * ; static_assert_size ! (DepKind , 2) ; # [cfg (any (target_arch = "x86" , target_arch = "x86_64"))] static_assert_size ! (DepNode , 18) ; # [cfg (not (any (target_arch = "x86" , target_arch = "x86_64")))] static_assert_size ! (DepNode , 24) ; }
    };
}

size_asserts!()