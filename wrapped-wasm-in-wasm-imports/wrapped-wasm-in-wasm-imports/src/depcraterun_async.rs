// Generated macro for run_async (function)
macro_rules! Depcraterun_async {
() => {
// Module: crate
// Provides: {"run_async"}
// Dependencies: {}
async fn run_async () -> Result < () , JsValue > { console_log ! ("instantiating a new Wasm module directly") ; let imports = make_imports () ? ; let a = JsFuture :: from (WebAssembly :: instantiate_buffer (WASM , & imports)) . await ? ; let instance : WebAssembly :: Instance = Reflect :: get (& a , & "instance" . into ()) ? . dyn_into () ? ; let exports = instance . exports () ; let add = Reflect :: get (& exports , & "add" . into ()) ? . dyn_into :: < Function > () . expect ("add export wasn't a function") ; let three = add . call2 (& JsValue :: undefined () , & 1 . into () , & 2 . into ()) ? ; console_log ! ("1 + 2 = {:?}" , three) ; Ok (()) }
};
}
