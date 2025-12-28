macro_rules! deps {
    () => {
        Edge!();
        ObligationForest!();
        Node!();
        ForestObligation!();
    };
}

macro_rules! impl_309 {
    () => {
        deps!();
        impl < 'a , O : ForestObligation + 'a > dot :: GraphWalk < 'a > for & 'a ObligationForest < O > { type Node = usize ; type Edge = (usize , usize) ; fn nodes (& self) -> dot :: Nodes < '_ , Self :: Node > { (0 .. self . nodes . len ()) . collect () } fn edges (& self) -> dot :: Edges < '_ , Self :: Edge > { (0 .. self . nodes . len ()) . flat_map (| i | { let node = & self . nodes [i] ; node . dependents . iter () . map (move | & d | (d , i)) }) . collect () } fn source (& self , (s , _) : & Self :: Edge) -> Self :: Node { * s } fn target (& self , (_ , t) : & Self :: Edge) -> Self :: Node { * t } }
    };
}

impl_309!()