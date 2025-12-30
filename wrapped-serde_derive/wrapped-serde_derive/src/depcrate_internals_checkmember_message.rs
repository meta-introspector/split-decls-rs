// Generated macro for member_message (function)
macro_rules! Depcrate_internals_checkmember_message {
() => {
// Module: crate::internals::check
// Provides: {"member_message"}
// Dependencies: {}
fn member_message (member : & Member) -> String { match member { Member :: Named (ident) => format ! ("`{}`" , ident) , Member :: Unnamed (i) => format ! ("#{}" , i . index) , } }
};
}
