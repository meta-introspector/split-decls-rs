macro_rules! deps {
    () => {
        MovePath!();
        MovePathLinearIter!();
    };
}

macro_rules! impl_190 {
    () => {
        deps!();
        impl < 'a , 'tcx , F > Iterator for MovePathLinearIter < 'a , 'tcx , F > where F : FnMut (MovePathIndex , & 'a MovePath < 'tcx >) -> Option < (MovePathIndex , & 'a MovePath < 'tcx >) > , { type Item = (MovePathIndex , & 'a MovePath < 'tcx >) ; fn next (& mut self) -> Option < Self :: Item > { let ret = self . next . take () ? ; self . next = (self . fetch_next) (ret . 0 , ret . 1) ; Some (ret) } }
    };
}

impl_190!();