// Generated macro for impl_64 (impl)
macro_rules! Depcrate_originimpl_64 {
() => {
// Module: crate::origin
// Provides: {"impl_64"}
// Dependencies: {}
impl Origin { # [doc = " Creates a new opaque origin that is only equal to itself."] pub fn new_opaque () -> Self { static COUNTER : AtomicUsize = AtomicUsize :: new (0) ; Self :: Opaque (OpaqueOrigin (COUNTER . fetch_add (1 , Ordering :: SeqCst))) } # [doc = " Return whether this origin is a (scheme, host, port) tuple"] # [doc = " (as opposed to an opaque origin)."] pub fn is_tuple (& self) -> bool { matches ! (* self , Self :: Tuple (..)) } # [doc = " <https://html.spec.whatwg.org/multipage/#ascii-serialisation-of-an-origin>"] pub fn ascii_serialization (& self) -> String { match * self { Self :: Opaque (_) => "null" . to_owned () , Self :: Tuple (ref scheme , ref host , port) => { if default_port (scheme) == Some (port) { format ! ("{scheme}://{host}") } else { format ! ("{scheme}://{host}:{port}") } } } } # [doc = " <https://html.spec.whatwg.org/multipage/#unicode-serialisation-of-an-origin>"] pub fn unicode_serialization (& self) -> String { match * self { Self :: Opaque (_) => "null" . to_owned () , Self :: Tuple (ref scheme , ref host , port) => { let host = match * host { Host :: Domain (ref domain) => { let (domain , _errors) = idna :: domain_to_unicode (domain) ; Host :: Domain (domain) } _ => host . clone () , } ; if default_port (scheme) == Some (port) { format ! ("{scheme}://{host}") } else { format ! ("{scheme}://{host}:{port}") } } } } }
};
}
