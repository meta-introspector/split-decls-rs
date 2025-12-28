macro_rules! WithDepNode {
    () => {
        # [derive (Debug , Clone , Eq , PartialEq)] pub struct WithDepNode < T > { dep_node : DepNodeIndex , cached_value : T , }
    };
}

WithDepNode!();