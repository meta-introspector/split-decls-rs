macro_rules! macro_5 {
    () => {
        scoped_thread_local ! (static TLV : Cell <* const () >) ;
    };
}

macro_5!();