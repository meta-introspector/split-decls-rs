macro_rules! deps {
    () => {
        DiagnosticSpan!();
        DiagnosticCode!();
    };
}

macro_rules! Diagnostic {
    () => {
        deps!();
        # [doc = " The root diagnostic JSON output emitted by the compiler."] # [derive (Clone , Deserialize , Debug , Hash , Eq , PartialEq)] pub struct Diagnostic { # [doc = " The primary error message."] pub message : String , pub code : Option < DiagnosticCode > , # [doc = " \"error: internal compiler error\", \"error\", \"warning\", \"note\", \"help\"."] level : String , pub spans : Vec < DiagnosticSpan > , # [doc = " Associated diagnostic messages."] pub children : Vec < Diagnostic > , # [doc = " The message as rustc would render it."] pub rendered : Option < String > , }
    };
}

Diagnostic!()