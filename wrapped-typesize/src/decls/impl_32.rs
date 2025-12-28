macro_rules! deps {
    () => {
        TypeSize!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl < T : Eq + Hash + TypeSize , S : BuildHasher > TypeSize for HashSet < T , S > { fn extra_size (& self) -> usize { self . allocation_size () + self . iter () . map (T :: extra_size) . sum :: < usize > () } # [cfg (feature = "details")] fn get_collection_item_count (& self) -> Option < usize > { Some (self . len ()) } }
    };
}

impl_32!()