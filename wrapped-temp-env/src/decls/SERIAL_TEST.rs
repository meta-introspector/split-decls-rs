macro_rules! SERIAL_TEST {
    () => {
        # [doc = " Make sure that the environment isn't modified concurrently."] static SERIAL_TEST : ReentrantMutex < () > = ReentrantMutex :: new (()) ;
    };
}

SERIAL_TEST!();