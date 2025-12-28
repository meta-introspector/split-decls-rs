macro_rules! deps {
    () => {
        CanonicalVarKind!();
        Interner!();
        ParamEnv!();
        GenericArg!();
    };
}

macro_rules! CanonicalParamEnvCacheEntry {
    () => {
        deps!();
        # [derive_where (Clone , Debug ; I : Interner)] pub struct CanonicalParamEnvCacheEntry < I : Interner > { pub param_env : I :: ParamEnv , pub variables : Vec < I :: GenericArg > , pub variable_lookup_table : HashMap < I :: GenericArg , usize > , pub var_kinds : Vec < CanonicalVarKind < I > > , }
    };
}

CanonicalParamEnvCacheEntry!();