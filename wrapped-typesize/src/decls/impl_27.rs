macro_rules! deps {
    () => {
        TypeSize!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl < K : TypeSize , V : TypeSize , S > TypeSize for HashMap < K , V , S > { fn extra_size (& self) -> usize { generic_map_extra_size (self . iter () , self . capacity () , self . len ()) } # [cfg (feature = "details")] fn get_collection_item_count (& self) -> Option < usize > { Some (self . len ()) } }
    };
}

impl_27!();