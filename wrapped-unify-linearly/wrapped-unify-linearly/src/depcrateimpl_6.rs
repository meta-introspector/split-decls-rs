// Generated macro for impl_6 (impl)
macro_rules! Depcrateimpl_6 {
() => {
// Module: crate
// Provides: {"impl_6"}
// Dependencies: {}
impl < L , R > Parser for Chain < L , R > where L : Parser , R : Parser < Input = L :: Input > { type Input = L :: Input ; type Output = (L :: Output , R :: Output) ; fn parse (self , _input : Self :: Input) -> Result < (Self :: Output , Self :: Input) , () > { Err (()) } }
};
}
