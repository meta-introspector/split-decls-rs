// Generated macro for impl_127 (impl)
macro_rules! Depcrate_macro_interpreterimpl_127 {
() => {
// Module: crate::macro_interpreter
// Provides: {"impl_127"}
// Dependencies: {}
impl RdfStateMachine { pub fn new () -> Self { Self :: default () } pub fn emit_triple (& mut self , subject : & str , predicate : & str , object : & str) { let triple = RdfTriple { subject : subject . to_string () , predicate : predicate . to_string () , object : object . to_string () , timestamp : std :: time :: SystemTime :: now () . duration_since (std :: time :: UNIX_EPOCH) . unwrap () . as_millis () as u64 , } ; self . triples . push (triple) ; } pub fn enter_function (& mut self , func_name : & str) { self . execution_state . call_stack . push (func_name . to_string ()) ; self . execution_state . current_function = Some (func_name . to_string ()) ; self . emit_triple (func_name , "rdf:type" , "ExecutionFunction") ; self . emit_triple (func_name , "execution:entered" , & format ! ("{}" , self . timestamp ())) ; } pub fn exit_function (& mut self , func_name : & str) { self . execution_state . call_stack . pop () ; self . execution_state . current_function = self . execution_state . call_stack . last () . cloned () ; self . emit_triple (func_name , "execution:exited" , & format ! ("{}" , self . timestamp ())) ; } pub fn capture_data (& mut self , key : & str , value : & str) { self . execution_state . data_captured . insert (key . to_string () , value . to_string ()) ; if let Some (current_func) = self . execution_state . current_function . clone () { self . emit_triple (& current_func , & format ! ("data:{}" , key) , value) ; } } fn timestamp (& self) -> u64 { std :: time :: SystemTime :: now () . duration_since (std :: time :: UNIX_EPOCH) . unwrap () . as_millis () as u64 } }
};
}
