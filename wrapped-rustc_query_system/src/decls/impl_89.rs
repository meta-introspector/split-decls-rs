macro_rules! deps {
    () => {
        DepNode!();
        SerializedNodeHeader!();
        EdgesVec!();
        SerializedDepGraph!();
        DepNodeColorMap!();
        Deps!();
        NodeInfo!();
    };
}

macro_rules! impl_89 {
    () => {
        deps!();
        impl NodeInfo { fn encode < D : Deps > (& self , e : & mut MemEncoder , index : DepNodeIndex) { let NodeInfo { node , fingerprint , ref edges } = * self ; let header = SerializedNodeHeader :: < D > :: new (node , index , fingerprint , edges . max_index () , edges . len () ,) ; e . write_array (header . bytes) ; if header . len () . is_none () { e . emit_u32 (edges . len () . try_into () . unwrap ()) ; } let bytes_per_index = header . bytes_per_index () ; for node_index in edges . iter () { e . write_with (| dest | { * dest = node_index . as_u32 () . to_le_bytes () ; bytes_per_index }) ; } } # [doc = " Encode a node that was promoted from the previous graph. It reads the edges directly from"] # [doc = " the previous dep graph and expects all edges to already have a new dep node index assigned."] # [doc = " This avoids the overhead of constructing `EdgesVec`, which would be needed to call `encode`."] # [inline] fn encode_promoted < D : Deps > (e : & mut MemEncoder , node : DepNode , index : DepNodeIndex , fingerprint : Fingerprint , prev_index : SerializedDepNodeIndex , colors : & DepNodeColorMap , previous : & SerializedDepGraph ,) -> usize { let edges = previous . edge_targets_from (prev_index) ; let edge_count = edges . size_hint () . 0 ; let edge_max = edges . clone () . map (| i | colors . current (i) . unwrap () . as_u32 ()) . max () . unwrap_or (0) ; let header = SerializedNodeHeader :: < D > :: new (node , index , fingerprint , edge_max , edge_count) ; e . write_array (header . bytes) ; if header . len () . is_none () { e . emit_u32 (edge_count . try_into () . unwrap ()) ; } let bytes_per_index = header . bytes_per_index () ; for node_index in edges { let node_index = colors . current (node_index) . unwrap () ; e . write_with (| dest | { * dest = node_index . as_u32 () . to_le_bytes () ; bytes_per_index }) ; } edge_count } }
    };
}

impl_89!();