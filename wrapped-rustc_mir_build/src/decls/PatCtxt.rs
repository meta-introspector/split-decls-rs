macro_rules! deps {
    () => {
        PatMigration!();
    };
}

macro_rules! PatCtxt {
    () => {
        deps!();
        struct PatCtxt < 'a , 'tcx > { tcx : TyCtxt < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > , typeck_results : & 'a ty :: TypeckResults < 'tcx > , # [doc = " Used by the Rust 2024 migration lint."] rust_2024_migration : Option < PatMigration < 'a > > , }
    };
}

PatCtxt!();