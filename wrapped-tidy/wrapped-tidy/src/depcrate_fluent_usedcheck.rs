// Generated macro for check (function)
macro_rules! Depcrate_fluent_usedcheck {
() => {
// Module: crate::fluent_used
// Provides: {"check"}
// Dependencies: {}
pub fn check (path : & Path , mut all_defined_msgs : HashMap < String , String > , bad : & mut bool) { let mut msgs_appear_only_once = HashMap :: new () ; walk (path , | path , _ | filter_dirs (path) , & mut | _ , contents | { filter_used_messages (contents , & mut all_defined_msgs , & mut msgs_appear_only_once) ; }) ; for (name , filename) in msgs_appear_only_once { tidy_error ! (bad , "{filename}: message `{}` is not used" , name ,) ; } }
};
}
