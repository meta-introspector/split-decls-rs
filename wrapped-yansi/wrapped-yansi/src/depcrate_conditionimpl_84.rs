// Generated macro for impl_84 (impl)
macro_rules! Depcrate_conditionimpl_84 {
() => {
// Module: crate::condition
// Provides: {"impl_84"}
// Dependencies: {}
impl fmt :: Debug for Condition { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if * self == Condition :: DEFAULT { f . write_str ("Condition::DEFAULT") } else if * self == Condition :: ALWAYS { f . write_str ("Condition::ALWAYS") } else if * self == Condition :: NEVER { f . write_str ("Condition::NEVER") } else { f . debug_tuple ("Condition") . field (& self . 0) . finish () } } }
};
}
