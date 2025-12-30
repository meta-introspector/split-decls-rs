// Generated macro for traits (module)
macro_rules! Depcratetraits {
() => {
// Module: crate
// Provides: {"traits"}
// Dependencies: {}
# [doc = " Bring traits from this crate, `tracing`, and `tracing_subscriber` into scope"] # [doc = " anonymously."] pub mod traits { pub use crate :: Processor as _ ; pub use tracing :: Instrument as _ ; pub use tracing_subscriber :: { layer :: SubscriberExt as _ , util :: SubscriberInitExt as _ } ; }
};
}
