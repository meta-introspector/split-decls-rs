macro_rules! deps {
    () => {
        ThirPrinter!();
    };
}

macro_rules! thir_tree {
    () => {
        deps!();
        # [doc = " Create a THIR tree for debugging."] pub fn thir_tree (tcx : TyCtxt < '_ > , owner_def : LocalDefId) -> String { match super :: cx :: thir_body (tcx , owner_def) { Ok ((thir , expr)) => { let thir = thir . steal () ; let mut printer = ThirPrinter :: new (& thir) ; printer . print (expr) ; printer . into_buffer () } Err (_) => "error" . into () , } }
    };
}

thir_tree!()