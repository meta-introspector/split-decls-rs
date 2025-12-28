macro_rules! JoinSemiLattice {
    () => {
        # [doc = " A [partially ordered set][poset] that has a [least upper bound][lub] for any pair of elements"] # [doc = " in the set."] # [doc = ""] # [doc = " [lub]: https://en.wikipedia.org/wiki/Infimum_and_supremum"] # [doc = " [poset]: https://en.wikipedia.org/wiki/Partially_ordered_set"] pub trait JoinSemiLattice : Eq { # [doc = " Computes the least upper bound of two elements, storing the result in `self` and returning"] # [doc = " `true` if `self` has changed."] # [doc = ""] # [doc = " The lattice join operator is abbreviated as `∨`."] fn join (& mut self , other : & Self) -> bool ; }
    };
}

JoinSemiLattice!()