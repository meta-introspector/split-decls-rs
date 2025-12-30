// Generated macro for create_worker (function)
macro_rules! Depcrate_actor_spawnercreate_worker {
() => {
// Module: crate::actor::spawner
// Provides: {"create_worker"}
// Dependencies: {}
fn create_worker (path : & str) -> DedicatedWorker { let js_shim_url = Url :: new_with_base (path , & window () . location () . href () . expect ("failed to read href.") ,) . expect ("failed to create url for javascript entrypoint") . to_string () ; let wasm_url = js_shim_url . replace (".js" , "_bg.wasm") ; let array = Array :: new () ; array . push (& format ! (r#"importScripts("{js_shim_url}");wasm_bindgen("{wasm_url}");"#) . into ()) ; let blob = Blob :: new_with_str_sequence_and_options (& array , BlobPropertyBag :: new () . type_ ("application/javascript") ,) . unwrap () ; let url = Url :: create_object_url_with_blob (& blob) . unwrap () ; DedicatedWorker :: new (& url) . expect ("failed to spawn worker") }
};
}
