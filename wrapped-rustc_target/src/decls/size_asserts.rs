macro_rules! deps {
    () => {
        FnAbi!();
        ArgAbi!();
    };
}

macro_rules! size_asserts {
    () => {
        deps!();
        # [cfg (target_pointer_width = "64")] mod size_asserts { use rustc_data_structures :: static_assert_size ; use super :: * ; static_assert_size ! (ArgAbi <'_ , usize >, 56) ; static_assert_size ! (FnAbi <'_ , usize >, 80) ; }
    };
}

size_asserts!();