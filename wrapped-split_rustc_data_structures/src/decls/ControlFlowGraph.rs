macro_rules! deps {
    () => {
        Predecessors!();
        DirectedGraph!();
        StartNode!();
        Successors!();
    };
}

macro_rules! ControlFlowGraph {
    () => {
        deps!();
        # [doc = " Alias for [`DirectedGraph`] + [`StartNode`] + [`Predecessors`] + [`Successors`]."] pub trait ControlFlowGraph : DirectedGraph + StartNode + Predecessors + Successors { }
    };
}

ControlFlowGraph!()