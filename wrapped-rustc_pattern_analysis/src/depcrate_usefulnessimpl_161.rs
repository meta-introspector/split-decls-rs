// Generated macro for impl_161 (impl)
macro_rules! Depcrate_usefulnessimpl_161 {
() => {
// Module: crate::usefulness
// Provides: {"impl_161"}
// Dependencies: {}
impl < 'a , 'p , Cx : PatCx > UsefulnessCtxt < 'a , 'p , Cx > { fn increase_complexity_level (& mut self , complexity_add : usize) -> Result < () , Cx :: Error > { self . complexity_level += complexity_add ; if self . complexity_level <= self . complexity_limit { Ok (()) } else { self . tycx . complexity_exceeded () } } }
};
}
