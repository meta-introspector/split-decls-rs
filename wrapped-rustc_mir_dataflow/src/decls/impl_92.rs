macro_rules! deps {
    () => {
        MaybeReachable!();
        JoinSemiLattice!();
    };
}

macro_rules! impl_92 {
    () => {
        deps!();
        impl < T : JoinSemiLattice + Clone > JoinSemiLattice for MaybeReachable < T > { fn join (& mut self , other : & Self) -> bool { match (& mut * self , & other) { (_ , MaybeReachable :: Unreachable) => false , (MaybeReachable :: Unreachable , _) => { * self = other . clone () ; true } (MaybeReachable :: Reachable (this) , MaybeReachable :: Reachable (other)) => this . join (other) , } } }
    };
}

impl_92!();