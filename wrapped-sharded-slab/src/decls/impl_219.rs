macro_rules! deps {
    () => {
        State!();
        DontDropMe!();
    };
}

macro_rules! impl_219 {
    () => {
        deps!();
        impl DontDropMe { fn new (id : usize) -> (Arc < State > , Self) { let state = Arc :: new (State { is_dropped : AtomicBool :: new (false) , is_cleared : AtomicBool :: new (false) , id , }) ; (state . clone () , Self (state)) } }
    };
}

impl_219!()