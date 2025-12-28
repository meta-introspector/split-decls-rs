macro_rules! deps {
    () => {
        StartNode!();
        Successors!();
        TriColorVisitor!();
        DirectedGraph!();
        TriColorDepthFirstSearch!();
    };
}

macro_rules! impl_106 {
    () => {
        deps!();
        impl < G > TriColorDepthFirstSearch < '_ , G > where G : ? Sized + DirectedGraph + Successors + StartNode , { # [doc = " Performs a depth-first search, starting from `G::start_node()`."] # [doc = ""] # [doc = " This won't visit nodes that are not reachable from the start node."] pub fn run_from_start < V > (self , visitor : & mut V) -> Option < V :: BreakVal > where V : TriColorVisitor < G > , { let root = self . graph . start_node () ; self . run_from (root , visitor) } }
    };
}

impl_106!();