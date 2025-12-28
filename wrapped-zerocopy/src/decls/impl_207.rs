macro_rules! deps {
    () => {
        TryFromBytes!();
        SendSyncPhantomData!();
        ValidityError!();
    };
}

macro_rules! impl_207 {
    () => {
        deps!();
        impl < Src : Clone , Dst : ? Sized + TryFromBytes > Clone for ValidityError < Src , Dst > { # [inline] fn clone (& self) -> Self { Self { src : self . src . clone () , _dst : SendSyncPhantomData :: default () } } }
    };
}

impl_207!();