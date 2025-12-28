macro_rules! deps {
    () => {
        TypeSize!();
    };
}

macro_rules! impl_115 {
    () => {
        deps!();
        impl TypeSize for String { fn extra_size (& self) -> usize { core :: mem :: size_of :: < u8 > () * self . capacity () } # [cfg (feature = "details")] fn get_collection_item_count (& self) -> Option < usize > { Some (self . len ()) } }
    };
}

impl_115!();