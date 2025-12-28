macro_rules! deps {
    () => {
        SplitByteSlice!();
        Immutable!();
    };
}

macro_rules! impl_168 {
    () => {
        deps!();
        impl < B , T > Ref < B , [T] > where B : SplitByteSlice , T : Immutable , { # [deprecated (since = "0.8.0" , note = "replaced by `Ref::from_prefix_with_elems`")] # [must_use = "has no side effects"] # [doc (hidden)] # [inline (always)] pub fn new_slice_from_prefix (bytes : B , count : usize) -> Option < (Ref < B , [T] > , B) > { Ref :: from_prefix_with_elems (bytes , count) . ok () } # [deprecated (since = "0.8.0" , note = "replaced by `Ref::from_suffix_with_elems`")] # [must_use = "has no side effects"] # [doc (hidden)] # [inline (always)] pub fn new_slice_from_suffix (bytes : B , count : usize) -> Option < (B , Ref < B , [T] >) > { Ref :: from_suffix_with_elems (bytes , count) . ok () } }
    };
}

impl_168!()