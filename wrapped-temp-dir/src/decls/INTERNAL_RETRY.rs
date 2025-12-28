macro_rules! INTERNAL_RETRY {
    () => {
        # [doc (hidden)] pub static INTERNAL_RETRY : AtomicBool = AtomicBool :: new (true) ;
    };
}

INTERNAL_RETRY!();