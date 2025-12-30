// Generated macro for internal (module)
macro_rules! Depcrateinternal {
() => {
// Module: crate
// Provides: {"internal"}
// Dependencies: {}
# [doc = " Internal classes that are used in integration tests."] # [doc = " The contents of this section DO NOT form part of the stable interface."] # [doc (hidden)] pub mod internal { # [doc = " Low-level TLS message parsing and encoding functions."] pub mod msgs { pub mod codec { pub use crate :: msgs :: codec :: { Codec , Reader } ; } pub mod enums { pub use crate :: msgs :: enums :: { AlertLevel , ExtensionType } ; } pub mod fragmenter { pub use crate :: msgs :: fragmenter :: MessageFragmenter ; } pub mod message { pub use crate :: msgs :: message :: { Message , MessagePayload } ; } pub mod persist { pub use crate :: msgs :: persist :: ServerSessionValue ; } } pub mod fuzzing { pub use crate :: msgs :: deframer :: fuzz_deframer ; } }
};
}
