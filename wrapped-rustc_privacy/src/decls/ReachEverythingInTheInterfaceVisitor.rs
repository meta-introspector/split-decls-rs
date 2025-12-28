macro_rules! deps {
    () => {
        EmbargoVisitor!();
    };
}

macro_rules! ReachEverythingInTheInterfaceVisitor {
    () => {
        deps!();
        struct ReachEverythingInTheInterfaceVisitor < 'a , 'tcx > { effective_vis : EffectiveVisibility , item_def_id : LocalDefId , ev : & 'a mut EmbargoVisitor < 'tcx > , level : Level , }
    };
}

ReachEverythingInTheInterfaceVisitor!()