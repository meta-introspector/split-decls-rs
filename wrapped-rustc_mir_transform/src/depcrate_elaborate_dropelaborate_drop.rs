// Generated macro for elaborate_drop (function)
macro_rules! Depcrate_elaborate_dropelaborate_drop {
() => {
// Module: crate::elaborate_drop
// Provides: {"elaborate_drop"}
// Dependencies: {}
# [doc = " \"Elaborates\" a drop of `place`/`path` and patches `bb`'s terminator to execute it."] # [doc = ""] # [doc = " The passed `elaborator` is used to determine what should happen at the drop terminator. It"] # [doc = " decides whether the drop can be statically determined or whether it needs a dynamic drop flag,"] # [doc = " and whether the drop is \"open\", ie. should be expanded to drop all subfields of the dropped"] # [doc = " value."] # [doc = ""] # [doc = " When this returns, the MIR patch in the `elaborator` contains the necessary changes."] pub (crate) fn elaborate_drop < 'b , 'tcx , D > (elaborator : & mut D , source_info : SourceInfo , place : Place < 'tcx > , path : D :: Path , succ : BasicBlock , unwind : Unwind , bb : BasicBlock , dropline : Option < BasicBlock > ,) where D : DropElaborator < 'b , 'tcx > , 'tcx : 'b , { DropCtxt { elaborator , source_info , place , path , succ , unwind , dropline } . elaborate_drop (bb) }
};
}
