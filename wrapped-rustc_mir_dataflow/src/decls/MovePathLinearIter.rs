macro_rules! deps {
    () => {
        MovePath!();
    };
}

macro_rules! MovePathLinearIter {
    () => {
        deps!();
        struct MovePathLinearIter < 'a , 'tcx , F > { next : Option < (MovePathIndex , & 'a MovePath < 'tcx >) > , fetch_next : F , }
    };
}

MovePathLinearIter!()