// Generated macro for impl_171 (impl)
macro_rules! Depcrate_oneshot_registrarimpl_171 {
() => {
// Module: crate::oneshot::registrar
// Provides: {"impl_171"}
// Dependencies: {}
impl < N , CODEC > OneshotRegistrar < N , CODEC > where N : Oneshot + 'static , CODEC : Codec + 'static , { # [doc = " Creates a new Oneshot Registrar."] pub fn new () -> Self { Self { inner : OneshotWorker :: < N > :: registrar () . encoding :: < CODEC > () , } } # [doc = " Sets the encoding."] pub fn encoding < C > (& self) -> OneshotRegistrar < N , C > where C : Codec + 'static , { OneshotRegistrar { inner : self . inner . encoding :: < C > () , } } # [doc = " Registers the worker."] pub fn register (& self) where N :: Input : Serialize + for < 'de > Deserialize < 'de > , N :: Output : Serialize + for < 'de > Deserialize < 'de > , { self . inner . register () } }
};
}
