// Generated macro for init_logger (function)
macro_rules! Depcrateinit_logger {
() => {
// Module: crate
// Provides: {"init_logger"}
// Dependencies: {}
# [doc = " Initialize the logger with the given values for the filter, coloring, and other options env variables."] pub fn init_logger (cfg : LoggerConfig) -> Result < () , Error > { init_logger_with_additional_layer (cfg , | | Registry :: default ()) }
};
}
