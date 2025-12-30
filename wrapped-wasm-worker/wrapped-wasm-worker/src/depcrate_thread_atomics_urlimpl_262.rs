// Generated macro for impl_262 (impl)
macro_rules! Depcrate_thread_atomics_urlimpl_262 {
() => {
// Module: crate::thread::atomics::url
// Provides: {"impl_262"}
// Dependencies: {}
impl ScriptUrl { # [doc = " Creates a new [`Url`]."] pub (super) fn new (script : & str) -> Self { let sequence = Array :: of1 (& script . into ()) ; let property = BlobPropertyBag :: new () ; property . set_type ("text/javascript") ; let blob = Blob :: new_with_str_sequence_and_options (& sequence , & property) . expect ("`new Blob()` should never throw") ; let url = Url :: create_object_url_with_blob (& blob) . expect ("`URL.createObjectURL()` should never throw") ; Self (url) } # [doc = " Returns the object URL."] # [must_use] pub (super) fn as_raw (& self) -> & str { & self . 0 } }
};
}
