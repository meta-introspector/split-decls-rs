macro_rules! deps {
    () => {
        TypeSize!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl < const N : usize , T : TypeSize > TypeSize for [T ; N] { fn extra_size (& self) -> usize { self . iter () . map (T :: extra_size) . sum () } # [cfg (feature = "details")] fn get_collection_item_count (& self) -> Option < usize > { Some (N) } }
    };
}

impl_67!()