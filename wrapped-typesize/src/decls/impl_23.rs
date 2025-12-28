macro_rules! deps {
    () => {
        TypeSize!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl < K , V , S > TypeSize for ExtractMap < K , V , S > where V : TypeSize , { fn extra_size (& self) -> usize { generic_vec_extra_size (self . iter () , self . capacity () , self . len ()) } # [cfg (feature = "details")] fn get_collection_item_count (& self) -> Option < usize > { Some (self . len ()) } }
    };
}

impl_23!()