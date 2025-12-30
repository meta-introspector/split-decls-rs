// Generated macro for impl_236 (impl)
macro_rules! Depcrate_reactor_registrarimpl_236 {
() => {
// Module: crate::reactor::registrar
// Provides: {"impl_236"}
// Dependencies: {}
impl < R , CODEC > ReactorRegistrar < R , CODEC > where R : Reactor + 'static , CODEC : Codec + 'static , { # [doc = " Creates a new reactor registrar."] pub fn new () -> Self { Self { inner : ReactorWorker :: < R > :: registrar () . encoding :: < CODEC > () , } } # [doc = " Sets the encoding."] pub fn encoding < C > (& self) -> ReactorRegistrar < R , C > where C : Codec + 'static , { ReactorRegistrar { inner : self . inner . encoding :: < C > () , } } # [doc = " Registers the worker."] pub fn register (& self) where < R :: Scope as ReactorScoped > :: Input : Serialize + for < 'de > Deserialize < 'de > , < R :: Scope as ReactorScoped > :: Output : Serialize + for < 'de > Deserialize < 'de > , { self . inner . register () } }
};
}
