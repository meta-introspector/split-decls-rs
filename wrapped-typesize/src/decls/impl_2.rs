macro_rules! deps {
    () => {
        Field!();
        TypeSize!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl < T : TypeSize + Copy > TypeSize for core :: cell :: Cell < T > { fn extra_size (& self) -> usize { self . get () . extra_size () } if_typesize_details ! { fn get_collection_item_count (& self) -> Option < usize > { self . get () . get_collection_item_count () } fn get_size_details (& self) -> alloc :: vec :: Vec < crate :: Field > { self . get () . get_size_details () } } }
    };
}

impl_2!()