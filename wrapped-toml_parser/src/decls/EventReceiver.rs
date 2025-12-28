macro_rules! deps {
    () => {
        Encoding!();
        ErrorSink!();
        Span!();
    };
}

macro_rules! EventReceiver {
    () => {
        deps!();
        pub trait EventReceiver { fn std_table_open (& mut self , _span : Span , _error : & mut dyn ErrorSink) { } fn std_table_close (& mut self , _span : Span , _error : & mut dyn ErrorSink) { } fn array_table_open (& mut self , _span : Span , _error : & mut dyn ErrorSink) { } fn array_table_close (& mut self , _span : Span , _error : & mut dyn ErrorSink) { } # [doc = " Returns if entering the inline table is allowed"] # [must_use] fn inline_table_open (& mut self , _span : Span , _error : & mut dyn ErrorSink) -> bool { true } fn inline_table_close (& mut self , _span : Span , _error : & mut dyn ErrorSink) { } # [doc = " Returns if entering the array is allowed"] # [must_use] fn array_open (& mut self , _span : Span , _error : & mut dyn ErrorSink) -> bool { true } fn array_close (& mut self , _span : Span , _error : & mut dyn ErrorSink) { } fn simple_key (& mut self , _span : Span , _kind : Option < Encoding > , _error : & mut dyn ErrorSink) { } fn key_sep (& mut self , _span : Span , _error : & mut dyn ErrorSink) { } fn key_val_sep (& mut self , _span : Span , _error : & mut dyn ErrorSink) { } fn scalar (& mut self , _span : Span , _kind : Option < Encoding > , _error : & mut dyn ErrorSink) { } fn value_sep (& mut self , _span : Span , _error : & mut dyn ErrorSink) { } fn whitespace (& mut self , _span : Span , _error : & mut dyn ErrorSink) { } fn comment (& mut self , _span : Span , _error : & mut dyn ErrorSink) { } fn newline (& mut self , _span : Span , _error : & mut dyn ErrorSink) { } fn error (& mut self , _span : Span , _error : & mut dyn ErrorSink) { } }
    };
}

EventReceiver!();