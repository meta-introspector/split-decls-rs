// Generated macro for look_up_flag_from_env (function)
macro_rules! Depcrate_cmdlinelook_up_flag_from_env {
() => {
// Module: crate::cmdline
// Provides: {"look_up_flag_from_env"}
// Dependencies: {}
fn look_up_flag_from_env (flag : & str) -> Option < FlagType > { env :: var (& env_var_for_flag (flag)) . ok () . map (| value | match & * value { "pass" => FlagType :: Pass (false) , "pass-arg" => FlagType :: Pass (true) , "drop" => FlagType :: Drop (false) , "drop-arg" => FlagType :: Drop (true) , _ => FlagType :: Error ("incorrect flag type in environment; \
                                  must be one of `pass`, `pass-arg`, \
                                  `drop`, `drop-arg`") , }) }
};
}
