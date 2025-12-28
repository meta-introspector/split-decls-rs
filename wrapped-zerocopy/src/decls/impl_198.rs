macro_rules! deps {
    () => {
        SizeError!();
        SendSyncPhantomData!();
    };
}

macro_rules! impl_198 {
    () => {
        deps!();
        impl < Src : Clone , Dst : ? Sized > Clone for SizeError < Src , Dst > { # [inline] fn clone (& self) -> Self { Self { src : self . src . clone () , _dst : SendSyncPhantomData :: default () } } }
    };
}

impl_198!()