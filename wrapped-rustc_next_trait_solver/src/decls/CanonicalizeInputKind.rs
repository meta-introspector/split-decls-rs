macro_rules! CanonicalizeInputKind {
    () => {
        # [derive (Debug , Clone , Copy)] enum CanonicalizeInputKind { # [doc = " When canonicalizing the `param_env`, we keep `'static` as merging"] # [doc = " trait candidates relies on it when deciding whether a where-bound"] # [doc = " is trivial."] ParamEnv , # [doc = " When canonicalizing predicates, we don't keep `'static`."] Predicate , }
    };
}

CanonicalizeInputKind!()