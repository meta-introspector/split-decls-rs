macro_rules! deps {
    () => {
        ChangeSelfPath!();
    };
}

macro_rules! impl_108 {
    () => {
        deps!();
        impl ChangeSelfPath < '_ > { pub fn new < 'a > (self_ty : & 'a syn :: Type , trait_ : Option < (& 'a syn :: Path , & 'a HashSet < syn :: Ident >) > ,) -> ChangeSelfPath < 'a > { ChangeSelfPath { self_ty , trait_ } } }
    };
}

impl_108!();