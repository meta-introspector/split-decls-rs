// Generated macro for LoadShed (struct)
macro_rules! Depcrate_load_shedLoadShed {
() => {
// Module: crate::load_shed
// Provides: {"LoadShed"}
// Dependencies: {}
# [doc = " A [`Service`] that sheds load when the inner service isn't ready."] # [doc = ""] # [doc = " [`Service`]: crate::Service"] # [derive (Debug)] pub struct LoadShed < S > { inner : S , is_ready : bool , }
};
}
