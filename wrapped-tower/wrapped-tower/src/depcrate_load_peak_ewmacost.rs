// Generated macro for Cost (struct)
macro_rules! Depcrate_load_peak_ewmaCost {
() => {
// Module: crate::load::peak_ewma
// Provides: {"Cost"}
// Dependencies: {}
# [doc = " Represents the relative cost of communicating with a service."] # [doc = ""] # [doc = " The underlying value estimates the amount of pending work to a service: the Peak-EWMA"] # [doc = " latency estimate multiplied by the number of pending requests."] # [derive (Copy , Clone , Debug , PartialEq , PartialOrd)] pub struct Cost (f64) ;
};
}
