macro_rules! TrailingSliceLayout {
    () => {
        # [cfg_attr (any (kani , test) , derive (Debug , PartialEq , Eq))] # [derive (Copy , Clone)] pub (crate) struct TrailingSliceLayout < E = usize > { pub (crate) offset : usize , pub (crate) elem_size : E , }
    };
}

TrailingSliceLayout!();