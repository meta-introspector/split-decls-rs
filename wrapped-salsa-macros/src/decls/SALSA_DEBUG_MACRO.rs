macro_rules! SALSA_DEBUG_MACRO {
    () => {
        static SALSA_DEBUG_MACRO : OnceLock < Option < String > > = OnceLock :: new () ;
    };
}

SALSA_DEBUG_MACRO!()