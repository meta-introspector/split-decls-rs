// Generated macro for other_58430 (other)
macro_rules! Depcrate_winrt_winstringother_58430 {
() => {
// Module: crate::winrt::winstring
// Provides: {"other_58430"}
// Dependencies: {}
extern "system" { pub fn WindowsInspectString (targetHString : UINT_PTR , machine : USHORT , callback : PINSPECT_HSTRING_CALLBACK , context : * const VOID , length : * mut UINT32 , targetStringAddress : * mut UINT_PTR ,) -> HRESULT ; pub fn HSTRING_UserSize (pFlags : * const ULONG , StartingSize : ULONG , ppidl : * const HSTRING ,) -> ULONG ; pub fn HSTRING_UserMarshal (pFlags : * const ULONG , pBuffer : * mut UCHAR , ppidl : * const HSTRING ,) -> * mut UCHAR ; pub fn HSTRING_UserUnmarshal (pFlags : * const ULONG , pBuffer : * const UCHAR , ppidl : * mut HSTRING ,) -> * mut UCHAR ; pub fn HSTRING_UserFree (pFlags : * const ULONG , ppidl : * const HSTRING ,) ; # [cfg (target_arch = "x86_64")] pub fn HSTRING_UserSize64 (pFlags : * const ULONG , StartingSize : ULONG , ppidl : * const HSTRING ,) -> ULONG ; # [cfg (target_arch = "x86_64")] pub fn HSTRING_UserMarshal64 (pFlags : * const ULONG , pBuffer : * mut UCHAR , ppidl : * const HSTRING ,) -> * mut UCHAR ; # [cfg (target_arch = "x86_64")] pub fn HSTRING_UserUnmarshal64 (pFlags : * const ULONG , pBuffer : * const UCHAR , ppidl : * mut HSTRING ,) -> * mut UCHAR ; # [cfg (target_arch = "x86_64")] pub fn HSTRING_UserFree64 (pFlags : * const ULONG , ppidl : * const HSTRING ,) ; }
};
}
