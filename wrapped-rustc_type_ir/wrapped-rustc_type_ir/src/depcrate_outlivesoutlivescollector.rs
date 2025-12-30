// Generated macro for OutlivesCollector (struct)
macro_rules! Depcrate_outlivesOutlivesCollector {
() => {
// Module: crate::outlives
// Provides: {"OutlivesCollector"}
// Dependencies: {}
struct OutlivesCollector < 'a , I : Interner > { cx : I , out : & 'a mut SmallVec < [Component < I > ; 4] > , visited : SsoHashSet < I :: Ty > , }
};
}
