// Generated macro for impl_576 (impl)
macro_rules! Depcrate_format_report_formatterimpl_576 {
() => {
// Module: crate::format_report_formatter
// Provides: {"impl_576"}
// Dependencies: {}
impl < 'a > FormatReportFormatterBuilder < 'a > { # [doc = " Creates a new [`FormatReportFormatterBuilder`]."] pub fn new (report : & 'a FormatReport) -> Self { Self { report , enable_colors : false , } } # [doc = " Enables colors and formatting in the output."] # [must_use] pub fn enable_colors (self , enable_colors : bool) -> Self { Self { enable_colors , .. self } } # [doc = " Creates a new [`FormatReportFormatter`] from the settings in this builder."] pub fn build (self) -> FormatReportFormatter < 'a > { FormatReportFormatter { report : self . report , enable_colors : self . enable_colors , } } }
};
}
