macro_rules! deps {
    () => {
        StartNode!();
        Successors!();
        Predecessors!();
        DirectedGraph!();
    };
}

macro_rules! ControlFlowGraph {
    () => {
        deps!();
        # [doc = " Alias for [`DirectedGraph`] + [`StartNode`] + [`Predecessors`] + [`Successors`]."] pub trait ControlFlowGraph : DirectedGraph + StartNode + Predecessors + Successors { }
    };
}

ControlFlowGraph!();