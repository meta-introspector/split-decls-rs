macro_rules! deps {
    () => {
        Durability!();
        Revision!();
        Runtime!();
    };
}

macro_rules! impl_278 {
    () => {
        deps!();
        impl Default for Runtime { fn default () -> Self { Runtime { revisions : [Revision :: start () ; Durability :: LEN] , revision_canceled : Default :: default () , dependency_graph : Default :: default () , table : Default :: default () , } } }
    };
}

impl_278!()