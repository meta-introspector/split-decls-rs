macro_rules! deps {
    () => {
        Slab!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl < T > Clone for Slab < T > where T : Clone , { fn clone (& self) -> Self { Self { entries : self . entries . clone () , len : self . len , next : self . next , } } fn clone_from (& mut self , source : & Self) { self . entries . clone_from (& source . entries) ; self . len = source . len ; self . next = source . next ; } }
    };
}

impl_6!()