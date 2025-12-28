macro_rules! deps {
    () => {
        TypeSize!();
        Field!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl < T : TypeSize > TypeSize for parking_lot :: RwLock < T > { fn extra_size (& self) -> usize { self . read () . extra_size () } if_typesize_details ! { fn get_collection_item_count (& self) -> Option < usize > { self . read () . get_collection_item_count () } fn get_size_details (& self) -> alloc :: vec :: Vec < crate :: Field > { self . read () . get_size_details () } } }
    };
}

impl_42!();