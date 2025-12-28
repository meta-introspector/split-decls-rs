macro_rules! deps {
    () => {
        WipProbe!();
    };
}

macro_rules! WipEvaluationStep {
    () => {
        deps!();
        # [derive_where (PartialEq , Eq , Debug ; I : Interner)] struct WipEvaluationStep < I : Interner > { # [doc = " Unlike `EvalCtxt::var_values`, we append a new"] # [doc = " generic arg here whenever we create a new inference"] # [doc = " variable."] # [doc = ""] # [doc = " This is necessary as we otherwise don't unify these"] # [doc = " vars when instantiating multiple `CanonicalState`."] var_values : Vec < I :: GenericArg > , probe_depth : usize , evaluation : WipProbe < I > , }
    };
}

WipEvaluationStep!();