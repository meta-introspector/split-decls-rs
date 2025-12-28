macro_rules! deps {
    () => {
        GraphEncoder!();
        DepNodeColor!();
        CurrentDepGraph!();
        NodeInfo!();
        EdgesVec!();
        DepNode!();
        DepNodeColorMap!();
        Deps!();
        SerializedDepGraph!();
        DepGraphQuery!();
        EncoderState!();
    };
}

macro_rules! impl_96 {
    () => {
        deps!();
        impl < D : Deps > GraphEncoder < D > { pub (crate) fn new (sess : & Session , encoder : FileEncoder , prev_node_count : usize , previous : Arc < SerializedDepGraph > ,) -> Self { let record_graph = sess . opts . unstable_opts . query_dep_graph . then (| | Lock :: new (DepGraphQuery :: new (prev_node_count))) ; let status = EncoderState :: new (encoder , sess . opts . unstable_opts . incremental_info , previous) ; GraphEncoder { status , record_graph , profiler : sess . prof . clone () } } pub (crate) fn with_query (& self , f : impl Fn (& DepGraphQuery)) { if let Some (record_graph) = & self . record_graph { f (& record_graph . lock ()) } } # [doc = " Encodes a node that does not exists in the previous graph."] pub (crate) fn send_new (& self , node : DepNode , fingerprint : Fingerprint , edges : EdgesVec ,) -> DepNodeIndex { let _prof_timer = self . profiler . generic_activity ("incr_comp_encode_dep_graph") ; let node = NodeInfo { node , fingerprint , edges } ; let mut local = self . status . local . borrow_mut () ; let index = self . status . next_index (& mut * local) ; self . status . bump_index (& mut * local) ; self . status . encode_node (index , & node , & self . record_graph , & mut * local) ; index } # [doc = " Encodes a node that exists in the previous graph, but was re-executed."] # [doc = ""] # [doc = " This will also ensure the dep node is colored either red or green."] pub (crate) fn send_and_color (& self , prev_index : SerializedDepNodeIndex , colors : & DepNodeColorMap , node : DepNode , fingerprint : Fingerprint , edges : EdgesVec , is_green : bool ,) -> DepNodeIndex { let _prof_timer = self . profiler . generic_activity ("incr_comp_encode_dep_graph") ; let node = NodeInfo { node , fingerprint , edges } ; let mut local = self . status . local . borrow_mut () ; let index = self . status . next_index (& mut * local) ; if is_green { match colors . try_mark_green (prev_index , index) { Ok (()) => () , Err (dep_node_index) => return dep_node_index , } } else { colors . insert (prev_index , DepNodeColor :: Red) ; } self . status . bump_index (& mut * local) ; self . status . encode_node (index , & node , & self . record_graph , & mut * local) ; index } # [doc = " Encodes a node that was promoted from the previous graph. It reads the information directly from"] # [doc = " the previous dep graph and expects all edges to already have a new dep node index assigned."] # [doc = ""] # [doc = " This will also ensure the dep node is marked green."] # [inline] pub (crate) fn send_promoted (& self , prev_index : SerializedDepNodeIndex , colors : & DepNodeColorMap ,) -> DepNodeIndex { let _prof_timer = self . profiler . generic_activity ("incr_comp_encode_dep_graph") ; let mut local = self . status . local . borrow_mut () ; let index = self . status . next_index (& mut * local) ; match colors . try_mark_green (prev_index , index) { Ok (()) => { self . status . bump_index (& mut * local) ; self . status . encode_promoted_node (index , prev_index , & self . record_graph , colors , & mut * local ,) ; index } Err (dep_node_index) => dep_node_index , } } pub (crate) fn finish (& self , current : & CurrentDepGraph < D >) -> FileEncodeResult { let _prof_timer = self . profiler . generic_activity ("incr_comp_encode_dep_graph_finish") ; self . status . finish (& self . profiler , current) } }
    };
}

impl_96!()