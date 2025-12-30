// Generated macro for Rotation (struct)
macro_rules! Depcrate_rollingRotation {
() => {
// Module: crate::rolling
// Provides: {"Rotation"}
// Dependencies: {}
# [doc = " Defines a fixed period for rolling of a log file."] # [doc = ""] # [doc = " To use a `Rotation`, pick one of the following options:"] # [doc = ""] # [doc = " ### Minutely Rotation"] # [doc = " ```rust"] # [doc = " # fn docs() {"] # [doc = " use tracing_appender::rolling::Rotation;"] # [doc = " let rotation = tracing_appender::rolling::Rotation::MINUTELY;"] # [doc = " # }"] # [doc = " ```"] # [doc = ""] # [doc = " ### Hourly Rotation"] # [doc = " ```rust"] # [doc = " # fn docs() {"] # [doc = " use tracing_appender::rolling::Rotation;"] # [doc = " let rotation = tracing_appender::rolling::Rotation::HOURLY;"] # [doc = " # }"] # [doc = " ```"] # [doc = ""] # [doc = " ### Daily Rotation"] # [doc = " ```rust"] # [doc = " # fn docs() {"] # [doc = " use tracing_appender::rolling::Rotation;"] # [doc = " let rotation = tracing_appender::rolling::Rotation::DAILY;"] # [doc = " # }"] # [doc = " ```"] # [doc = ""] # [doc = " ### Weekly Rotation"] # [doc = " ```rust"] # [doc = " # fn docs() {"] # [doc = " use tracing_appender::rolling::Rotation;"] # [doc = " let rotation = tracing_appender::rolling::Rotation::WEEKLY;"] # [doc = " # }"] # [doc = " ```"] # [doc = ""] # [doc = " ### No Rotation"] # [doc = " ```rust"] # [doc = " # fn docs() {"] # [doc = " use tracing_appender::rolling::Rotation;"] # [doc = " let rotation = tracing_appender::rolling::Rotation::NEVER;"] # [doc = " # }"] # [doc = " ```"] # [derive (Clone , Eq , PartialEq , Debug)] pub struct Rotation (RotationKind) ;
};
}
