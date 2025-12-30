// Generated macro for DropCtxt (struct)
macro_rules! Depcrate_elaborate_dropDropCtxt {
() => {
// Module: crate::elaborate_drop
// Provides: {"DropCtxt"}
// Dependencies: {}
# [derive (Debug)] struct DropCtxt < 'a , 'b , 'tcx , D > where D : DropElaborator < 'b , 'tcx > , { elaborator : & 'a mut D , source_info : SourceInfo , place : Place < 'tcx > , path : D :: Path , succ : BasicBlock , unwind : Unwind , dropline : Option < BasicBlock > , }
};
}
