// Generated macro for impl_241 (impl)
macro_rules! Depcrate_text_utilsimpl_241 {
() => {
// Module: crate::text::utils
// Provides: {"impl_241"}
// Dependencies: {}
impl < 'a , T : DiffableStrRef + Hash + Eq + ? Sized > QuickSeqRatio < 'a , T > { pub fn new (seq : & [& 'a T]) -> QuickSeqRatio < 'a , T > { let mut counts = HashMap :: new () ; for & word in seq { * counts . entry (word) . or_insert (0) += 1 ; } QuickSeqRatio (counts) } pub fn calc (& self , seq : & [& T]) -> f32 { let n = self . 0 . len () + seq . len () ; if n == 0 { return 1.0 ; } let mut available = HashMap :: new () ; let mut matches = 0 ; for & word in seq { let x = if let Some (count) = available . get (& word) { * count } else { self . 0 . get (& word) . copied () . unwrap_or (0) } ; available . insert (word , x - 1) ; if x > 0 { matches += 1 ; } } 2.0 * matches as f32 / n as f32 } }
};
}
