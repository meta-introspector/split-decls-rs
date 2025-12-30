// Generated macro for catch_unwind (function)
macro_rules! Depcrate_panickingcatch_unwind {
() => {
// Module: crate::panicking
// Provides: {"catch_unwind"}
// Dependencies: {}
# [doc = " Invoke a closure, capturing the cause of an unwinding panic if one occurs."] # [cfg (not (feature = "panic_immediate_abort"))] pub unsafe fn catch_unwind < R , F : FnOnce () -> R > (f : F) -> Result < R , Box < dyn Any + Send > > { union Data < F , R > { f : ManuallyDrop < F > , r : ManuallyDrop < R > , p : ManuallyDrop < Box < dyn Any + Send > > , } let mut data = Data { f : ManuallyDrop :: new (f) } ; let data_ptr = (& raw mut data) as * mut u8 ; unsafe { return if intrinsics :: catch_unwind (do_call :: < F , R > , data_ptr , do_catch :: < F , R >) == 0 { Ok (ManuallyDrop :: into_inner (data . r)) } else { Err (ManuallyDrop :: into_inner (data . p)) } ; } # [cold] # [optimize (size)] unsafe fn cleanup (payload : * mut u8) -> Box < dyn Any + Send + 'static > { let obj = unsafe { Box :: from_raw (__rust_panic_cleanup (payload)) } ; panic_count :: decrease () ; obj } # [inline] fn do_call < F : FnOnce () -> R , R > (data : * mut u8) { unsafe { let data = data as * mut Data < F , R > ; let data = & mut (* data) ; let f = ManuallyDrop :: take (& mut data . f) ; data . r = ManuallyDrop :: new (f ()) ; } } # [inline] # [rustc_nounwind] fn do_catch < F : FnOnce () -> R , R > (data : * mut u8 , payload : * mut u8) { unsafe { let data = data as * mut Data < F , R > ; let data = & mut (* data) ; let obj = cleanup (payload) ; data . p = ManuallyDrop :: new (obj) ; } } }
};
}
