macro_rules! entry_fn {
    () => {
        # [doc = " Return the function where execution starts if the current"] # [doc = " crate defines that. This is usually `main`, but could be"] # [doc = " `start` if the crate is a no-std crate."] pub fn entry_fn () -> Option < CrateItem > { with (| cx | cx . entry_fn ()) }
    };
}

entry_fn!();