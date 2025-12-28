macro_rules! Uncovered {
    () => {
        # [derive (Subdiagnostic)] # [label (pattern_analysis_uncovered)] pub struct Uncovered { # [primary_span] span : Span , count : usize , witness_1 : String , witness_2 : String , witness_3 : String , remainder : usize , }
    };
}

Uncovered!();