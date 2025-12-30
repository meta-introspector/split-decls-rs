// Generated macro for impl_77 (impl)
macro_rules! Depcrate_expressionimpl_77 {
() => {
// Module: crate::expression
// Provides: {"impl_77"}
// Dependencies: {}
impl TryFrom < & StaticDefinition > for Expression { type Error = String ; fn try_from (sd : & StaticDefinition) -> Result < Self , Self :: Error > { match sd { StaticDefinition :: Constant (imm) => Ok (imm . into ()) , StaticDefinition :: Generic (t) => t . parse () , } } }
};
}
