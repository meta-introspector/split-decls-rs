macro_rules! deps {
    () => {
        WorkProductMap!();
        Deps!();
        DepNode!();
        DepNodeColorMap!();
        SerializedDepGraph!();
        CurrentDepGraph!();
    };
}

macro_rules! DepGraphData {
    () => {
        deps!();
        pub (crate) struct DepGraphData < D : Deps > { # [doc = " The new encoding of the dependency graph, optimized for red/green"] # [doc = " tracking. The `current` field is the dependency graph of only the"] # [doc = " current compilation session: We don't merge the previous dep-graph into"] # [doc = " current one anymore, but we do reference shared data to save space."] current : CurrentDepGraph < D > , # [doc = " The dep-graph from the previous compilation session. It contains all"] # [doc = " nodes and edges as well as all fingerprints of nodes that have them."] previous : Arc < SerializedDepGraph > , colors : DepNodeColorMap , # [doc = " When we load, there may be `.o` files, cached MIR, or other such"] # [doc = " things available to us. If we find that they are not dirty, we"] # [doc = " load the path to the file storing those work-products here into"] # [doc = " this map. We can later look for and extract that data."] previous_work_products : WorkProductMap , dep_node_debug : Lock < FxHashMap < DepNode , String > > , # [doc = " Used by incremental compilation tests to assert that"] # [doc = " a particular query result was decoded from disk"] # [doc = " (not just marked green)"] debug_loaded_from_disk : Lock < FxHashSet < DepNode > > , }
    };
}

DepGraphData!();