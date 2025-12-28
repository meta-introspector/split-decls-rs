macro_rules! deps {
    () => {
        TrailingSliceLayout!();
        SliceDst!();
    };
}

macro_rules! SizeInfo {
    () => {
        deps!();
        # [cfg_attr (any (kani , test) , derive (Debug , PartialEq , Eq))] # [derive (Copy , Clone)] pub (crate) enum SizeInfo < E = usize > { Sized { size : usize } , SliceDst (TrailingSliceLayout < E >) , }
    };
}

SizeInfo!()