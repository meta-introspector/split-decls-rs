macro_rules! deps {
    () => {
        Dominators!();
        Node!();
        Kind!();
    };
}

macro_rules! impl_85 {
    () => {
        deps!();
        impl < Node : Idx > Dominators < Node > { # [doc = " Returns true if node is reachable from the start node."] pub fn is_reachable (& self , node : Node) -> bool { match & self . kind { Kind :: Path => true , Kind :: General (g) => g . time [node] . start != 0 , } } # [doc = " Returns the immediate dominator of node, if any."] pub fn immediate_dominator (& self , node : Node) -> Option < Node > { match & self . kind { Kind :: Path => { if 0 < node . index () { Some (Node :: new (node . index () - 1)) } else { None } } Kind :: General (g) => g . immediate_dominators [node] , } } # [doc = " Returns true if `a` dominates `b`."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if `b` is unreachable."] # [inline] pub fn dominates (& self , a : Node , b : Node) -> bool { match & self . kind { Kind :: Path => a . index () <= b . index () , Kind :: General (g) => { let a = g . time [a] ; let b = g . time [b] ; assert ! (b . start != 0 , "node {b:?} is not reachable") ; a . start <= b . start && b . finish <= a . finish } } } }
    };
}

impl_85!()