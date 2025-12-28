macro_rules! must_use {
    () => {
        # [doc = " A function which emits a warning if its return value is not used."] # [must_use] # [inline (always)] pub const fn must_use < T > (t : T) -> T { t }
    };
}

must_use!();