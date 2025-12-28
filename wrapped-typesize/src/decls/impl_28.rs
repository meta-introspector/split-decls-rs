macro_rules! deps {
    () => {
        TypeSize!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl < T : TypeSize , S > TypeSize for HashSet < T , S > { fn extra_size (& self) -> usize { generic_vec_extra_size (self . iter () , self . capacity () , self . len ()) } # [cfg (feature = "details")] fn get_collection_item_count (& self) -> Option < usize > { Some (self . len ()) } }
    };
}

impl_28!()