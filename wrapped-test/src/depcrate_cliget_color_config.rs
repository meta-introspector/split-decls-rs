// Generated macro for get_color_config (function)
macro_rules! Depcrate_cliget_color_config {
() => {
// Module: crate::cli
// Provides: {"get_color_config"}
// Dependencies: {}
fn get_color_config (matches : & getopts :: Matches) -> OptPartRes < ColorConfig > { let color = match matches . opt_str ("color") . as_deref () { Some ("auto") | None => ColorConfig :: AutoColor , Some ("always") => ColorConfig :: AlwaysColor , Some ("never") => ColorConfig :: NeverColor , Some (v) => { return Err (format ! ("argument for --color must be auto, always, or never (was \
                 {v})")) ; } } ; Ok (color) }
};
}
