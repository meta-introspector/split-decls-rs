// Generated macro for impl_20 (impl)
macro_rules! Depcrate_executor_futures_03impl_20 {
() => {
// Module: crate::executor::futures_03
// Provides: {"impl_20"}
// Dependencies: {}
impl < T > Spawn for Instrumented < T > where T : Spawn , { # [doc = " Spawns a future that will be run to completion."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " The executor may be unable to spawn tasks. Spawn errors should"] # [doc = " represent relatively rare scenarios, such as the executor"] # [doc = " having been shut down so that it is no longer able to accept"] # [doc = " tasks."] fn spawn_obj (& self , future : FutureObj < 'static , () >) -> Result < () , SpawnError > { let future = future . instrument (self . span . clone ()) ; self . inner . spawn_obj (FutureObj :: new (Box :: new (future))) } # [doc = " Determines whether the executor is able to spawn new tasks."] # [doc = ""] # [doc = " This method will return `Ok` when the executor is *likely*"] # [doc = " (but not guaranteed) to accept a subsequent spawn attempt."] # [doc = " Likewise, an `Err` return means that `spawn` is likely, but"] # [doc = " not guaranteed, to yield an error."] # [inline] fn status (& self) -> Result < () , SpawnError > { self . inner . status () } }
};
}
