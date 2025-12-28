macro_rules! deps {
    () => {
        RigidTy!();
        TyKind!();
    };
}

macro_rules! impl_330 {
    () => {
        deps!();
        impl From < RigidTy > for TyKind { fn from (value : RigidTy) -> Self { TyKind :: RigidTy (value) } }
    };
}

impl_330!();