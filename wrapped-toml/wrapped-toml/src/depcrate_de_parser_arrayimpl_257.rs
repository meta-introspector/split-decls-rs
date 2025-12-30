// Generated macro for impl_257 (impl)
macro_rules! Depcrate_de_parser_arrayimpl_257 {
() => {
// Module: crate::de::parser::array
// Provides: {"impl_257"}
// Dependencies: {}
impl < 'i > State < 'i > { fn open (& mut self , _open_event : & toml_parser :: parser :: Event) { } fn whitespace (& mut self , _event : & toml_parser :: parser :: Event) { } fn capture_value (& mut self , _event : & toml_parser :: parser :: Event , value : Spanned < DeValue < 'i > >) { self . trailing_start = None ; self . current_value = Some (value) ; } fn finish_value (& mut self , _event : & toml_parser :: parser :: Event , result : & mut DeArray < 'i >) { # [cfg (feature = "debug")] let _scope = TraceScope :: new ("array::finish_value") ; if let Some (value) = self . current_value . take () { result . push (value) ; } } fn sep_value (& mut self , event : & toml_parser :: parser :: Event) { self . trailing_start = Some (event . span () . end ()) ; } fn close (& mut self , _open_event : & toml_parser :: parser :: Event , _close_event : & toml_parser :: parser :: Event , _result : & mut DeArray < 'i > ,) { # [cfg (feature = "debug")] let _scope = TraceScope :: new ("array::close") ; } }
};
}
