macro_rules! deps {
    () => {
        SerializedDepGraph!();
        DepNode!();
        Deps!();
        EdgeHeader!();
        SerializedNodeHeader!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        impl SerializedDepGraph { # [instrument (level = "debug" , skip (d , deps))] pub fn decode < D : Deps > (d : & mut MemDecoder < '_ > , deps : & D) -> Arc < SerializedDepGraph > { debug ! ("position: {:?}" , d . position ()) ; let (node_max , node_count , edge_count) = d . with_position (d . len () - 3 * IntEncodedWithFixedSize :: ENCODED_SIZE , | d | { debug ! ("position: {:?}" , d . position ()) ; let node_max = IntEncodedWithFixedSize :: decode (d) . 0 as usize ; let node_count = IntEncodedWithFixedSize :: decode (d) . 0 as usize ; let edge_count = IntEncodedWithFixedSize :: decode (d) . 0 as usize ; (node_max , node_count , edge_count) }) ; debug ! ("position: {:?}" , d . position ()) ; debug ! (? node_count , ? edge_count) ; let graph_bytes = d . len () - (3 * IntEncodedWithFixedSize :: ENCODED_SIZE) - d . position () ; let mut nodes = IndexVec :: from_elem_n (DepNode { kind : D :: DEP_KIND_NULL , hash : PackedFingerprint :: from (Fingerprint :: ZERO) } , node_max ,) ; let mut fingerprints = IndexVec :: from_elem_n (Fingerprint :: ZERO , node_max) ; let mut edge_list_indices = IndexVec :: from_elem_n (EdgeHeader { repr : 0 , num_edges : 0 } , node_max) ; let mut edge_list_data = Vec :: with_capacity (graph_bytes - node_count * size_of :: < SerializedNodeHeader < D > > ()) ; for _ in 0 .. node_count { let node_header = SerializedNodeHeader :: < D > { bytes : d . read_array () , _marker : PhantomData } ; let index = node_header . index () ; let node = & mut nodes [index] ; assert ! (node_header . node () . kind != D :: DEP_KIND_NULL && node . kind == D :: DEP_KIND_NULL) ; * node = node_header . node () ; fingerprints [index] = node_header . fingerprint () ; let num_edges = node_header . len () . unwrap_or_else (| | d . read_u32 ()) ; let edges_len_bytes = node_header . bytes_per_index () * (num_edges as usize) ; let edges_header = node_header . edges_header (& edge_list_data , num_edges) ; edge_list_data . extend (d . read_raw_bytes (edges_len_bytes)) ; edge_list_indices [index] = edges_header ; } edge_list_data . extend (& [0u8 ; DEP_NODE_PAD]) ; let mut index : Vec < _ > = (0 .. (D :: DEP_KIND_MAX + 1)) . map (| _ | UnhashMap :: with_capacity_and_hasher (d . read_u32 () as usize , Default :: default ())) . collect () ; let session_count = d . read_u64 () ; for (idx , node) in nodes . iter_enumerated () { if index [node . kind . as_usize ()] . insert (node . hash , idx) . is_some () { if node . kind != D :: DEP_KIND_NULL && node . kind != D :: DEP_KIND_SIDE_EFFECT { let name = deps . name (node . kind) ; panic ! ("Error: A dep graph node ({name}) does not have an unique index. \
                     Running a clean build on a nightly compiler with `-Z incremental-verify-ich` \
                     can help narrow down the issue for reporting. A clean build may also work around the issue.\n
                     DepNode: {node:?}") } } } Arc :: new (SerializedDepGraph { nodes , fingerprints , edge_list_indices , edge_list_data , index , session_count , }) } }
    };
}

impl_84!()