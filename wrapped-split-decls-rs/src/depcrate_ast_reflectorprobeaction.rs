// Generated macro for ProbeAction (enum)
macro_rules! Depcrate_ast_reflectorProbeAction {
() => {
// Module: crate::ast_reflector
// Provides: {"ProbeAction"}
// Dependencies: {}
# [derive (Debug , Clone , Serialize , Deserialize)] pub enum ProbeAction { Log { message : String } , AddAttribute { attr : String } , WrapFunction { wrapper : String } , InjectCode { code : String , position : InjectionPosition } , Transform { macro_name : String , args : Vec < String > } , Collect { field : String } , Enhance { enhancement_type : String , data : String } , }
};
}
