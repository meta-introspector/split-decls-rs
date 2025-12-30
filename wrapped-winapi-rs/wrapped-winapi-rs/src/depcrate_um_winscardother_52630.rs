// Generated macro for other_52630 (other)
macro_rules! Depcrate_um_winscardother_52630 {
() => {
// Module: crate::um::winscard
// Provides: {"other_52630"}
// Dependencies: {}
extern "system" { pub fn SCardLocateCardsA (hContext : SCARDCONTEXT , mszCards : LPCSTR , rgReaderStates : LPSCARD_READERSTATEA , cReaders : DWORD ,) -> LONG ; pub fn SCardLocateCardsW (hContext : SCARDCONTEXT , mszCards : LPCWSTR , rgReaderStates : LPSCARD_READERSTATEW , cReaders : DWORD ,) -> LONG ; }
};
}
