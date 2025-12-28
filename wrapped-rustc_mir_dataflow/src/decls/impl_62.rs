macro_rules! deps {
    () => {
        Formatter!();
        OutputStyle!();
        Results!();
        Analysis!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl < 'mir , 'tcx , A > Formatter < 'mir , 'tcx , A > where A : Analysis < 'tcx > , { fn new (body : & 'mir Body < 'tcx > , analysis : & 'mir mut A , results : & 'mir Results < A :: Domain > , style : OutputStyle ,) -> Self { let reachable = traversal :: reachable_as_bitset (body) ; Formatter { body , analysis : analysis . into () , results , style , reachable } } }
    };
}

impl_62!();