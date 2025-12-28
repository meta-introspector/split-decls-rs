macro_rules! deps {
    () => {
        TypeSize!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl TypeSize for serde_json :: Map < String , serde_json :: Value > { fn extra_size (& self) -> usize { crate :: map :: generic_map_extra_size (self . iter () , self . len () , self . len ()) } # [cfg (feature = "details")] fn get_collection_item_count (& self) -> Option < usize > { Some (self . len ()) } }
    };
}

impl_47!()