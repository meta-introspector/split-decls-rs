macro_rules! deps {
    () => {
        VecGraph!();
    };
}

macro_rules! create_index {
    () => {
        deps!();
        # [doc = " Creates/initializes the index for the [`VecGraph`]. A helper for [`VecGraph::new`]."] # [doc = ""] # [doc = " - `num_nodes` is the target number of nodes in the graph"] # [doc = " - `sorted_edge_sources` are the edge sources, sorted"] # [doc = " - `associated_edge_targets` are the edge *targets* in the same order as sources"] # [doc = " - `edge_targets` is the vec of targets to be extended"] # [doc = " - `node_starts` is the index to be filled"] fn create_index < N : Idx + Ord > (num_nodes : usize , sorted_edge_sources : & mut dyn Iterator < Item = N > , associated_edge_targets : & mut dyn Iterator < Item = N > , edge_targets : & mut Vec < N > , node_starts : & mut IndexVec < N , usize > ,) { let offset = edge_targets . len () ; edge_targets . extend (associated_edge_targets) ; for (index , source) in sorted_edge_sources . enumerate () { while node_starts . len () <= source . index () { node_starts . push (index + offset) ; } } while node_starts . len () <= num_nodes { node_starts . push (edge_targets . len ()) ; } assert_eq ! (node_starts . len () , num_nodes + 1) ; }
    };
}

create_index!()