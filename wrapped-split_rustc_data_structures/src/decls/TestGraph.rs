macro_rules! TestGraph {
    () => {
        pub (super) struct TestGraph { num_nodes : usize , start_node : usize , successors : FxHashMap < usize , Vec < usize > > , predecessors : FxHashMap < usize , Vec < usize > > , }
    };
}

TestGraph!()