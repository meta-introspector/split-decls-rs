macro_rules! deps {
    () => {
        Layered!();
    };
}

macro_rules! impl_106 {
    () => {
        deps!();
        impl < A , B , S > fmt :: Debug for Layered < A , B , S > where A : fmt :: Debug , B : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { # [cfg (all (feature = "registry" , feature = "std"))] let alt = f . alternate () ; let mut s = f . debug_struct ("Layered") ; # [cfg (all (feature = "registry" , feature = "std"))] { if alt { s . field ("inner_is_registry" , & self . inner_is_registry) . field ("has_layer_filter" , & self . has_layer_filter) . field ("inner_has_layer_filter" , & self . inner_has_layer_filter) ; } } s . field ("layer" , & self . layer) . field ("inner" , & self . inner) . finish () } }
    };
}

impl_106!()