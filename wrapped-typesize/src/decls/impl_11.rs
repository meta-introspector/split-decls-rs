macro_rules! deps {
    () => {
        TypeSize!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl < const CAP : usize > TypeSize for ArrayString < CAP > { # [cfg (feature = "details")] fn get_collection_item_count (& self) -> Option < usize > { Some (self . len ()) } }
    };
}

impl_11!()