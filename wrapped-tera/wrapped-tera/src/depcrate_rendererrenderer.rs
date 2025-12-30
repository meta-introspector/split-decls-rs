// Generated macro for Renderer (struct)
macro_rules! Depcrate_rendererRenderer {
() => {
// Module: crate::renderer
// Provides: {"Renderer"}
// Dependencies: {}
# [doc = " Given a `Tera` and reference to `Template` and a `Context`, renders text"] # [derive (Debug)] pub struct Renderer < 'a > { # [doc = " Template to render"] template : & 'a Template , # [doc = " Houses other templates, filters, global functions, etc"] tera : & 'a Tera , # [doc = " Read-only context to be bound to template˝"] context : & 'a Context , # [doc = " If set rendering should be escaped"] should_escape : bool , }
};
}
