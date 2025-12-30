// Generated macro for impl_73 (impl)
macro_rules! Depcrate_ansiimpl_73 {
() => {
// Module: crate::ansi
// Provides: {"impl_73"}
// Dependencies: {}
impl PrivateMode { fn new (mode : u16) -> Self { match mode { 1 => Self :: Named (NamedPrivateMode :: CursorKeys) , 3 => Self :: Named (NamedPrivateMode :: ColumnMode) , 6 => Self :: Named (NamedPrivateMode :: Origin) , 7 => Self :: Named (NamedPrivateMode :: LineWrap) , 12 => Self :: Named (NamedPrivateMode :: BlinkingCursor) , 25 => Self :: Named (NamedPrivateMode :: ShowCursor) , 1000 => Self :: Named (NamedPrivateMode :: ReportMouseClicks) , 1002 => Self :: Named (NamedPrivateMode :: ReportCellMouseMotion) , 1003 => Self :: Named (NamedPrivateMode :: ReportAllMouseMotion) , 1004 => Self :: Named (NamedPrivateMode :: ReportFocusInOut) , 1005 => Self :: Named (NamedPrivateMode :: Utf8Mouse) , 1006 => Self :: Named (NamedPrivateMode :: SgrMouse) , 1007 => Self :: Named (NamedPrivateMode :: AlternateScroll) , 1042 => Self :: Named (NamedPrivateMode :: UrgencyHints) , 1049 => Self :: Named (NamedPrivateMode :: SwapScreenAndSetRestoreCursor) , 2004 => Self :: Named (NamedPrivateMode :: BracketedPaste) , 2026 => Self :: Named (NamedPrivateMode :: SyncUpdate) , _ => Self :: Unknown (mode) , } } # [doc = " Get the raw value of the mode."] pub fn raw (self) -> u16 { match self { Self :: Named (named) => named as u16 , Self :: Unknown (mode) => mode , } } }
};
}
