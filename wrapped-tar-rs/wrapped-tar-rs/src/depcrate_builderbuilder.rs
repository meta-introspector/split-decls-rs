// Generated macro for Builder (struct)
macro_rules! Depcrate_builderBuilder {
() => {
// Module: crate::builder
// Provides: {"Builder"}
// Dependencies: {}
# [doc = " A structure for building archives"] # [doc = ""] # [doc = " This structure has methods for building up an archive from scratch into any"] # [doc = " arbitrary writer."] pub struct Builder < W : Write > { options : BuilderOptions , finished : bool , obj : Option < W > , }
};
}
