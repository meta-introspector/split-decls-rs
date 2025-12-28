macro_rules! deps {
    () => {
        AlignmentError!();
        SendSyncPhantomData!();
    };
}

macro_rules! impl_187 {
    () => {
        deps!();
        impl < Src : Clone , Dst : ? Sized > Clone for AlignmentError < Src , Dst > { # [inline] fn clone (& self) -> Self { Self { src : self . src . clone () , _dst : SendSyncPhantomData :: default () } } }
    };
}

impl_187!();