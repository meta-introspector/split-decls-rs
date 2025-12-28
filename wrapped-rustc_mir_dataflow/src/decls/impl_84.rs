macro_rules! deps {
    () => {
        JoinSemiLattice!();
        FlatSet!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        impl < T : Clone + Eq > JoinSemiLattice for FlatSet < T > { fn join (& mut self , other : & Self) -> bool { let result = match (& * self , other) { (Self :: Top , _) | (_ , Self :: Bottom) => return false , (Self :: Elem (a) , Self :: Elem (b)) if a == b => return false , (Self :: Bottom , Self :: Elem (x)) => Self :: Elem (x . clone ()) , _ => Self :: Top , } ; * self = result ; true } }
    };
}

impl_84!();