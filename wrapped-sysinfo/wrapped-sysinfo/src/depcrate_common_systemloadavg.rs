// Generated macro for LoadAvg (struct)
macro_rules! Depcrate_common_systemLoadAvg {
() => {
// Module: crate::common::system
// Provides: {"LoadAvg"}
// Dependencies: {}
# [doc = " A struct representing system load average value."] # [doc = ""] # [doc = " It is returned by [`System::load_average`][crate::System::load_average]."] # [doc = ""] # [doc = " ```no_run"] # [doc = " use sysinfo::System;"] # [doc = ""] # [doc = " let load_avg = System::load_average();"] # [doc = " println!("] # [doc = "     \"one minute: {}%, five minutes: {}%, fifteen minutes: {}%\","] # [doc = "     load_avg.one,"] # [doc = "     load_avg.five,"] # [doc = "     load_avg.fifteen,"] # [doc = " );"] # [doc = " ```"] # [repr (C)] # [derive (Default , Debug , Clone)] pub struct LoadAvg { # [doc = " Average load within one minute."] pub one : f64 , # [doc = " Average load within five minutes."] pub five : f64 , # [doc = " Average load within fifteen minutes."] pub fifteen : f64 , }
};
}
