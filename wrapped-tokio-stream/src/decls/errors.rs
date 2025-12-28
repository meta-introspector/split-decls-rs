macro_rules! errors {
    () => {
        # [doc = " Error types for the wrappers."] pub mod errors { cfg_sync ! { pub use crate :: wrappers :: broadcast :: BroadcastStreamRecvError ; } }
    };
}

errors!();