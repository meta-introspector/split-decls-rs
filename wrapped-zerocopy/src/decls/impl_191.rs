macro_rules! deps {
    () => {
        AlignmentError!();
    };
}

macro_rules! impl_191 {
    () => {
        deps!();
        # [cfg (test)] impl < Src , Dst > AlignmentError < Src , Dst > { fn new_checked (src : Src) -> AlignmentError < Src , Dst > { assert_ne ! (core :: mem :: align_of ::< Dst > () , 1) ; unsafe { AlignmentError :: new_unchecked (src) } } }
    };
}

impl_191!();