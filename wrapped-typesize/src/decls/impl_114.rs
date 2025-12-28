macro_rules! deps {
    () => {
        TypeSize!();
    };
}

macro_rules! impl_114 {
    () => {
        deps!();
        impl < T : TypeSize > TypeSize for VecDeque < T > { fn extra_size (& self) -> usize { generic_vec_extra_size :: < T > (self . iter () , self . capacity () , self . len ()) } # [cfg (feature = "details")] fn get_collection_item_count (& self) -> Option < usize > { Some (self . len ()) } }
    };
}

impl_114!()