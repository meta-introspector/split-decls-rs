macro_rules! deps {
    () => {
        DepNode!();
        DepGraphQuery!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl DepGraphQuery { pub fn new (prev_node_count : usize) -> DepGraphQuery { let node_count = prev_node_count + prev_node_count / 4 ; let edge_count = 6 * node_count ; let graph = LinkedGraph :: with_capacity (node_count , edge_count) ; let indices = FxHashMap :: default () ; let dep_index_to_index = IndexVec :: new () ; DepGraphQuery { graph , indices , dep_index_to_index } } pub fn push (& mut self , index : DepNodeIndex , node : DepNode , edges : & [DepNodeIndex]) { let source = self . graph . add_node (node) ; self . dep_index_to_index . insert (index , source) ; self . indices . insert (node , source) ; for & target in edges . iter () { let target = self . dep_index_to_index [target] ; if let Some (target) = target { self . graph . add_edge (source , target , ()) ; } } } pub fn nodes (& self) -> Vec < & DepNode > { self . graph . all_nodes () . iter () . map (| n | & n . data) . collect () } pub fn edges (& self) -> Vec < (& DepNode , & DepNode) > { self . graph . all_edges () . iter () . map (| edge | (edge . source () , edge . target ())) . map (| (s , t) | (self . graph . node_data (s) , self . graph . node_data (t))) . collect () } fn reachable_nodes (& self , node : & DepNode , direction : Direction) -> Vec < & DepNode > { if let Some (& index) = self . indices . get (node) { self . graph . depth_traverse (index , direction) . map (| s | self . graph . node_data (s)) . collect () } else { vec ! [] } } # [doc = " All nodes that can reach `node`."] pub fn transitive_predecessors (& self , node : & DepNode) -> Vec < & DepNode > { self . reachable_nodes (node , INCOMING) } }
    };
}

impl_73!();