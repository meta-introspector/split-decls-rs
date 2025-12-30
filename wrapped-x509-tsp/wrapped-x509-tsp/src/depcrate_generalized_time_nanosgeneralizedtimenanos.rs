// Generated macro for GeneralizedTimeNanos (struct)
macro_rules! Depcrate_generalized_time_nanosGeneralizedTimeNanos {
() => {
// Module: crate::generalized_time_nanos
// Provides: {"GeneralizedTimeNanos"}
// Dependencies: {}
# [doc = " ASN.1 `GeneralizedTime` type."] # [doc = ""] # [doc = " This type implements the validity requirements specified in"] # [doc = " X.690 DER encoding of GeneralizedTime:"] # [doc = ""] # [doc = " > 11.7.1 - The encoding shall terminate with a \"Z\""] # [doc = " > 11.7.2 - The seconds element shall always be present."] # [doc = " > 11.7.3 - The fractional-seconds elements, if present, shall omit all"] # [doc = " >          trailing zeros; if the elements correspond to 0, they shall be wholly"] # [doc = " >          omitted, and the decimal point element also shall be omitted"] # [doc = " > 11.7.4 - The decimal point element, if present, shall be the point option \".\"."] # [doc = " > 11.7.5 - Midnight (GMT) shall be represented in the form `YYYYMMDD000000Z`"] # [doc = " >          where `YYYYMMDD` represents the day following the midnight in question"] # [derive (Copy , Clone , Debug , Eq , PartialEq , PartialOrd , Ord)] pub struct GeneralizedTimeNanos { datetime : DateTime , # [doc = " Nanoseconds (0-999 999 999)"] nanoseconds : u32 , }
};
}
