// Generated macro for FileLines (struct)
macro_rules! Depcrate_config_file_linesFileLines {
() => {
// Module: crate::config::file_lines
// Provides: {"FileLines"}
// Dependencies: {}
# [doc = " A set of lines in files."] # [doc = ""] # [doc = " It is represented as a multimap keyed on file names, with values a collection of"] # [doc = " non-overlapping ranges sorted by their start point. An inner `None` is interpreted to mean all"] # [doc = " lines in all files."] # [derive (Clone , Debug , Default , PartialEq)] pub struct FileLines (Option < HashMap < FileName , Vec < Range > > >) ;
};
}
