// Generated macro for impl_130 (impl)
macro_rules! Depcrateimpl_130 {
() => {
// Module: crate
// Provides: {"impl_130"}
// Dependencies: {}
impl < 'a > StepResult < 'a > { fn new (name : & 'a str , result : Result < () >) -> StepResult < 'a > { StepResult { name , result , should_fail : false , metadata : Vec :: new () , } } fn should_fail (mut self , fail : bool) -> Self { self . should_fail = fail ; self } fn metadata (mut self , name : & 'a str , value : impl fmt :: Display) -> Self { self . metadata . push ((name , value . to_string ())) ; self } }
};
}
