macro_rules! deps {
    () => {
        Value!();
        QueryContext!();
        HandleCycleError!();
        QueryConfig!();
        CycleError!();
        Cycle!();
    };
}

macro_rules! handle_cycle_error {
    () => {
        deps!();
        fn handle_cycle_error < Q , Qcx > (query : Q , qcx : Qcx , cycle_error : & CycleError , error : Diag < '_ > ,) -> Q :: Value where Q : QueryConfig < Qcx > , Qcx : QueryContext , { use HandleCycleError :: * ; match query . handle_cycle_error () { Error => { let guar = error . emit () ; query . value_from_cycle_error (* qcx . dep_context () , cycle_error , guar) } Fatal => { error . emit () ; qcx . dep_context () . sess () . dcx () . abort_if_errors () ; unreachable ! () } DelayBug => { let guar = error . delay_as_bug () ; query . value_from_cycle_error (* qcx . dep_context () , cycle_error , guar) } Stash => { let guar = if let Some (root) = cycle_error . cycle . first () && let Some (span) = root . query . info . span { error . stash (span , StashKey :: Cycle) . unwrap () } else { error . emit () } ; query . value_from_cycle_error (* qcx . dep_context () , cycle_error , guar) } } }
    };
}

handle_cycle_error!()