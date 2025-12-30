// Generated macro for Harness (struct)
macro_rules! DepcrateHarness {
() => {
// Module: crate
// Provides: {"Harness"}
// Dependencies: {}
# [doc = " [`Harness`] for discovering test inputs and asserting against snapshot files"] pub struct Harness < S , T , I , E > { root : std :: path :: PathBuf , overrides : Option < ignore :: overrides :: Override > , setup : S , test : T , config : snapbox :: Assert , test_output : std :: marker :: PhantomData < I > , test_error : std :: marker :: PhantomData < E > , }
};
}
