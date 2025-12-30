// Generated macro for HierarchicalLayer (struct)
macro_rules! DepcrateHierarchicalLayer {
() => {
// Module: crate
// Provides: {"HierarchicalLayer"}
// Dependencies: {}
# [derive (Debug)] pub struct HierarchicalLayer < W = fn () -> io :: Stderr , FT = () > where W : for < 'writer > MakeWriter < 'writer > + 'static , FT : FormatTime , { make_writer : W , bufs : Mutex < Buffers > , config : Config , timer : FT , }
};
}
