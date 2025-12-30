// Generated macro for FormatReport (struct)
macro_rules! DepcrateFormatReport {
() => {
// Module: crate
// Provides: {"FormatReport"}
// Dependencies: {}
# [doc = " Reports on any issues that occurred during a run of Rustfmt."] # [doc = ""] # [doc = " Can be reported to the user using the `Display` impl on [`FormatReportFormatter`]."] # [derive (Clone)] pub struct FormatReport { internal : Rc < RefCell < (FormatErrorMap , ReportedErrors) > > , non_formatted_ranges : Vec < (usize , usize) > , }
};
}
