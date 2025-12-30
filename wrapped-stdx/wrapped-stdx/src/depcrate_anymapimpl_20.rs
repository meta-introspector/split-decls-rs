// Generated macro for impl_20 (impl)
macro_rules! Depcrate_anymapimpl_20 {
() => {
// Module: crate::anymap
// Provides: {"impl_20"}
// Dependencies: {}
impl < A : ? Sized + Downcast > Map < A > { # [doc = " Returns a reference to the value stored in the collection for the type `T`,"] # [doc = " if it exists."] # [inline] # [must_use] pub fn get < T : IntoBox < A > > (& self) -> Option < & T > { self . raw . get (& TypeId :: of :: < T > ()) . map (| any | unsafe { any . downcast_unchecked_ref :: < T > () }) } # [doc = " Gets the entry for the given type in the collection for in-place manipulation"] # [inline] pub fn entry < T : IntoBox < A > > (& mut self) -> Entry < '_ , A , T > { match self . raw . entry (TypeId :: of :: < T > ()) { hash_map :: Entry :: Occupied (e) => { Entry :: Occupied (OccupiedEntry { inner : e , type_ : PhantomData }) } hash_map :: Entry :: Vacant (e) => { Entry :: Vacant (VacantEntry { inner : e , type_ : PhantomData }) } } } }
};
}
