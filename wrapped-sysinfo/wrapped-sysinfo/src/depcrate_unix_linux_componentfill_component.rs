// Generated macro for fill_component (function)
macro_rules! Depcrate_unix_linux_componentfill_component {
() => {
// Module: crate::unix::linux::component
// Provides: {"fill_component"}
// Dependencies: {}
# [doc = " Check given `item` dispatch to read the right `file` with the right parsing and store data in"] # [doc = " given `component`. `id` is provided for `label` creation."] fn fill_component (component : & mut ComponentInner , item : & str , folder : & Path , file : & str) { let hwmon_file = folder . join (file) ; match item { "type" => { component . sensor_type = read_number_from_file :: < u8 > (& hwmon_file) . map (ThermalSensorType :: from) } "input" => { let temperature = get_temperature_from_file (& hwmon_file) ; component . input_file = Some (hwmon_file) ; component . temperature = temperature ; if component . max . is_none () { component . max = temperature ; } } "label" => component . label = get_file_line (& hwmon_file , 10) . unwrap_or_default () , "highest" => { component . max = get_temperature_from_file (& hwmon_file) . or (component . temperature) ; component . highest_file = Some (hwmon_file) ; } "crit" => component . threshold_critical = get_temperature_from_file (& hwmon_file) , _ => { sysinfo_debug ! ("This hwmon-temp file is still not supported! Contributions are appreciated.;) {:?}" , hwmon_file ,) ; } } }
};
}
