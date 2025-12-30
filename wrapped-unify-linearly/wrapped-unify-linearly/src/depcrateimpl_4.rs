// Generated macro for impl_4 (impl)
macro_rules! Depcrateimpl_4 {
() => {
// Module: crate
// Provides: {"impl_4"}
// Dependencies: {}
impl < T > Parser for Token < T > where T : Iterator { type Input = T ; type Output = T :: Item ; fn parse (self , _input : Self :: Input) -> Result < (Self :: Output , Self :: Input) , () > { Err (()) } }
};
}
