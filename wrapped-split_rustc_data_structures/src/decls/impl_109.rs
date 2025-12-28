macro_rules! deps {
    () => {
        NodeStatus!();
        CycleDetector!();
        Node!();
        DirectedGraph!();
        TriColorVisitor!();
    };
}

macro_rules! impl_109 {
    () => {
        deps!();
        impl < G > TriColorVisitor < G > for CycleDetector where G : ? Sized + DirectedGraph , { type BreakVal = () ; fn node_examined (& mut self , _node : G :: Node , prior_status : Option < NodeStatus > ,) -> ControlFlow < Self :: BreakVal > { match prior_status { Some (NodeStatus :: Visited) => ControlFlow :: Break (()) , _ => ControlFlow :: Continue (()) , } } }
    };
}

impl_109!();