// Generated macro for get_temperature_from_file (function)
macro_rules! Depcrate_unix_linux_componentget_temperature_from_file {
() => {
// Module: crate::unix::linux::component
// Provides: {"get_temperature_from_file"}
// Dependencies: {}
# [inline] fn get_temperature_from_file (file : & Path) -> Option < f32 > { let temp = read_number_from_file (file) ; convert_temp_celsius (temp) }
};
}
