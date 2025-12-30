// Generated macro for impl_120 (impl)
macro_rules! Depcrate_errorimpl_120 {
() => {
// Module: crate::error
// Provides: {"impl_120"}
// Dependencies: {}
# [cfg (feature = "std")] impl < I , C > TreeError < I , C > { # [doc = " Translate the input type"] pub fn map_input < I2 , O : Clone + Fn (I) -> I2 > (self , op : O) -> TreeError < I2 , C > { match self { TreeError :: Base (base) => TreeError :: Base (TreeErrorBase { input : op (base . input) , cause : base . cause , }) , TreeError :: Stack { base , stack } => { let base = Box :: new (base . map_input (op . clone ())) ; let stack = stack . into_iter () . map (| frame | match frame { TreeErrorFrame :: Kind (kind) => TreeErrorFrame :: Kind (TreeErrorBase { input : op (kind . input) , cause : kind . cause , }) , TreeErrorFrame :: Context (context) => { TreeErrorFrame :: Context (TreeErrorContext { input : op (context . input) , context : context . context , }) } }) . collect () ; TreeError :: Stack { base , stack } } TreeError :: Alt (alt) => { TreeError :: Alt (alt . into_iter () . map (| e | e . map_input (op . clone ())) . collect ()) } } } fn append_frame (self , frame : TreeErrorFrame < I , C >) -> Self { match self { TreeError :: Stack { base , mut stack } => { stack . push (frame) ; TreeError :: Stack { base , stack } } base => TreeError :: Stack { base : Box :: new (base) , stack : vec ! [frame] , } , } } }
};
}
