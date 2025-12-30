// Generated macro for ValidationAttrs (struct)
macro_rules! Depcrate_attr_validationValidationAttrs {
() => {
// Module: crate::attr::validation
// Provides: {"ValidationAttrs"}
// Dependencies: {}
# [derive (Default)] pub struct ValidationAttrs { pub length : Option < LengthOrRange > , pub range : Option < LengthOrRange > , pub pattern : Option < Expr > , pub regex : Option < Expr > , pub contains : Option < Expr > , pub required : bool , pub format : Option < Format > , pub inner : Option < Box < ValidationAttrs > > , }
};
}
