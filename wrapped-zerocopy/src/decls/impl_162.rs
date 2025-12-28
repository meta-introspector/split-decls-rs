macro_rules! deps {
    () => {
        Unaligned!();
        CastError!();
        SplitByteSlice!();
        KnownLayout!();
        Immutable!();
    };
}

macro_rules! impl_162 {
    () => {
        deps!();
        impl < B , T > Ref < B , T > where B : SplitByteSlice , T : Unaligned + KnownLayout + Immutable + ? Sized , { # [deprecated (since = "0.8.0" , note = "use `Ref::from_prefix`; for `T: Unaligned`, the returned `CastError` implements `Into<SizeError>`")] # [doc (hidden)] # [must_use = "has no side effects"] # [inline (always)] pub fn new_unaligned_from_prefix (bytes : B) -> Option < (Ref < B , T > , B) > { Self :: from_prefix (bytes) . ok () } }
    };
}

impl_162!()