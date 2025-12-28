macro_rules! deps {
    () => {
        TypeSize!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl < A : BitViewSized + TypeSize , O : BitOrder > TypeSize for BitArray < A , O > { fn extra_size (& self) -> usize { self . data . extra_size () } # [cfg (feature = "details")] fn get_collection_item_count (& self) -> Option < usize > { Some (self . len ()) } }
    };
}

impl_13!();