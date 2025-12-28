macro_rules! deps {
    () => {
        Deps!();
        DepNode!();
        DepGraphData!();
    };
}

macro_rules! panic_on_forbidden_read {
    () => {
        deps!();
        # [cold] # [inline (never)] fn panic_on_forbidden_read < D : Deps > (data : & DepGraphData < D > , dep_node_index : DepNodeIndex) -> ! { let mut dep_node = None ; for prev_index in data . colors . values . indices () { if data . colors . current (prev_index) == Some (dep_node_index) { dep_node = Some (data . previous . index_to_node (prev_index)) ; break ; } } if dep_node . is_none () && let Some (nodes) = & data . current . nodes_in_current_session { # [allow (rustc :: potential_query_instability)] if let Some ((node , _)) = nodes . lock () . iter () . find (| & (_ , index) | * index == dep_node_index) { dep_node = Some (* node) ; } } let dep_node = dep_node . map_or_else (| | format ! ("with index {:?}" , dep_node_index) , | dep_node | format ! ("`{:?}`" , dep_node) ,) ; panic ! ("Error: trying to record dependency on DepNode {dep_node} in a \
         context that does not allow it (e.g. during query deserialization). \
         The most common case of recording a dependency on a DepNode `foo` is \
         when the corresponding query `foo` is invoked. Invoking queries is not \
         allowed as part of loading something from the incremental on-disk cache. \
         See <https://github.com/rust-lang/rust/pull/91919>.") }
    };
}

panic_on_forbidden_read!();