macro_rules! deps {
    () => {
        Successors!();
        DirectedGraph!();
        Predecessors!();
        ControlFlowGraph!();
        StartNode!();
    };
}

macro_rules! impl_229 {
    () => {
        deps!();
        impl < T > ControlFlowGraph for T where T : DirectedGraph + StartNode + Predecessors + Successors { }
    };
}

impl_229!();