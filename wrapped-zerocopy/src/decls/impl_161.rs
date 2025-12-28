macro_rules! deps {
    () => {
        Unaligned!();
        CastError!();
        Immutable!();
        ByteSlice!();
        KnownLayout!();
    };
}

macro_rules! impl_161 {
    () => {
        deps!();
        impl < B , T > Ref < B , T > where B : ByteSlice , T : Unaligned + KnownLayout + Immutable + ? Sized , { # [deprecated (since = "0.8.0" , note = "use `Ref::from_bytes`; for `T: Unaligned`, the returned `CastError` implements `Into<SizeError>`")] # [doc (hidden)] # [must_use = "has no side effects"] # [inline (always)] pub fn new_unaligned (bytes : B) -> Option < Ref < B , T > > { Self :: from_bytes (bytes) . ok () } }
    };
}

impl_161!();