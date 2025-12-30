// Generated macro for impl_1306 (impl)
macro_rules! Depcrate_mock_commandimpl_1306 {
() => {
// Module: crate::mock_command
// Provides: {"impl_1306"}
// Dependencies: {}
# [async_trait] impl RunCommand for MockCommand { type C = MockChild ; fn arg < S : AsRef < OsStr > > (& mut self , arg : S) -> & mut MockCommand { self . args . push (arg . as_ref () . to_owned ()) ; self } fn args < S : AsRef < OsStr > > (& mut self , args : & [S]) -> & mut MockCommand { self . args . extend (args . iter () . map (| a | a . as_ref () . to_owned ())) ; self } fn env < K , V > (& mut self , _key : K , _val : V) -> & mut MockCommand where K : AsRef < OsStr > , V : AsRef < OsStr > , { self } fn envs < I , K , V > (& mut self , _vars : I) -> & mut Self where I : IntoIterator < Item = (K , V) > , K : AsRef < OsStr > , V : AsRef < OsStr > , { self } fn env_clear (& mut self) -> & mut MockCommand { self } fn current_dir < P : AsRef < Path > > (& mut self , _dir : P) -> & mut MockCommand { self } fn stdin (& mut self , _cfg : Stdio) -> & mut MockCommand { self } fn stdout (& mut self , _cfg : Stdio) -> & mut MockCommand { self } fn stderr (& mut self , _cfg : Stdio) -> & mut MockCommand { self } async fn spawn (& mut self) -> Result < MockChild > { match self . child . take () . unwrap () { ChildOrCall :: Child (c) => c , ChildOrCall :: Call (f) => f (& self . args) , } } }
};
}
