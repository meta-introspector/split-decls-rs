// Generated macro for FormatContext (struct)
macro_rules! Depcrate_formattingFormatContext {
() => {
// Module: crate::formatting
// Provides: {"FormatContext"}
// Dependencies: {}
struct FormatContext < 'a , T : FormatHandler > { krate : & 'a ast :: Crate , report : FormatReport , psess : ParseSess , config : & 'a Config , handler : & 'a mut T , }
};
}
