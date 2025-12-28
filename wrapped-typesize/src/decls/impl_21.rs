macro_rules! deps {
    () => {
        TypeSize!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl < K : TypeSize , V : TypeSize , S > TypeSize for DashMap < K , V , S > where K : Eq + Hash , S : Default + BuildHasher + Clone , { fn extra_size (& self) -> usize { self . shards () . iter () . map (TypeSize :: get_size) . sum :: < usize > () } # [cfg (feature = "details")] fn get_collection_item_count (& self) -> Option < usize > { Some (self . len ()) } }
    };
}

impl_21!()