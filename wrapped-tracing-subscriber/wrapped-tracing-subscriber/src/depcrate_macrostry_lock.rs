// Generated macro for try_lock (macro)
macro_rules! Depcrate_macrostry_lock {
() => {
// Module: crate::macros
// Provides: {"try_lock"}
// Dependencies: {}
# [cfg (feature = "std")] macro_rules ! try_lock { ($ lock : expr) => { try_lock ! ($ lock , else return) } ; ($ lock : expr , else $ els : expr) => { if let :: core :: result :: Result :: Ok (l) = $ lock { l } else if std :: thread :: panicking () { $ els } else { panic ! ("lock poisoned") } } ; }
};
}
