// Generated macro for type_will_always_be_passed_directly (function)
macro_rules! Depcrate_deduce_param_attrstype_will_always_be_passed_directly {
() => {
// Module: crate::deduce_param_attrs
// Provides: {"type_will_always_be_passed_directly"}
// Dependencies: {}
# [doc = " Returns true if values of a given type will never be passed indirectly, regardless of ABI."] fn type_will_always_be_passed_directly (ty : Ty < '_ >) -> bool { matches ! (ty . kind () , ty :: Bool | ty :: Char | ty :: Float (..) | ty :: Int (..) | ty :: RawPtr (..) | ty :: Ref (..) | ty :: Slice (..) | ty :: Uint (..)) }
};
}
