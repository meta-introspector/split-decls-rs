macro_rules! deps {
    () => {
        DirectedGraph!();
        Node!();
        Successors!();
    };
}

macro_rules! DepthFirstSearch {
    () => {
        deps!();
        # [doc = " A \"depth-first search\" iterator for a directed graph."] pub struct DepthFirstSearch < G > where G : DirectedGraph + Successors , { graph : G , stack : Vec < G :: Node > , visited : DenseBitSet < G :: Node > , }
    };
}

DepthFirstSearch!();