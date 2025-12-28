macro_rules! deps {
    () => {
        Builder!();
        JoinHandle!();
    };
}

macro_rules! spawn {
    () => {
        deps!();
        # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if failed to spawn the thread."] pub fn spawn < F , T > (intent : ThreadIntent , name : String , f : F) -> JoinHandle < T > where F : (FnOnce () -> T) + Send + 'static , T : Send + 'static , { Builder :: new (intent , name) . spawn (f) . expect ("failed to spawn thread") }
    };
}

spawn!();