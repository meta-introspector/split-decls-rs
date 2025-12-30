// Generated macro for Emitter (trait)
macro_rules! Depcrate_emitterEmitter {
() => {
// Module: crate::emitter
// Provides: {"Emitter"}
// Dependencies: {}
pub (crate) trait Emitter { fn emit_formatted_file (& mut self , output : & mut dyn Write , formatted_file : FormattedFile < '_ > ,) -> Result < EmitterResult , io :: Error > ; fn emit_header (& self , _output : & mut dyn Write) -> Result < () , io :: Error > { Ok (()) } fn emit_footer (& self , _output : & mut dyn Write) -> Result < () , io :: Error > { Ok (()) } }
};
}
