macro_rules! deps {
    () => {
        DepNode!();
    };
}

macro_rules! DepGraphQuery {
    () => {
        deps!();
        pub struct DepGraphQuery { pub graph : LinkedGraph < DepNode , () > , pub indices : FxHashMap < DepNode , NodeIndex > , pub dep_index_to_index : IndexVec < DepNodeIndex , Option < NodeIndex > > , }
    };
}

DepGraphQuery!()