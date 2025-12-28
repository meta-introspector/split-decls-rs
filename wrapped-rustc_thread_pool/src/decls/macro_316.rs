macro_rules! macro_316 {
    () => {
        thread_local ! (pub static TLV : Cell <* const () > = const { Cell :: new (ptr :: null ()) }) ;
    };
}

macro_316!();