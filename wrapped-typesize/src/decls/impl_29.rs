macro_rules! deps {
    () => {
        TypeSize!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl < T : TypeSize > TypeSize for HashTable < T > { fn extra_size (& self) -> usize { generic_vec_extra_size (self . iter () , self . capacity () , self . len ()) } # [cfg (feature = "details")] fn get_collection_item_count (& self) -> Option < usize > { Some (self . len ()) } }
    };
}

impl_29!();