macro_rules! deps {
    () => {
        Generation!();
        Pack!();
        Tid!();
        Addr!();
        DefaultConfig!();
    };
}

macro_rules! idx {
    () => {
        deps!();
        mod idx { use crate :: { cfg , page :: { self , slot } , Pack , Tid , } ; use proptest :: prelude :: * ; proptest ! { # [test] # [cfg_attr (loom , ignore)] fn tid_roundtrips (tid in 0usize .. Tid ::< cfg :: DefaultConfig >:: BITS) { let tid = Tid ::< cfg :: DefaultConfig >:: from_usize (tid) ; let packed = tid . pack (0) ; assert_eq ! (tid , Tid :: from_packed (packed)) ; } # [test] # [cfg_attr (loom , ignore)] fn idx_roundtrips (tid in 0usize .. Tid ::< cfg :: DefaultConfig >:: BITS , gen in 0usize .. slot :: Generation ::< cfg :: DefaultConfig >:: BITS , addr in 0usize .. page :: Addr ::< cfg :: DefaultConfig >:: BITS ,) { let tid = Tid ::< cfg :: DefaultConfig >:: from_usize (tid) ; let gen = slot :: Generation ::< cfg :: DefaultConfig >:: from_usize (gen) ; let addr = page :: Addr ::< cfg :: DefaultConfig >:: from_usize (addr) ; let packed = tid . pack (gen . pack (addr . pack (0))) ; assert_eq ! (addr , page :: Addr :: from_packed (packed)) ; assert_eq ! (gen , slot :: Generation :: from_packed (packed)) ; assert_eq ! (tid , Tid :: from_packed (packed)) ; } } }
    };
}

idx!()