macro_rules! deps {
    () => {
        EdgeHeader!();
        DepNode!();
    };
}

macro_rules! SerializedDepGraph {
    () => {
        deps!();
        # [doc = " Data for use when recompiling the **current crate**."] # [doc = ""] # [doc = " There may be unused indices with DEP_KIND_NULL in this graph due to batch allocation of"] # [doc = " indices to threads."] # [derive (Debug , Default)] pub struct SerializedDepGraph { # [doc = " The set of all DepNodes in the graph"] nodes : IndexVec < SerializedDepNodeIndex , DepNode > , # [doc = " The set of all Fingerprints in the graph. Each Fingerprint corresponds to"] # [doc = " the DepNode at the same index in the nodes vector."] fingerprints : IndexVec < SerializedDepNodeIndex , Fingerprint > , # [doc = " For each DepNode, stores the list of edges originating from that"] # [doc = " DepNode. Encoded as a [start, end) pair indexing into edge_list_data,"] # [doc = " which holds the actual DepNodeIndices of the target nodes."] edge_list_indices : IndexVec < SerializedDepNodeIndex , EdgeHeader > , # [doc = " A flattened list of all edge targets in the graph, stored in the same"] # [doc = " varint encoding that we use on disk. Edge sources are implicit in edge_list_indices."] edge_list_data : Vec < u8 > , # [doc = " Stores a map from fingerprints to nodes per dep node kind."] # [doc = " This is the reciprocal of `nodes`."] index : Vec < UnhashMap < PackedFingerprint , SerializedDepNodeIndex > > , # [doc = " The number of previous compilation sessions. This is used to generate"] # [doc = " unique anon dep nodes per session."] session_count : u64 , }
    };
}

SerializedDepGraph!()