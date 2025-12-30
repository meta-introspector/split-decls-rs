// Generated macro for impl_73 (impl)
macro_rules! Depcrate_searcherimpl_73 {
() => {
// Module: crate::searcher
// Provides: {"impl_73"}
// Dependencies: {}
impl Config { # [doc = " Return the maximal amount of lines needed to fulfill this"] # [doc = " configuration's context."] # [doc = ""] # [doc = " If this returns `0`, then no context is ever needed."] fn max_context (& self) -> usize { cmp :: max (self . before_context , self . after_context) } # [doc = " Build a line buffer from this configuration."] fn line_buffer (& self) -> LineBuffer { let mut builder = LineBufferBuilder :: new () ; builder . line_terminator (self . line_term . as_byte ()) . binary_detection (self . binary . 0) ; if let Some (limit) = self . heap_limit { let (capacity , additional) = if limit <= DEFAULT_BUFFER_CAPACITY { (limit , 0) } else { (DEFAULT_BUFFER_CAPACITY , limit - DEFAULT_BUFFER_CAPACITY) } ; builder . capacity (capacity) . buffer_alloc (BufferAllocation :: Error (additional)) ; } builder . build () } }
};
}
