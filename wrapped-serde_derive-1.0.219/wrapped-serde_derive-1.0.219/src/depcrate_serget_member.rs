// Generated macro for get_member (function)
macro_rules! Depcrate_serget_member {
() => {
// Module: crate::ser
// Provides: {"get_member"}
// Dependencies: {}
fn get_member (params : & Parameters , field : & Field , member : & Member) -> TokenStream { let self_var = & params . self_var ; match (params . is_remote , field . attrs . getter ()) { (false , None) => { if params . is_packed { quote ! (& { # self_var .# member }) } else { quote ! (&# self_var .# member) } } (true , None) => { let inner = if params . is_packed { quote ! (& { # self_var .# member }) } else { quote ! (&# self_var .# member) } ; let ty = field . ty ; quote ! (_serde :: __private :: ser :: constrain ::<# ty > (# inner)) } (true , Some (getter)) => { let ty = field . ty ; quote ! (_serde :: __private :: ser :: constrain ::<# ty > (&# getter (# self_var))) } (false , Some (_)) => { unreachable ! ("getter is only allowed for remote impls") ; } } }
};
}
