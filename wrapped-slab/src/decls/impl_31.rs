macro_rules! deps {
    () => {
        Slab!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl < T > fmt :: Debug for Slab < T > where T : fmt :: Debug , { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if fmt . alternate () { fmt . debug_map () . entries (self . iter ()) . finish () } else { fmt . debug_struct ("Slab") . field ("len" , & self . len) . field ("cap" , & self . capacity ()) . finish () } } }
    };
}

impl_31!();