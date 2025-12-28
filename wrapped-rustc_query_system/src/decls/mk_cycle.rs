macro_rules! deps {
    () => {
        CycleError!();
        Value!();
        QueryContext!();
        QueryConfig!();
    };
}

macro_rules! mk_cycle {
    () => {
        deps!();
        # [cold] # [inline (never)] fn mk_cycle < Q , Qcx > (query : Q , qcx : Qcx , cycle_error : CycleError) -> Q :: Value where Q : QueryConfig < Qcx > , Qcx : QueryContext , { let error = report_cycle (qcx . dep_context () . sess () , & cycle_error) ; handle_cycle_error (query , qcx , & cycle_error , error) }
    };
}

mk_cycle!()