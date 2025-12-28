macro_rules! deps {
    () => {
        TypeSize!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl < K : TypeSize , V : TypeSize , S > TypeSize for std :: collections :: HashMap < K , V , S > { fn extra_size (& self) -> usize { generic_map_extra_size :: < K , V > (self . iter () , self . capacity () , self . len ()) } # [cfg (feature = "details")] fn get_collection_item_count (& self) -> Option < usize > { Some (self . len ()) } }
    };
}

impl_65!();