macro_rules! deps {
    () => {
        TypeSize!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl < K : Eq + Hash + TypeSize , V : TypeSize , S : BuildHasher > TypeSize for HashMap < K , V , S > { fn extra_size (& self) -> usize { let base_extra_size = self . allocation_size () ; let extra_extra_size = self . iter () . map (| (k , v) | K :: extra_size (k) + V :: extra_size (v)) . sum :: < usize > () ; base_extra_size + extra_extra_size } # [cfg (feature = "details")] fn get_collection_item_count (& self) -> Option < usize > { Some (self . len ()) } }
    };
}

impl_31!();