macro_rules! deps {
    () => {
        TypeSize!();
        Field!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl < T : TypeSize > TypeSize for core :: cell :: RefCell < T > { fn extra_size (& self) -> usize { self . borrow () . extra_size () } if_typesize_details ! { fn get_collection_item_count (& self) -> Option < usize > { self . borrow () . get_collection_item_count () } fn get_size_details (& self) -> alloc :: vec :: Vec < crate :: Field > { self . borrow () . get_size_details () } } }
    };
}

impl_3!();