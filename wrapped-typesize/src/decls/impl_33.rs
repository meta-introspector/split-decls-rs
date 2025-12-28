macro_rules! deps {
    () => {
        TypeSize!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl < T : TypeSize > TypeSize for HashTable < T > { fn extra_size (& self) -> usize { self . allocation_size () + self . iter () . map (T :: extra_size) . sum :: < usize > () } # [cfg (feature = "details")] fn get_collection_item_count (& self) -> Option < usize > { Some (self . len ()) } }
    };
}

impl_33!()