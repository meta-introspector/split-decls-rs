macro_rules! deps {
    () => {
        Immutable!();
        FromBytes!();
        IntoByteSliceMut!();
        IntoBytes!();
    };
}

macro_rules! impl_167 {
    () => {
        deps!();
        impl < 'a , B , T > Ref < B , [T] > where B : 'a + IntoByteSliceMut < 'a > , T : FromBytes + IntoBytes + Immutable , { # [deprecated (since = "0.8.0" , note = "`Ref::into_mut` now supports slices")] # [doc (hidden)] # [inline (always)] pub fn into_mut_slice (self) -> & 'a mut [T] { Ref :: into_mut (self) } }
    };
}

impl_167!()