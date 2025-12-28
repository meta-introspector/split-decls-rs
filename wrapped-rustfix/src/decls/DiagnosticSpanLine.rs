macro_rules! deps {
    () => {
        Span!();
    };
}

macro_rules! DiagnosticSpanLine {
    () => {
        deps!();
        # [doc = " Span information of a single line."] # [derive (Clone , Deserialize , Debug , Eq , PartialEq , Hash)] pub struct DiagnosticSpanLine { pub text : String , # [doc = " 1-based, character offset in self.text."] pub highlight_start : usize , pub highlight_end : usize , }
    };
}

DiagnosticSpanLine!();