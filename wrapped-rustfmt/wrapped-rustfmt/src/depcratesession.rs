// Generated macro for Session (struct)
macro_rules! DepcrateSession {
() => {
// Module: crate
// Provides: {"Session"}
// Dependencies: {}
# [doc = " A session is a run of rustfmt across a single or multiple inputs."] pub struct Session < 'b , T : Write > { pub config : Config , pub out : Option < & 'b mut T > , pub (crate) errors : ReportedErrors , source_file : SourceFile , emitter : Box < dyn Emitter + 'b > , }
};
}
