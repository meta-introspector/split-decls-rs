// Generated macro for DeconstructedPat (struct)
macro_rules! Depcrate_patDeconstructedPat {
() => {
// Module: crate::pat
// Provides: {"DeconstructedPat"}
// Dependencies: {}
# [doc = " Values and patterns can be represented as a constructor applied to some fields. This represents"] # [doc = " a pattern in this form. A `DeconstructedPat` will almost always come from user input; the only"] # [doc = " exception are some `Wildcard`s introduced during pattern lowering."] pub struct DeconstructedPat < Cx : PatCx > { ctor : Constructor < Cx > , fields : Vec < IndexedPat < Cx > > , # [doc = " The number of fields in this pattern. E.g. if the pattern is `SomeStruct { field12: true, .."] # [doc = " }` this would be the total number of fields of the struct."] # [doc = " This is also the same as `self.ctor.arity(self.ty)`."] arity : usize , ty : Cx :: Ty , # [doc = " Extra data to store in a pattern."] data : Cx :: PatData , # [doc = " Globally-unique id used to track usefulness at the level of subpatterns."] pub (crate) uid : PatId , }
};
}
