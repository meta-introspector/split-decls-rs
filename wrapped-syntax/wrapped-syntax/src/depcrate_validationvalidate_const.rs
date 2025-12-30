// Generated macro for validate_const (function)
macro_rules! Depcrate_validationvalidate_const {
() => {
// Module: crate::validation
// Provides: {"validate_const"}
// Dependencies: {}
fn validate_const (const_ : ast :: Const , errors : & mut Vec < SyntaxError >) { if let Some (mut_token) = const_ . const_token () . and_then (| t | t . next_token ()) . and_then (| t | algo :: skip_trivia_token (t , Direction :: Next)) . filter (| t | t . kind () == T ! [mut]) { errors . push (SyntaxError :: new ("const globals cannot be mutable" , mut_token . text_range ())) ; } }
};
}
