macro_rules! deps {
    () => {
        JoinSemiLattice!();
    };
}

macro_rules! impl_82 {
    () => {
        deps!();
        impl < T : Idx > JoinSemiLattice for MixedBitSet < T > { fn join (& mut self , other : & Self) -> bool { self . union (other) } }
    };
}

impl_82!();