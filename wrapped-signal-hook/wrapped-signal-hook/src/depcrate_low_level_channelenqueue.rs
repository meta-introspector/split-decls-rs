// Generated macro for enqueue (function)
macro_rules! Depcrate_low_level_channelenqueue {
() => {
// Module: crate::low_level::channel
// Provides: {"enqueue"}
// Dependencies: {}
fn enqueue (q : & AtomicU16 , val : u16) { let mut current = q . load (Ordering :: Relaxed) ; loop { let empty = (0 .. SLOTS as u16) . find (| i | get (current , * i) == 0) . expect ("No empty slot available") ; let modified = set (current , empty , val) ; match q . compare_exchange_weak (current , modified , Ordering :: Release , Ordering :: Relaxed) { Ok (_) => break , Err (changed) => current = changed , } } }
};
}
