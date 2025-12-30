// Generated macro for BufferLayer (struct)
macro_rules! Depcrate_buffer_layerBufferLayer {
() => {
// Module: crate::buffer::layer
// Provides: {"BufferLayer"}
// Dependencies: {}
# [doc = " Adds an mpsc buffer in front of an inner service."] # [doc = ""] # [doc = " The default Tokio executor is used to run the given service,"] # [doc = " which means that this layer can only be used on the Tokio runtime."] # [doc = ""] # [doc = " See the module documentation for more details."] pub struct BufferLayer < Request > { bound : usize , _p : PhantomData < fn (Request) > , }
};
}
