macro_rules! deps {
    () => {
        SliceDst!();
        DstLayout!();
        SizeInfo!();
        PointerMetadata!();
    };
}

macro_rules! impl_432 {
    () => {
        deps!();
        impl PointerMetadata for () { # [inline] # [allow (clippy :: unused_unit)] fn from_elem_count (_elems : usize) -> () { } # [inline] fn size_for_metadata (self , layout : DstLayout) -> Option < usize > { match layout . size_info { SizeInfo :: Sized { size } => Some (size) , SizeInfo :: SliceDst (_) => None , } } }
    };
}

impl_432!();