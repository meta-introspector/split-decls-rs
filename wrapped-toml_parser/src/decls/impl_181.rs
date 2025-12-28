macro_rules! deps {
    () => {
        EventReceiver!();
        EventKind!();
        Span!();
        Event!();
        ErrorSink!();
        Encoding!();
    };
}

macro_rules! impl_181 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] # [allow (unused_qualifications)] impl EventReceiver for alloc :: vec :: Vec < Event > { fn std_table_open (& mut self , span : Span , _error : & mut dyn ErrorSink) { self . push (Event { kind : EventKind :: StdTableOpen , encoding : None , span , }) ; } fn std_table_close (& mut self , span : Span , _error : & mut dyn ErrorSink) { self . push (Event { kind : EventKind :: StdTableClose , encoding : None , span , }) ; } fn array_table_open (& mut self , span : Span , _error : & mut dyn ErrorSink) { self . push (Event { kind : EventKind :: ArrayTableOpen , encoding : None , span , }) ; } fn array_table_close (& mut self , span : Span , _error : & mut dyn ErrorSink) { self . push (Event { kind : EventKind :: ArrayTableClose , encoding : None , span , }) ; } fn inline_table_open (& mut self , span : Span , _error : & mut dyn ErrorSink) -> bool { self . push (Event { kind : EventKind :: InlineTableOpen , encoding : None , span , }) ; true } fn inline_table_close (& mut self , span : Span , _error : & mut dyn ErrorSink) { self . push (Event { kind : EventKind :: InlineTableClose , encoding : None , span , }) ; } fn array_open (& mut self , span : Span , _error : & mut dyn ErrorSink) -> bool { self . push (Event { kind : EventKind :: ArrayOpen , encoding : None , span , }) ; true } fn array_close (& mut self , span : Span , _error : & mut dyn ErrorSink) { self . push (Event { kind : EventKind :: ArrayClose , encoding : None , span , }) ; } fn simple_key (& mut self , span : Span , encoding : Option < Encoding > , _error : & mut dyn ErrorSink) { self . push (Event { kind : EventKind :: SimpleKey , encoding , span , }) ; } fn key_sep (& mut self , span : Span , _error : & mut dyn ErrorSink) { self . push (Event { kind : EventKind :: KeySep , encoding : None , span , }) ; } fn key_val_sep (& mut self , span : Span , _error : & mut dyn ErrorSink) { self . push (Event { kind : EventKind :: KeyValSep , encoding : None , span , }) ; } fn scalar (& mut self , span : Span , encoding : Option < Encoding > , _error : & mut dyn ErrorSink) { self . push (Event { kind : EventKind :: Scalar , encoding , span , }) ; } fn value_sep (& mut self , span : Span , _error : & mut dyn ErrorSink) { self . push (Event { kind : EventKind :: ValueSep , encoding : None , span , }) ; } fn whitespace (& mut self , span : Span , _error : & mut dyn ErrorSink) { self . push (Event { kind : EventKind :: Whitespace , encoding : None , span , }) ; } fn comment (& mut self , span : Span , _error : & mut dyn ErrorSink) { self . push (Event { kind : EventKind :: Comment , encoding : None , span , }) ; } fn newline (& mut self , span : Span , _error : & mut dyn ErrorSink) { self . push (Event { kind : EventKind :: Newline , encoding : None , span , }) ; } fn error (& mut self , span : Span , _error : & mut dyn ErrorSink) { self . push (Event { kind : EventKind :: Error , encoding : None , span , }) ; } }
    };
}

impl_181!()