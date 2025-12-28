macro_rules! dedup_dtorck_constraint {
    () => {
        fn dedup_dtorck_constraint (c : & mut DropckConstraint < '_ >) { let mut outlives = FxHashSet :: default () ; let mut dtorck_types = FxHashSet :: default () ; c . outlives . retain (| & val | outlives . replace (val) . is_none ()) ; c . dtorck_types . retain (| & val | dtorck_types . replace (val) . is_none ()) ; }
    };
}

dedup_dtorck_constraint!()