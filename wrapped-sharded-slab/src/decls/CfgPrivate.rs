macro_rules! deps {
    () => {
        Config!();
        Generation!();
        RefCount!();
        Pack!();
        Addr!();
        DebugConfig!();
        Tid!();
    };
}

macro_rules! CfgPrivate {
    () => {
        deps!();
        pub (crate) trait CfgPrivate : Config { const USED_BITS : usize = Generation :: < Self > :: LEN + Generation :: < Self > :: SHIFT ; const INITIAL_SZ : usize = next_pow2 (Self :: INITIAL_PAGE_SIZE) ; const MAX_SHARDS : usize = next_pow2 (Self :: MAX_THREADS - 1) ; const ADDR_INDEX_SHIFT : usize = Self :: INITIAL_SZ . trailing_zeros () as usize + 1 ; fn page_size (n : usize) -> usize { Self :: INITIAL_SZ * 2usize . pow (n as _) } fn debug () -> DebugConfig < Self > { DebugConfig { _cfg : PhantomData } } fn validate () { assert ! (Self :: INITIAL_SZ . is_power_of_two () , "invalid Config: {:#?}" , Self :: debug () ,) ; assert ! (Self :: INITIAL_SZ <= Addr ::< Self >:: BITS , "invalid Config: {:#?}" , Self :: debug ()) ; assert ! (Generation ::< Self >:: BITS >= 3 , "invalid Config: {:#?}\ngeneration counter should be at least 3 bits!" , Self :: debug ()) ; assert ! (Self :: USED_BITS <= WIDTH , "invalid Config: {:#?}\ntotal number of bits per index is too large to fit in a word!" , Self :: debug ()) ; assert ! (WIDTH - Self :: USED_BITS >= Self :: RESERVED_BITS , "invalid Config: {:#?}\nindices are too large to fit reserved bits!" , Self :: debug ()) ; assert ! (RefCount ::< Self >:: MAX > 1 , "invalid config: {:#?}\n maximum concurrent references would be {}" , Self :: debug () , RefCount ::< Self >:: MAX ,) ; } # [inline (always)] fn unpack < A : Pack < Self > > (packed : usize) -> A { A :: from_packed (packed) } # [inline (always)] fn unpack_addr (packed : usize) -> Addr < Self > { Self :: unpack (packed) } # [inline (always)] fn unpack_tid (packed : usize) -> crate :: Tid < Self > { Self :: unpack (packed) } # [inline (always)] fn unpack_gen (packed : usize) -> Generation < Self > { Self :: unpack (packed) } }
    };
}

CfgPrivate!()