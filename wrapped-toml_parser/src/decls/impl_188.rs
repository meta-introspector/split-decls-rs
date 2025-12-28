macro_rules! deps {
    () => {
        Span!();
        EventReceiver!();
        RecursionGuard!();
        ParseError!();
        Encoding!();
        ErrorSink!();
    };
}

macro_rules! impl_188 {
    () => {
        deps!();
        impl EventReceiver for RecursionGuard < '_ > { fn std_table_open (& mut self , span : Span , error : & mut dyn ErrorSink) { self . receiver . std_table_open (span , error) ; } fn std_table_close (& mut self , span : Span , error : & mut dyn ErrorSink) { self . receiver . std_table_close (span , error) ; } fn array_table_open (& mut self , span : Span , error : & mut dyn ErrorSink) { self . receiver . array_table_open (span , error) ; } fn array_table_close (& mut self , span : Span , error : & mut dyn ErrorSink) { self . receiver . array_table_close (span , error) ; } fn inline_table_open (& mut self , span : Span , error : & mut dyn ErrorSink) -> bool { let allowed = self . receiver . inline_table_open (span , error) ; self . depth += 1 ; let within_depth = self . within_depth () ; if allowed && ! within_depth { error . report_error (ParseError :: new ("cannot recurse further; max recursion depth met") . with_unexpected (span) ,) ; } allowed && within_depth } fn inline_table_close (& mut self , span : Span , error : & mut dyn ErrorSink) { self . depth -= 1 ; self . receiver . inline_table_close (span , error) ; } fn array_open (& mut self , span : Span , error : & mut dyn ErrorSink) -> bool { let allowed = self . receiver . array_open (span , error) ; self . depth += 1 ; let within_depth = self . within_depth () ; if allowed && ! within_depth { error . report_error (ParseError :: new ("cannot recurse further; max recursion depth met") . with_unexpected (span) ,) ; } allowed && within_depth } fn array_close (& mut self , span : Span , error : & mut dyn ErrorSink) { self . depth -= 1 ; self . receiver . array_close (span , error) ; } fn simple_key (& mut self , span : Span , encoding : Option < Encoding > , error : & mut dyn ErrorSink) { self . receiver . simple_key (span , encoding , error) ; } fn key_sep (& mut self , span : Span , error : & mut dyn ErrorSink) { self . receiver . key_sep (span , error) ; } fn key_val_sep (& mut self , span : Span , error : & mut dyn ErrorSink) { self . receiver . key_val_sep (span , error) ; } fn scalar (& mut self , span : Span , encoding : Option < Encoding > , error : & mut dyn ErrorSink) { self . receiver . scalar (span , encoding , error) ; } fn value_sep (& mut self , span : Span , error : & mut dyn ErrorSink) { self . receiver . value_sep (span , error) ; } fn whitespace (& mut self , span : Span , error : & mut dyn ErrorSink) { self . receiver . whitespace (span , error) ; } fn comment (& mut self , span : Span , error : & mut dyn ErrorSink) { self . receiver . comment (span , error) ; } fn newline (& mut self , span : Span , error : & mut dyn ErrorSink) { self . receiver . newline (span , error) ; } fn error (& mut self , span : Span , error : & mut dyn ErrorSink) { self . receiver . error (span , error) ; } }
    };
}

impl_188!()