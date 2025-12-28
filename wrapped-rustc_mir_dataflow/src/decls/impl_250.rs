macro_rules! deps {
    () => {
        State!();
        JoinSemiLattice!();
    };
}

macro_rules! impl_250 {
    () => {
        deps!();
        impl < V : JoinSemiLattice + Clone > JoinSemiLattice for State < V > { fn join (& mut self , other : & Self) -> bool { match (& mut * self , other) { (_ , State :: Unreachable) => false , (State :: Unreachable , _) => { * self = other . clone () ; true } (State :: Reachable (this) , State :: Reachable (other)) => this . join (other) , } } }
    };
}

impl_250!();