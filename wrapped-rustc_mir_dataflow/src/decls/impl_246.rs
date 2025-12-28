macro_rules! deps {
    () => {
        StateData!();
        JoinSemiLattice!();
    };
}

macro_rules! impl_246 {
    () => {
        deps!();
        impl < V : JoinSemiLattice + Clone > JoinSemiLattice for StateData < V > { fn join (& mut self , other : & Self) -> bool { let mut changed = false ; # [allow (rustc :: potential_query_instability)] for (i , v) in other . map . iter () { match self . map . entry (* i) { StdEntry :: Vacant (e) => { e . insert (v . clone ()) ; changed = true } StdEntry :: Occupied (e) => changed |= e . into_mut () . join (v) , } } changed } }
    };
}

impl_246!();