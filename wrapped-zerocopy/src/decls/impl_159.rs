macro_rules! deps {
    () => {
        SplitByteSlice!();
        KnownLayout!();
        Immutable!();
    };
}

macro_rules! impl_159 {
    () => {
        deps!();
        impl < B , T > Ref < B , T > where B : SplitByteSlice , T : KnownLayout + Immutable + ? Sized , { # [deprecated (since = "0.8.0" , note = "renamed to `Ref::from_prefix`")] # [doc (hidden)] # [must_use = "has no side effects"] # [inline (always)] pub fn new_from_prefix (bytes : B) -> Option < (Ref < B , T > , B) > { Self :: from_prefix (bytes) . ok () } }
    };
}

impl_159!()