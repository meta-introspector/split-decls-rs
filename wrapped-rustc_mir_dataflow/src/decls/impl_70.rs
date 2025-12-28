macro_rules! deps {
    () => {
        DebugWithContext!();
        StateDiffCollector!();
        OutputStyle!();
        Results!();
        Analysis!();
    };
}

macro_rules! impl_70 {
    () => {
        deps!();
        impl < D > StateDiffCollector < D > { fn run < 'tcx , A > (body : & Body < 'tcx > , block : BasicBlock , analysis : & mut A , results : & Results < A :: Domain > , style : OutputStyle ,) -> Self where A : Analysis < 'tcx , Domain = D > , D : DebugWithContext < A > , { let mut collector = StateDiffCollector { prev_state : analysis . bottom_value (body) , after : vec ! [] , before : (style == OutputStyle :: BeforeAndAfter) . then_some (vec ! []) , } ; visit_results (body , std :: iter :: once (block) , analysis , results , & mut collector) ; collector } }
    };
}

impl_70!()