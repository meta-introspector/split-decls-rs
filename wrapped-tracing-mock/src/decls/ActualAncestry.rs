macro_rules! deps {
    () => {
        ActualSpan!();
    };
}

macro_rules! ActualAncestry {
    () => {
        deps!();
        pub (crate) enum ActualAncestry { HasExplicitParent (ActualSpan) , IsExplicitRoot , HasContextualParent (ActualSpan) , IsContextualRoot , }
    };
}

ActualAncestry!();