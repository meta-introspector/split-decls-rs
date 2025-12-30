// Generated macro for ThermalSensorType (enum)
macro_rules! Depcrate_unix_linux_componentThermalSensorType {
() => {
// Module: crate::unix::linux::component
// Provides: {"ThermalSensorType"}
// Dependencies: {}
# [doc = " Information about thermal sensor. It may be unavailable as it's"] # [doc = " kernel module and chip dependent."] enum ThermalSensorType { # [doc = " 1: CPU embedded diode"] CPUEmbeddedDiode , # [doc = " 2: 3904 transistor"] Transistor3904 , # [doc = " 3: thermal diode"] ThermalDiode , # [doc = " 4: thermistor"] Thermistor , # [doc = " 5: AMD AMDSI"] AMDAMDSI , # [doc = " 6: Intel PECI"] IntelPECI , # [doc = " Not all types are supported by all chips so we keep space for unknown sensors."] # [allow (dead_code)] Unknown (u8) , }
};
}
