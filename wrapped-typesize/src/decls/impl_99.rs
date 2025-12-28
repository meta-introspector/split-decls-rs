macro_rules! deps {
    () => {
        TypeSize!();
    };
}

macro_rules! impl_99 {
    () => {
        deps!();
        impl < T : TypeSize , S > TypeSize for HashSet < T , S > { fn extra_size (& self) -> usize { generic_vec_extra_size :: < T > (self . iter () , self . capacity () , self . len ()) } }
    };
}

impl_99!();