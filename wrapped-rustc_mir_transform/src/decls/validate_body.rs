macro_rules! validate_body {
    () => {
        pub (super) fn validate_body < 'tcx > (tcx : TyCtxt < 'tcx > , body : & mut Body < 'tcx > , when : String) { validate :: Validator { when } . run_pass (tcx , body) ; }
    };
}

validate_body!()