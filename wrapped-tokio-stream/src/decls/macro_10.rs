macro_rules! macro_10 {
    () => {
        cfg_sync ! { mod broadcast ; pub use broadcast :: BroadcastStream ; mod watch ; pub use watch :: WatchStream ; }
    };
}

macro_10!()