macro_rules! deps {
    () => {
        JoinSemiLattice!();
    };
}

macro_rules! impl_81 {
    () => {
        deps!();
        # [doc = " A `DenseBitSet` represents the lattice formed by the powerset of all possible values of the"] # [doc = " index type `T` ordered by inclusion. Equivalently, it is a tuple of \"two-point\" lattices, one"] # [doc = " for each possible value of `T`."] impl < T : Idx > JoinSemiLattice for DenseBitSet < T > { fn join (& mut self , other : & Self) -> bool { self . union (other) } }
    };
}

impl_81!()