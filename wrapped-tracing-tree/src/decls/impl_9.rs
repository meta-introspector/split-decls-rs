macro_rules! deps {
    () => {
        Config!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl Config { pub fn with_ansi (self , ansi : bool) -> Self { Self { ansi , .. self } } pub fn with_indent_lines (self , indent_lines : bool) -> Self { Self { indent_lines , .. self } } pub fn with_targets (self , targets : bool) -> Self { Self { targets , .. self } } pub fn with_thread_ids (self , render_thread_ids : bool) -> Self { Self { render_thread_ids , .. self } } pub fn with_thread_names (self , render_thread_names : bool) -> Self { Self { render_thread_names , .. self } } pub fn with_wraparound (self , wraparound : usize) -> Self { Self { wraparound , .. self } } pub fn with_verbose_entry (self , verbose_entry : bool) -> Self { Self { verbose_entry , .. self } } pub fn with_verbose_exit (self , verbose_exit : bool) -> Self { Self { verbose_exit , .. self } } pub fn with_span_retrace (self , enabled : bool) -> Self { Self { span_retrace : enabled , .. self } } pub fn with_deferred_spans (self , enable : bool) -> Self { Self { deferred_spans : enable , .. self } } pub fn with_span_modes (self , enable : bool) -> Self { Self { span_modes : enable , .. self } } pub fn with_bracketed_fields (self , bracketed_fields : bool) -> Self { Self { bracketed_fields , .. self } } pub (crate) fn prefix (& self) -> String { let mut buf = String :: new () ; if self . render_thread_ids { write ! (buf , "{:?}" , std :: thread :: current () . id ()) . unwrap () ; if buf . ends_with (')') { buf . truncate (buf . len () - 1) ; } if buf . starts_with ("ThreadId(") { buf . drain (0 .. "ThreadId(" . len ()) ; } } if self . render_thread_names { if let Some (name) = std :: thread :: current () . name () { if self . render_thread_ids { buf . push (':') ; } buf . push_str (name) ; } } buf } }
    };
}

impl_9!();