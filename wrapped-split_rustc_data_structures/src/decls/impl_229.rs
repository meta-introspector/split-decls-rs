macro_rules! deps {
    () => {
        Predecessors!();
        StartNode!();
        DirectedGraph!();
        ControlFlowGraph!();
        Successors!();
    };
}

macro_rules! impl_229 {
    () => {
        deps!();
        impl < T > ControlFlowGraph for T where T : DirectedGraph + StartNode + Predecessors + Successors { }
    };
}

impl_229!()