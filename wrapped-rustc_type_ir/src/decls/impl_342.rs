macro_rules! deps {
    () => {
        OutlivesPredicate!();
        Interner!();
        Region!();
        Lift!();
    };
}

macro_rules! impl_342 {
    () => {
        deps!();
        impl < I : Interner , U : Interner , A > Lift < U > for OutlivesPredicate < I , A > where A : Lift < U > , I :: Region : Lift < U , Lifted = U :: Region > , { type Lifted = OutlivesPredicate < U , A :: Lifted > ; fn lift_to_interner (self , cx : U) -> Option < Self :: Lifted > { Some (OutlivesPredicate (self . 0 . lift_to_interner (cx) ? , self . 1 . lift_to_interner (cx) ?)) } }
    };
}

impl_342!()