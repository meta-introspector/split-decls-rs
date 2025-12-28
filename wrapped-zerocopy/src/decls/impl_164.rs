macro_rules! deps {
    () => {
        ByteSlice!();
        Immutable!();
    };
}

macro_rules! impl_164 {
    () => {
        deps!();
        impl < B , T > Ref < B , [T] > where B : ByteSlice , T : Immutable , { # [deprecated (since = "0.8.0" , note = "`Ref::from_bytes` now supports slices")] # [doc (hidden)] # [inline (always)] pub fn new_slice (bytes : B) -> Option < Ref < B , [T] > > { Self :: from_bytes (bytes) . ok () } }
    };
}

impl_164!()