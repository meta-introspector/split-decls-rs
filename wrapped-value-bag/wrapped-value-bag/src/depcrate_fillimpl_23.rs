// Generated macro for impl_23 (impl)
macro_rules! Depcrate_fillimpl_23 {
() => {
// Module: crate::fill
// Provides: {"impl_23"}
// Dependencies: {}
impl < 's , 'f > Slot < 's , 'f > { pub (crate) fn new (visitor : & 's mut dyn InternalVisitor < 'f >) -> Self { Slot { visitor } } pub (crate) fn fill < F > (self , f : F) -> Result < () , Error > where F : FnOnce (& mut dyn InternalVisitor < 'f >) -> Result < () , Error > , { f (self . visitor) } # [doc = " Fill the slot with a value."] # [doc = ""] # [doc = " The given value doesn't need to satisfy any particular lifetime constraints."] pub fn fill_any < T > (self , value : T) -> Result < () , Error > where T : Into < ValueBag < 'f > > , { self . fill (| visitor | value . into () . inner . internal_visit (visitor)) } }
};
}
