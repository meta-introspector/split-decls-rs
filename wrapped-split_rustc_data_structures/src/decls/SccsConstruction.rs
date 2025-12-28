macro_rules! deps {
    () => {
        Node!();
        DirectedGraph!();
        SccData!();
        Annotations!();
        NodeState!();
        Successors!();
    };
}

macro_rules! SccsConstruction {
    () => {
        deps!();
        struct SccsConstruction < 'c , 'a , G , A > where G : DirectedGraph + Successors , A : Annotations < G :: Node > , { graph : & 'c G , # [doc = " The state of each node; used during walk to record the stack"] # [doc = " and after walk to record what cycle each node ended up being"] # [doc = " in."] node_states : IndexVec < G :: Node , NodeState < G :: Node , A :: SccIdx , A :: Ann > > , # [doc = " The stack of nodes that we are visiting as part of the DFS."] node_stack : Vec < G :: Node > , # [doc = " The stack of successors: as we visit a node, we mark our"] # [doc = " position in this stack, and when we encounter a successor SCC,"] # [doc = " we push it on the stack. When we complete an SCC, we can pop"] # [doc = " everything off the stack that was found along the way."] successors_stack : Vec < A :: SccIdx > , # [doc = " A set used to strip duplicates. As we accumulate successors"] # [doc = " into the successors_stack, we sometimes get duplicate entries."] # [doc = " We use this set to remove those -- we also keep its storage"] # [doc = " around between successors to amortize memory allocation costs."] duplicate_set : FxHashSet < A :: SccIdx > , scc_data : SccData < A :: SccIdx > , annotations : & 'a mut A , }
    };
}

SccsConstruction!()