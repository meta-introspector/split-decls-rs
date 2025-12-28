macro_rules! StopAfterDataFlowEndedCompilation {
    () => {
        # [derive (Diagnostic)] # [diag (mir_dataflow_stop_after_dataflow_ended_compilation)] pub (crate) struct StopAfterDataFlowEndedCompilation ;
    };
}

StopAfterDataFlowEndedCompilation!()