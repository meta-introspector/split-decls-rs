// Generated macro for macro_233 (macro)
macro_rules! Depcrate_tymacro_233 {
() => {
// Module: crate::ty
// Provides: {"macro_233"}
// Dependencies: {}
ast_struct ! { # [doc = " The explicit Self type in a \"qualified path\". The actual"] # [doc = " path, including the trait and the associated item, is stored"] # [doc = " separately. `position` represents the index of the associated"] # [doc = " item qualified with this Self type."] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = " <Vec<T> as a::b::Trait>::AssociatedItem"] # [doc = "  ^~~~~     ~~~~~~~~~~~~~~^"] # [doc = "  ty        position = 3"] # [doc = ""] # [doc = " <Vec<T>>::AssociatedItem"] # [doc = "  ^~~~~    ^"] # [doc = "  ty       position = 0"] # [doc = " ```"] pub struct QSelf { pub lt_token : tokens :: Lt , pub ty : Box < Ty >, pub position : usize , pub as_token : Option < tokens :: As >, pub gt_token : tokens :: Gt , } }
};
}
