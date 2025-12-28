macro_rules! deps {
    () => {
        Ty!();
        RigidTy!();
    };
}

macro_rules! impl_329 {
    () => {
        deps!();
        impl RigidTy { # [doc = " Get the discriminant type for this type."] pub fn discriminant_ty (& self) -> Ty { with (| cx | cx . rigid_ty_discriminant_ty (self)) } }
    };
}

impl_329!()