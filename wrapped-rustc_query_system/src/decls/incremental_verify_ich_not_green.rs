macro_rules! deps {
    () => {
        DepContext!();
    };
}

macro_rules! incremental_verify_ich_not_green {
    () => {
        deps!();
        # [cold] # [inline (never)] fn incremental_verify_ich_not_green < Tcx > (tcx : Tcx , prev_index : SerializedDepNodeIndex) where Tcx : DepContext , { panic ! ("fingerprint for green query instance not loaded from cache: {:?}" , tcx . dep_graph () . data () . unwrap () . prev_node_of (prev_index)) }
    };
}

incremental_verify_ich_not_green!()