macro_rules! pack_hash_index_and_kind {
    () => {
        # [inline] const fn pack_hash_index_and_kind (hash : u16 , index : u32 , kind : u32) -> u32 { (hash as u32) | (index << HASH_BITS) | (kind << (HASH_BITS + INDEX_BITS)) }
    };
}

pack_hash_index_and_kind!()