// Generated macro for impl_738 (impl)
macro_rules! Depcrate_unix_linux_componentimpl_738 {
() => {
// Module: crate::unix::linux::component
// Provides: {"impl_738"}
// Dependencies: {}
impl From < u8 > for ThermalSensorType { fn from (input : u8) -> Self { match input { 0 => Self :: CPUEmbeddedDiode , 1 => Self :: Transistor3904 , 3 => Self :: ThermalDiode , 4 => Self :: Thermistor , 5 => Self :: AMDAMDSI , 6 => Self :: IntelPECI , n => Self :: Unknown (n) , } } }
};
}
