// Generated macro for impl_73 (impl)
macro_rules! Depcrate_runnerimpl_73 {
() => {
// Module: crate::runner
// Provides: {"impl_73"}
// Dependencies: {}
impl Output { fn sequence (path : std :: path :: PathBuf) -> Self { Self { path , id : None , spawn : Spawn { exit : None , status : SpawnStatus :: Skipped , } , stdout : None , stderr : None , fs : Default :: default () , duration : Default :: default () , } } fn step (path : std :: path :: PathBuf , step : String) -> Self { Self { path , id : Some (step) , spawn : Default :: default () , stdout : None , stderr : None , fs : Default :: default () , duration : Default :: default () , } } fn output (mut self , output : std :: process :: Output) -> Self { self . spawn . exit = Some (output . status) ; assert_eq ! (self . spawn . status , SpawnStatus :: Skipped) ; self . spawn . status = SpawnStatus :: Ok ; self . stdout = Some (Stream { stream : Stdio :: Stdout , content : output . stdout . into_data () , status : StreamStatus :: Ok , }) ; self . stderr = Some (Stream { stream : Stdio :: Stderr , content : output . stderr . into_data () , status : StreamStatus :: Ok , }) ; self } fn error (mut self , msg : crate :: Error) -> Self { self . spawn . status = SpawnStatus :: Failure (msg) ; self } fn duration (mut self , duration : std :: time :: Duration) -> Self { self . duration = Some (duration) ; self } fn is_ok (& self) -> bool { self . spawn . is_ok () && self . stdout . as_ref () . map (| s | s . is_ok ()) . unwrap_or (true) && self . stderr . as_ref () . map (| s | s . is_ok ()) . unwrap_or (true) && self . fs . is_ok () } fn name (& self) -> String { self . id . as_deref () . map (| id | format ! ("{}:{}" , self . path . display () , id)) . unwrap_or_else (| | self . path . display () . to_string ()) } }
};
}
