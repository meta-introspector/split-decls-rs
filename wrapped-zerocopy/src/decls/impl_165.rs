macro_rules! deps {
    () => {
        Immutable!();
        Unaligned!();
        ByteSlice!();
        CastError!();
    };
}

macro_rules! impl_165 {
    () => {
        deps!();
        impl < B , T > Ref < B , [T] > where B : ByteSlice , T : Unaligned + Immutable , { # [deprecated (since = "0.8.0" , note = "`Ref::from_bytes` now supports slices; for `T: Unaligned`, the returned `CastError` implements `Into<SizeError>`")] # [doc (hidden)] # [inline (always)] pub fn new_slice_unaligned (bytes : B) -> Option < Ref < B , [T] > > { Ref :: from_bytes (bytes) . ok () } }
    };
}

impl_165!();