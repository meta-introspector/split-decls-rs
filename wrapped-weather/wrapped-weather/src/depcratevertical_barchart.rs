// Generated macro for vertical_barchart (function)
macro_rules! Depcratevertical_barchart {
() => {
// Module: crate
// Provides: {"vertical_barchart"}
// Dependencies: {}
# [doc = " Create a vertical bar chart from the temperatures data."] fn vertical_barchart (temperatures : & [u8]) -> BarChart < '_ > { let bars : Vec < Bar > = temperatures . iter () . enumerate () . map (| (hour , value) | vertical_bar (hour , value)) . collect () ; BarChart :: default () . data (BarGroup :: default () . bars (& bars)) . bar_width (5) }
};
}
