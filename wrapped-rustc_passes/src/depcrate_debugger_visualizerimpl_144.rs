// Generated macro for impl_144 (impl)
macro_rules! Depcrate_debugger_visualizerimpl_144 {
() => {
// Module: crate::debugger_visualizer
// Provides: {"impl_144"}
// Dependencies: {}
impl DebuggerVisualizerCollector < '_ > { fn check_for_debugger_visualizer (& mut self , attr : & Attribute) { if attr . has_name (sym :: debugger_visualizer) { let Some (hints) = attr . meta_item_list () else { self . sess . dcx () . emit_err (DebugVisualizerInvalid { span : attr . span }) ; return ; } ; let [hint] = hints . as_slice () else { self . sess . dcx () . emit_err (DebugVisualizerInvalid { span : attr . span }) ; return ; } ; let Some (meta_item) = hint . meta_item () else { self . sess . dcx () . emit_err (DebugVisualizerInvalid { span : attr . span }) ; return ; } ; let (visualizer_type , visualizer_path) = match (meta_item . name () , meta_item . value_str ()) { (Some (sym :: natvis_file) , Some (value)) => (DebuggerVisualizerType :: Natvis , value) , (Some (sym :: gdb_script_file) , Some (value)) => { (DebuggerVisualizerType :: GdbPrettyPrinter , value) } (_ , _) => { self . sess . dcx () . emit_err (DebugVisualizerInvalid { span : meta_item . span }) ; return ; } } ; let file = match resolve_path (& self . sess , visualizer_path . as_str () , attr . span) { Ok (file) => file , Err (err) => { err . emit () ; return ; } } ; match self . sess . source_map () . load_binary_file (& file) { Ok ((source , _)) => { self . visualizers . push (DebuggerVisualizerFile :: new (source , visualizer_type , file ,)) ; } Err (error) => { self . sess . dcx () . emit_err (DebugVisualizerUnreadable { span : meta_item . span , file : & file , error , }) ; } } } } }
};
}
