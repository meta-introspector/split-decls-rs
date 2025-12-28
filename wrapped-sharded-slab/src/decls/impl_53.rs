macro_rules! deps {
    () => {
        RefCount!();
        DebugConfig!();
        Config!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl < C : Config > fmt :: Debug for DebugConfig < C > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct (std :: any :: type_name :: < C > ()) . field ("initial_page_size" , & C :: INITIAL_SZ) . field ("max_shards" , & C :: MAX_SHARDS) . field ("max_pages" , & C :: MAX_PAGES) . field ("used_bits" , & C :: USED_BITS) . field ("reserved_bits" , & C :: RESERVED_BITS) . field ("pointer_width" , & WIDTH) . field ("max_concurrent_references" , & RefCount :: < C > :: MAX) . finish () } }
    };
}

impl_53!();