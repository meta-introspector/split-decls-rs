macro_rules! deps {
    () => {
        IntoByteSlice!();
        FromBytes!();
        Immutable!();
    };
}

macro_rules! impl_166 {
    () => {
        deps!();
        impl < 'a , B , T > Ref < B , [T] > where B : 'a + IntoByteSlice < 'a > , T : FromBytes + Immutable , { # [deprecated (since = "0.8.0" , note = "`Ref::into_ref` now supports slices")] # [doc (hidden)] # [inline (always)] pub fn into_slice (self) -> & 'a [T] { Ref :: into_ref (self) } }
    };
}

impl_166!()