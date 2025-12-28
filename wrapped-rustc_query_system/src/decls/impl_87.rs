macro_rules! deps {
    () => {
        Deps!();
        SerializedNodeHeader!();
        DepNode!();
        Unpacked!();
        EdgeHeader!();
        DepKind!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        impl < D : Deps > SerializedNodeHeader < D > { const TOTAL_BITS : usize = size_of :: < DepKind > () * 8 ; const LEN_BITS : usize = Self :: TOTAL_BITS - Self :: KIND_BITS - Self :: WIDTH_BITS ; const WIDTH_BITS : usize = DEP_NODE_WIDTH_BITS ; const KIND_BITS : usize = Self :: TOTAL_BITS - D :: DEP_KIND_MAX . leading_zeros () as usize ; const MAX_INLINE_LEN : usize = (u16 :: MAX as usize >> (Self :: TOTAL_BITS - Self :: LEN_BITS)) - 1 ; # [inline] fn new (node : DepNode , index : DepNodeIndex , fingerprint : Fingerprint , edge_max_index : u32 , edge_count : usize ,) -> Self { debug_assert_eq ! (Self :: TOTAL_BITS , Self :: LEN_BITS + Self :: WIDTH_BITS + Self :: KIND_BITS) ; let mut head = node . kind . as_inner () ; let free_bytes = edge_max_index . leading_zeros () as usize / 8 ; let bytes_per_index = (DEP_NODE_SIZE - free_bytes) . saturating_sub (1) ; head |= (bytes_per_index as u16) << Self :: KIND_BITS ; if edge_count <= Self :: MAX_INLINE_LEN { head |= (edge_count as u16 + 1) << (Self :: KIND_BITS + Self :: WIDTH_BITS) ; } let hash : Fingerprint = node . hash . into () ; let mut bytes = [0u8 ; 38] ; bytes [.. 2] . copy_from_slice (& head . to_le_bytes ()) ; bytes [2 .. 6] . copy_from_slice (& index . as_u32 () . to_le_bytes ()) ; bytes [6 .. 22] . copy_from_slice (& hash . to_le_bytes ()) ; bytes [22 ..] . copy_from_slice (& fingerprint . to_le_bytes ()) ; # [cfg (debug_assertions)] { let res = Self { bytes , _marker : PhantomData } ; assert_eq ! (fingerprint , res . fingerprint ()) ; assert_eq ! (node , res . node ()) ; if let Some (len) = res . len () { assert_eq ! (edge_count , len as usize) ; } } Self { bytes , _marker : PhantomData } } # [inline] fn unpack (& self) -> Unpacked { let head = u16 :: from_le_bytes (self . bytes [.. 2] . try_into () . unwrap ()) ; let index = u32 :: from_le_bytes (self . bytes [2 .. 6] . try_into () . unwrap ()) ; let hash = self . bytes [6 .. 22] . try_into () . unwrap () ; let fingerprint = self . bytes [22 ..] . try_into () . unwrap () ; let kind = head & mask (Self :: KIND_BITS) as u16 ; let bytes_per_index = (head >> Self :: KIND_BITS) & mask (Self :: WIDTH_BITS) as u16 ; let len = (head as u32) >> (Self :: WIDTH_BITS + Self :: KIND_BITS) ; Unpacked { len : len . checked_sub (1) , bytes_per_index : bytes_per_index as usize + 1 , kind : DepKind :: new (kind) , index : SerializedDepNodeIndex :: from_u32 (index) , hash : Fingerprint :: from_le_bytes (hash) . into () , fingerprint : Fingerprint :: from_le_bytes (fingerprint) , } } # [inline] fn len (& self) -> Option < u32 > { self . unpack () . len } # [inline] fn bytes_per_index (& self) -> usize { self . unpack () . bytes_per_index } # [inline] fn index (& self) -> SerializedDepNodeIndex { self . unpack () . index } # [inline] fn fingerprint (& self) -> Fingerprint { self . unpack () . fingerprint } # [inline] fn node (& self) -> DepNode { let Unpacked { kind , hash , .. } = self . unpack () ; DepNode { kind , hash } } # [inline] fn edges_header (& self , edge_list_data : & [u8] , num_edges : u32) -> EdgeHeader { EdgeHeader { repr : (edge_list_data . len () << DEP_NODE_WIDTH_BITS) | (self . bytes_per_index () - 1) , num_edges , } } }
    };
}

impl_87!()