macro_rules! deps {
    () => {
        Replacement!();
        DiagnosticSpan!();
    };
}

macro_rules! collect_span {
    () => {
        deps!();
        # [doc = " Converts a [`DiagnosticSpan`] into a [`Replacement`]."] fn collect_span (span : & DiagnosticSpan) -> Option < Replacement > { let snippet = span_to_snippet (span) ; let replacement = span . suggested_replacement . clone () ? ; Some (Replacement { snippet , replacement , }) }
    };
}

collect_span!()