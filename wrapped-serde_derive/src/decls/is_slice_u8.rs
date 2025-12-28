macro_rules! is_slice_u8 {
    () => {
        fn is_slice_u8 (ty : & syn :: Type) -> bool { match ungroup (ty) { syn :: Type :: Slice (ty) => is_primitive_type (& ty . elem , "u8") , _ => false , } }
    };
}

is_slice_u8!();