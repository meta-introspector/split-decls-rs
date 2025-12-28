macro_rules! deps {
    () => {
        DepNode!();
        SerializedDepGraph!();
    };
}

macro_rules! impl_80 {
    () => {
        deps!();
        impl SerializedDepGraph { # [inline] pub fn edge_targets_from (& self , source : SerializedDepNodeIndex ,) -> impl Iterator < Item = SerializedDepNodeIndex > + Clone { let header = self . edge_list_indices [source] ; let mut raw = & self . edge_list_data [header . start () ..] ; let bytes_per_index = header . bytes_per_index () ; let mask = header . mask () ; (0 .. header . num_edges) . map (move | _ | { let index = & raw [.. DEP_NODE_SIZE] ; raw = & raw [bytes_per_index ..] ; let index = u32 :: from_le_bytes (index . try_into () . unwrap ()) & mask ; SerializedDepNodeIndex :: from_u32 (index) }) } # [inline] pub fn index_to_node (& self , dep_node_index : SerializedDepNodeIndex) -> DepNode { self . nodes [dep_node_index] } # [inline] pub fn node_to_index_opt (& self , dep_node : & DepNode) -> Option < SerializedDepNodeIndex > { self . index . get (dep_node . kind . as_usize ()) ? . get (& dep_node . hash) . cloned () } # [inline] pub fn fingerprint_by_index (& self , dep_node_index : SerializedDepNodeIndex) -> Fingerprint { self . fingerprints [dep_node_index] } # [inline] pub fn node_count (& self) -> usize { self . nodes . len () } # [inline] pub fn session_count (& self) -> u64 { self . session_count } }
    };
}

impl_80!();