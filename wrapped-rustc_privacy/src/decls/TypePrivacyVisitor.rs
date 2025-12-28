macro_rules! TypePrivacyVisitor {
    () => {
        # [doc = " Type privacy visitor, checks types for privacy and reports violations."] # [doc = " Both explicitly written types and inferred types of expressions and patterns are checked."] # [doc = " Checks are performed on \"semantic\" types regardless of names and their hygiene."] struct TypePrivacyVisitor < 'tcx > { tcx : TyCtxt < 'tcx > , module_def_id : LocalModDefId , maybe_typeck_results : Option < & 'tcx ty :: TypeckResults < 'tcx > > , span : Span , }
    };
}

TypePrivacyVisitor!();