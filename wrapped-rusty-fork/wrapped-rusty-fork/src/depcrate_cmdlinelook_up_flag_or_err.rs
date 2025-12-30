// Generated macro for look_up_flag_or_err (function)
macro_rules! Depcrate_cmdlinelook_up_flag_or_err {
() => {
// Module: crate::cmdline
// Provides: {"look_up_flag_or_err"}
// Dependencies: {}
fn look_up_flag_or_err (flag : & str) -> Result < (bool , bool) > { match look_up_flag (flag) { None => Err (Error :: UnknownFlag (flag . to_owned ())) , Some (FlagType :: Error (message)) => Err (Error :: DisallowedFlag (flag . to_owned () , message . to_owned ())) , Some (FlagType :: Pass (has_arg)) => Ok ((true , has_arg)) , Some (FlagType :: Drop (has_arg)) => Ok ((false , has_arg)) , } }
};
}
