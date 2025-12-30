// Generated macro for impl_32 (impl)
macro_rules! Depcrateimpl_32 {
() => {
// Module: crate
// Provides: {"impl_32"}
// Dependencies: {}
impl < 'a , 'b , X , F > MapAccess < 'a , 'b , X , F > { fn new (delegate : X , callback : & 'b mut F , path : & 'a Path < 'a >) -> Self { MapAccess { delegate , callback , path , key : None , } } fn key < E > (& mut self) -> Result < String , E > where E : de :: Error , { self . key . take () . ok_or_else (| | E :: custom ("non-string key")) } }
};
}
