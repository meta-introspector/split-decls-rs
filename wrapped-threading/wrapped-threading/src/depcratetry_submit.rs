// Generated macro for try_submit (function)
macro_rules! Depcratetry_submit {
() => {
// Module: crate
// Provides: {"try_submit"}
// Dependencies: {}
unsafe fn try_submit < F : FnOnce () + Send > (environment : * const TP_CALLBACK_ENVIRON_V3 , f : F) { unsafe extern "system" fn callback < F : FnOnce () + Send > (_ : PTP_CALLBACK_INSTANCE , callback : * mut c_void ,) { unsafe { Box :: from_raw (callback as * mut F) () ; } } unsafe { check (TrySubmitThreadpoolCallback (Some (callback :: < F >) , Box :: into_raw (Box :: new (f)) as _ , environment ,)) ; } }
};
}
