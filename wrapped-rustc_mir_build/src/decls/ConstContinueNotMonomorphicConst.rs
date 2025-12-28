macro_rules! deps {
    () => {
        ConstContinueNotMonomorphicConstReason!();
    };
}

macro_rules! ConstContinueNotMonomorphicConst {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (mir_build_const_continue_not_const)] # [help] pub (crate) struct ConstContinueNotMonomorphicConst { # [primary_span] pub span : Span , # [subdiagnostic] pub reason : ConstContinueNotMonomorphicConstReason , }
    };
}

ConstContinueNotMonomorphicConst!();