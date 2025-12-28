macro_rules! deps {
    () => {
        TypeSize!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl < T : BitStore , O : BitOrder > TypeSize for BitVec < T , O > { fn extra_size (& self) -> usize { div_ceil (self . capacity () , bitvec :: mem :: bits_of :: < T > ()) } # [cfg (feature = "details")] fn get_collection_item_count (& self) -> Option < usize > { Some (self . len ()) } }
    };
}

impl_14!();