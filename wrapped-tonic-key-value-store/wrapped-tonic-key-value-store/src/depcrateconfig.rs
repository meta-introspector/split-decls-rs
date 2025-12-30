// Generated macro for Config (struct)
macro_rules! DepcrateConfig {
() => {
// Module: crate
// Provides: {"Config"}
// Dependencies: {}
# [doc = " Simple key/value store with an HTTP API"] # [derive (Debug , Parser)] struct Config { # [doc = " The port to listen on"] # [arg (short = 'p' , long , default_value = "3000")] port : u16 , # [command (subcommand)] command : Command , }
};
}
