macro_rules! deps {
    () => {
        DepthFirstSearch!();
        Node!();
        Successors!();
        DirectedGraph!();
    };
}

macro_rules! impl_99 {
    () => {
        deps!();
        impl < G > DepthFirstSearch < G > where G : DirectedGraph + Successors , { pub fn new (graph : G) -> Self { Self { stack : vec ! [] , visited : DenseBitSet :: new_empty (graph . num_nodes ()) , graph } } # [doc = " Version of `push_start_node` that is convenient for chained"] # [doc = " use."] pub fn with_start_node (mut self , start_node : G :: Node) -> Self { self . push_start_node (start_node) ; self } # [doc = " Pushes another start node onto the stack. If the node"] # [doc = " has not already been visited, then you will be able to"] # [doc = " walk its successors (and so forth) after the current"] # [doc = " contents of the stack are drained. If multiple start nodes"] # [doc = " are added into the walk, then their mutual successors"] # [doc = " will all be walked. You can use this method once the"] # [doc = " iterator has been completely drained to add additional"] # [doc = " start nodes."] pub fn push_start_node (& mut self , start_node : G :: Node) { if self . visited . insert (start_node) { self . stack . push (start_node) ; } } # [doc = " Searches all nodes reachable from the current start nodes."] # [doc = " This is equivalent to just invoke `next` repeatedly until"] # [doc = " you get a `None` result."] pub fn complete_search (& mut self) { for _ in self . by_ref () { } } # [doc = " Returns true if node has been visited thus far."] # [doc = " A node is considered \"visited\" once it is pushed"] # [doc = " onto the internal stack; it may not yet have been yielded"] # [doc = " from the iterator. This method is best used after"] # [doc = " the iterator is completely drained."] pub fn visited (& self , node : G :: Node) -> bool { self . visited . contains (node) } # [doc = " Returns a reference to the set of nodes that have been visited, with"] # [doc = " the same caveats as [`Self::visited`]."] # [doc = ""] # [doc = " When incorporating the visited nodes into another bitset, using bulk"] # [doc = " operations like `union` or `intersect` can be more efficient than"] # [doc = " processing each node individually."] pub fn visited_set (& self) -> & DenseBitSet < G :: Node > { & self . visited } }
    };
}

impl_99!();