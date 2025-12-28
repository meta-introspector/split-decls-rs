macro_rules! deps {
    () => {
        TyQ!();
    };
}

macro_rules! DictKey {
    () => {
        deps!();
        # [doc = " Substitution dictionary key."] # [derive (Eq , Hash , PartialEq)] pub (crate) enum DictKey < 'tcx > { Ty (Ty < 'tcx > , TyQ) , Region (Region < 'tcx >) , Const (Const < 'tcx >) , Predicate (ExistentialPredicate < 'tcx >) , }
    };
}

DictKey!()