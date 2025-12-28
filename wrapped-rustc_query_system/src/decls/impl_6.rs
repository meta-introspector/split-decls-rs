macro_rules! deps {
    () => {
        WithDepNode!();
        DepContext!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl < T : Clone > WithDepNode < T > { pub fn new (dep_node : DepNodeIndex , cached_value : T) -> Self { WithDepNode { dep_node , cached_value } } pub fn get < Tcx : DepContext > (& self , tcx : Tcx) -> T { tcx . dep_graph () . read_index (self . dep_node) ; self . cached_value . clone () } }
    };
}

impl_6!()