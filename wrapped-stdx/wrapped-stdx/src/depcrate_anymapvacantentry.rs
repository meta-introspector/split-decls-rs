// Generated macro for VacantEntry (struct)
macro_rules! Depcrate_anymapVacantEntry {
() => {
// Module: crate::anymap
// Provides: {"VacantEntry"}
// Dependencies: {}
# [doc = " A view into a single empty location in an `Map`."] pub struct VacantEntry < 'map , A : ? Sized + Downcast , V : 'map > { inner : hash_map :: VacantEntry < 'map , TypeId , Box < A > > , type_ : PhantomData < V > , }
};
}
