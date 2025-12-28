macro_rules! deps {
    () => {
        Binder!();
        Interner!();
        Lift!();
    };
}

macro_rules! impl_218 {
    () => {
        deps!();
        impl < I : Interner , U : Interner , T > Lift < U > for Binder < I , T > where T : Lift < U > , I :: BoundVarKinds : Lift < U , Lifted = U :: BoundVarKinds > , { type Lifted = Binder < U , T :: Lifted > ; fn lift_to_interner (self , cx : U) -> Option < Self :: Lifted > { Some (Binder { value : self . value . lift_to_interner (cx) ? , bound_vars : self . bound_vars . lift_to_interner (cx) ? , }) } }
    };
}

impl_218!();