// Generated macro for generate_chrono_datetime (function)
macro_rules! Depcrate_readgenerate_chrono_datetime {
() => {
// Module: crate::read
// Provides: {"generate_chrono_datetime"}
// Dependencies: {}
# [cfg (feature = "chrono")] # [doc = " Generate a `NaiveDateTime` from a `DateTime`."] fn generate_chrono_datetime (datetime : & DateTime) -> Option < chrono :: NaiveDateTime > { if let Some (d) = chrono :: NaiveDate :: from_ymd_opt (datetime . year () . into () , datetime . month () . into () , datetime . day () . into () ,) { if let Some (d) = d . and_hms_opt (datetime . hour () . into () , datetime . minute () . into () , datetime . second () . into () ,) { return Some (d) ; } } None }
};
}
