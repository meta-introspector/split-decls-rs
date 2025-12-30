// Generated macro for macro_89 (macro)
macro_rules! Depcrate_conditionmacro_89 {
() => {
// Module: crate::condition
// Provides: {"macro_89"}
// Dependencies: {}
conditions ! { feature = "detect-env" env_set_or ("CLICOLOR_FORCE" , false) || env_set_or ("CLICOLOR" , true) , CLICOLOR : clicolor , CLICOLOR_LIVE : clicolor_live , ! env_set_or ("NO_COLOR" , false) , YES_COLOR : no_color , YES_COLOR_LIVE : no_color_live , }
};
}
