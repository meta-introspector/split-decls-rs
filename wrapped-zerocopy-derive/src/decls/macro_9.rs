macro_rules! macro_9 {
    () => {
        use_as_trait_name ! (KnownLayout => derive_known_layout_inner , Immutable => derive_no_cell_inner , TryFromBytes => derive_try_from_bytes_inner , FromZeros => derive_from_zeros_inner , FromBytes => derive_from_bytes_inner , IntoBytes => derive_into_bytes_inner , Unaligned => derive_unaligned_inner , ByteHash => derive_hash_inner , ByteEq => derive_eq_inner , SplitAt => derive_split_at_inner ,) ;
    };
}

macro_9!()