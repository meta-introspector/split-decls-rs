macro_rules! deps {
    () => {
        AdtDefinedHere!();
        InterpretedAsConst!();
        InterpretedAsConstSugg!();
        SuggestLet!();
        Inform!();
        MiscPatternSuggestion!();
    };
}

macro_rules! PatternNotCovered {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (mir_build_pattern_not_covered , code = E0005)] pub (crate) struct PatternNotCovered < 's , 'tcx > { # [primary_span] pub (crate) span : Span , pub (crate) origin : & 's str , # [subdiagnostic] pub (crate) uncovered : Uncovered , # [subdiagnostic] pub (crate) inform : Option < Inform > , # [subdiagnostic] pub (crate) interpreted_as_const : Option < InterpretedAsConst > , # [subdiagnostic] pub (crate) interpreted_as_const_sugg : Option < InterpretedAsConstSugg > , # [subdiagnostic] pub (crate) adt_defined_here : Option < AdtDefinedHere < 'tcx > > , # [note (mir_build_privately_uninhabited)] pub (crate) witness_1_is_privately_uninhabited : bool , pub (crate) witness_1 : String , # [note (mir_build_pattern_ty)] pub (crate) _p : () , pub (crate) pattern_ty : Ty < 'tcx > , # [subdiagnostic] pub (crate) let_suggestion : Option < SuggestLet > , # [subdiagnostic] pub (crate) misc_suggestion : Option < MiscPatternSuggestion > , }
    };
}

PatternNotCovered!()