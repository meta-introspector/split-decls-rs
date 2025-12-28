macro_rules! thir_flat {
    () => {
        # [doc = " Create a list-like THIR representation for debugging."] pub fn thir_flat (tcx : TyCtxt < '_ > , owner_def : LocalDefId) -> String { match super :: cx :: thir_body (tcx , owner_def) { Ok ((thir , _)) => format ! ("{:#?}" , thir . steal ()) , Err (_) => "error" . into () , } }
    };
}

thir_flat!()