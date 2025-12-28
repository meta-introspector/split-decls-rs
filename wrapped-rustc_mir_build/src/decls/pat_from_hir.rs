macro_rules! deps {
    () => {
        PatMigration!();
        PatCtxt!();
    };
}

macro_rules! pat_from_hir {
    () => {
        deps!();
        pub (super) fn pat_from_hir < 'a , 'tcx > (tcx : TyCtxt < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > , typeck_results : & 'a ty :: TypeckResults < 'tcx > , pat : & 'tcx hir :: Pat < 'tcx > ,) -> Box < Pat < 'tcx > > { let mut pcx = PatCtxt { tcx , typing_env , typeck_results , rust_2024_migration : typeck_results . rust_2024_migration_desugared_pats () . get (pat . hir_id) . map (PatMigration :: new) , } ; let result = pcx . lower_pattern (pat) ; debug ! ("pat_from_hir({:?}) = {:?}" , pat , result) ; if let Some (m) = pcx . rust_2024_migration { m . emit (tcx , pat . hir_id) ; } result }
    };
}

pat_from_hir!();