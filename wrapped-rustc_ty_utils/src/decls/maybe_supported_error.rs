macro_rules! deps {
    () => {
        GenericConstantTooComplexSub!();
        GenericConstantTooComplex!();
    };
}

macro_rules! maybe_supported_error {
    () => {
        deps!();
        fn maybe_supported_error (tcx : TyCtxt < '_ > , sub : GenericConstantTooComplexSub , root_span : Span ,) -> Result < ! , ErrorGuaranteed > { let reported = tcx . dcx () . emit_err (GenericConstantTooComplex { span : root_span , maybe_supported : true , sub , }) ; Err (reported) }
    };
}

maybe_supported_error!()