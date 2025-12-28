macro_rules! deps {
    () => {
        Config!();
        SpanMode!();
        Buffers!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl Buffers { pub fn new () -> Self { Self { current_buf : String :: new () , indent_buf : String :: new () , current_span : None , } } pub fn flush_current_buf (& mut self , mut writer : impl io :: Write) { write ! (writer , "{}" , & self . current_buf) . unwrap () ; self . current_buf . clear () ; } pub fn flush_indent_buf (& mut self) { self . current_buf . push_str (& self . indent_buf) ; self . indent_buf . clear () ; } pub (crate) fn indent_current (& mut self , indent : usize , config : & Config , style : SpanMode) { let prefix = config . prefix () ; if config . indent_lines { self . current_buf . push ('\n') ; match style { SpanMode :: Close { .. } | SpanMode :: PostClose => { if indent > 0 && (indent + 1) % config . wraparound == 0 { self . indent_buf . push_str (& prefix) ; for _ in 0 .. (indent % config . wraparound * config . indent_amount) { self . indent_buf . push_str (LINE_HORIZ) ; } self . indent_buf . push_str (LINE_OPEN) ; self . indent_buf . push ('\n') ; } } _ => { } } } indent_block (& self . current_buf , & mut self . indent_buf , indent % config . wraparound , config . indent_amount , config . indent_lines , & prefix , style ,) ; self . current_buf . clear () ; self . flush_indent_buf () ; if config . indent_lines { match style { SpanMode :: PreOpen | SpanMode :: Open { .. } => { if indent > 0 && (indent + 1) % config . wraparound == 0 { self . current_buf . push_str (& prefix) ; for _ in 0 .. (indent % config . wraparound * config . indent_amount) { self . current_buf . push_str (LINE_HORIZ) ; } self . current_buf . push_str (LINE_CLOSE) ; self . current_buf . push ('\n') ; } } _ => { } } } } }
    };
}

impl_12!()