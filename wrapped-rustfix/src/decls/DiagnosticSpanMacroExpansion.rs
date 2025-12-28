macro_rules! deps {
    () => {
        DiagnosticSpan!();
        Span!();
    };
}

macro_rules! DiagnosticSpanMacroExpansion {
    () => {
        deps!();
        # [doc = " Span information for macro expansions."] # [derive (Clone , Deserialize , Debug , Eq , PartialEq , Hash)] struct DiagnosticSpanMacroExpansion { # [doc = " span where macro was applied to generate this code; note that"] # [doc = " this may itself derive from a macro (if"] # [doc = " `span.expansion.is_some()`)"] span : DiagnosticSpan , # [doc = " name of macro that was applied (e.g., \"foo!\" or \"#[derive(Eq)]\")"] macro_decl_name : String , # [doc = " span where macro was defined (if known)"] def_site_span : Option < DiagnosticSpan > , }
    };
}

DiagnosticSpanMacroExpansion!()