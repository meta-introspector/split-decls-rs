macro_rules! deps {
    () => {
        Config!();
        Addr!();
    };
}

macro_rules! indices {
    () => {
        deps!();
        # [inline (always)] pub (crate) fn indices < C : cfg :: Config > (idx : usize) -> (Addr < C > , usize) { let addr = C :: unpack_addr (idx) ; (addr , addr . index ()) }
    };
}

indices!();