macro_rules! deps {
    () => {
        Field!();
        Owned!();
        TypeSize!();
        SizableArc!();
    };
}

macro_rules! impl_78 {
    () => {
        deps!();
        impl < T : TypeSize > TypeSize for SizableArc < T , Owned > { fn extra_size (& self) -> usize { T :: get_size (& self . 0) + (core :: mem :: size_of :: < AtomicUsize > () * 2) } if_typesize_details ! { fn get_collection_item_count (& self) -> Option < usize > { T :: get_collection_item_count (& self . 0) } fn get_size_details (& self) -> alloc :: vec :: Vec < crate :: Field > { T :: get_size_details (& self . 0) } } }
    };
}

impl_78!();