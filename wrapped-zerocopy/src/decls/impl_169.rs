macro_rules! deps {
    () => {
        Immutable!();
        CastError!();
        Unaligned!();
        SplitByteSlice!();
    };
}

macro_rules! impl_169 {
    () => {
        deps!();
        impl < B , T > Ref < B , [T] > where B : SplitByteSlice , T : Unaligned + Immutable , { # [deprecated (since = "0.8.0" , note = "use `Ref::from_prefix_with_elems`; for `T: Unaligned`, the returned `CastError` implements `Into<SizeError>`")] # [doc (hidden)] # [must_use = "has no side effects"] # [inline (always)] pub fn new_slice_unaligned_from_prefix (bytes : B , count : usize) -> Option < (Ref < B , [T] > , B) > { Ref :: from_prefix_with_elems (bytes , count) . ok () } # [deprecated (since = "0.8.0" , note = "use `Ref::from_suffix_with_elems`; for `T: Unaligned`, the returned `CastError` implements `Into<SizeError>`")] # [doc (hidden)] # [must_use = "has no side effects"] # [inline (always)] pub fn new_slice_unaligned_from_suffix (bytes : B , count : usize) -> Option < (B , Ref < B , [T] >) > { Ref :: from_suffix_with_elems (bytes , count) . ok () } }
    };
}

impl_169!();