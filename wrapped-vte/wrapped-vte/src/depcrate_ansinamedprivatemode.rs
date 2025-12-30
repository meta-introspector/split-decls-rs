// Generated macro for NamedPrivateMode (enum)
macro_rules! Depcrate_ansiNamedPrivateMode {
() => {
// Module: crate::ansi
// Provides: {"NamedPrivateMode"}
// Dependencies: {}
# [doc = " Private DEC modes."] # [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum NamedPrivateMode { CursorKeys = 1 , # [doc = " Select 80 or 132 columns per page (DECCOLM)."] # [doc = ""] # [doc = " CSI ? 3 h -> set 132 column font."] # [doc = " CSI ? 3 l -> reset 80 column font."] # [doc = ""] # [doc = " Additionally,"] # [doc = ""] # [doc = " * set margins to default positions"] # [doc = " * erases all data in page memory"] # [doc = " * resets DECLRMM to unavailable"] # [doc = " * clears data from the status line (if set to host-writable)"] ColumnMode = 3 , Origin = 6 , LineWrap = 7 , BlinkingCursor = 12 , ShowCursor = 25 , ReportMouseClicks = 1000 , ReportCellMouseMotion = 1002 , ReportAllMouseMotion = 1003 , ReportFocusInOut = 1004 , Utf8Mouse = 1005 , SgrMouse = 1006 , AlternateScroll = 1007 , UrgencyHints = 1042 , SwapScreenAndSetRestoreCursor = 1049 , BracketedPaste = 2004 , # [doc = " The mode is handled automatically by [`Processor`]."] SyncUpdate = 2026 , }
};
}
