// Generated macro for SourceFileLines (enum)
macro_rules! DepcrateSourceFileLines {
() => {
// Module: crate
// Provides: {"SourceFileLines"}
// Dependencies: {}
# [derive (Clone)] pub enum SourceFileLines { # [doc = " The source file lines, in decoded (random-access) form."] Lines (Vec < RelativeBytePos >) , # [doc = " The source file lines, in undecoded difference list form."] Diffs (SourceFileDiffs) , }
};
}
