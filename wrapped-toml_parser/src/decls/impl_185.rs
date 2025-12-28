macro_rules! deps {
    () => {
        ValidateWhitespace!();
        ErrorSink!();
        Encoding!();
        EventReceiver!();
        Span!();
    };
}

macro_rules! impl_185 {
    () => {
        deps!();
        impl EventReceiver for ValidateWhitespace < '_ , '_ > { fn std_table_open (& mut self , span : Span , error : & mut dyn ErrorSink) { self . receiver . std_table_open (span , error) ; } fn std_table_close (& mut self , span : Span , error : & mut dyn ErrorSink) { self . receiver . std_table_close (span , error) ; } fn array_table_open (& mut self , span : Span , error : & mut dyn ErrorSink) { self . receiver . array_table_open (span , error) ; } fn array_table_close (& mut self , span : Span , error : & mut dyn ErrorSink) { self . receiver . array_table_close (span , error) ; } fn inline_table_open (& mut self , span : Span , error : & mut dyn ErrorSink) -> bool { self . receiver . inline_table_open (span , error) } fn inline_table_close (& mut self , span : Span , error : & mut dyn ErrorSink) { self . receiver . inline_table_close (span , error) ; } fn array_open (& mut self , span : Span , error : & mut dyn ErrorSink) -> bool { self . receiver . array_open (span , error) } fn array_close (& mut self , span : Span , error : & mut dyn ErrorSink) { self . receiver . array_close (span , error) ; } fn simple_key (& mut self , span : Span , encoding : Option < Encoding > , error : & mut dyn ErrorSink) { self . receiver . simple_key (span , encoding , error) ; } fn key_sep (& mut self , span : Span , error : & mut dyn ErrorSink) { self . receiver . key_sep (span , error) ; } fn key_val_sep (& mut self , span : Span , error : & mut dyn ErrorSink) { self . receiver . key_val_sep (span , error) ; } fn scalar (& mut self , span : Span , encoding : Option < Encoding > , error : & mut dyn ErrorSink) { self . receiver . scalar (span , encoding , error) ; } fn value_sep (& mut self , span : Span , error : & mut dyn ErrorSink) { self . receiver . value_sep (span , error) ; } fn whitespace (& mut self , span : Span , error : & mut dyn ErrorSink) { # [cfg (feature = "unsafe")] let raw = unsafe { self . source . get_unchecked (span) } ; # [cfg (not (feature = "unsafe"))] let raw = self . source . get (span) . expect ("token spans are valid") ; raw . decode_whitespace (error) ; self . receiver . whitespace (span , error) ; } fn comment (& mut self , span : Span , error : & mut dyn ErrorSink) { # [cfg (feature = "unsafe")] let raw = unsafe { self . source . get_unchecked (span) } ; # [cfg (not (feature = "unsafe"))] let raw = self . source . get (span) . expect ("token spans are valid") ; raw . decode_comment (error) ; self . receiver . comment (span , error) ; } fn newline (& mut self , span : Span , error : & mut dyn ErrorSink) { # [cfg (feature = "unsafe")] let raw = unsafe { self . source . get_unchecked (span) } ; # [cfg (not (feature = "unsafe"))] let raw = self . source . get (span) . expect ("token spans are valid") ; raw . decode_newline (error) ; self . receiver . newline (span , error) ; } fn error (& mut self , span : Span , error : & mut dyn ErrorSink) { self . receiver . error (span , error) ; } }
    };
}

impl_185!()