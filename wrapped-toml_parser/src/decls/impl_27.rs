macro_rules! deps {
    () => {
        Expected!();
        ScalarKind!();
        ErrorSink!();
        Encoding!();
        ParseError!();
        StringBuilder!();
        Span!();
        Raw!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl < 'i > Raw < 'i > { pub fn new_unchecked (raw : & 'i str , encoding : Option < Encoding > , span : Span) -> Self { Self { raw , encoding , span , } } pub fn decode_key (& self , output : & mut dyn StringBuilder < 'i > , error : & mut dyn ErrorSink) { let mut error = | err : crate :: ParseError | { error . report_error (err . rebase_spans (self . span . start)) ; } ; match self . encoding { Some (Encoding :: LiteralString) => { crate :: decoder :: string :: decode_literal_string (* self , output , & mut error) ; } Some (Encoding :: BasicString) => { crate :: decoder :: string :: decode_basic_string (* self , output , & mut error) ; } Some (Encoding :: MlLiteralString) => { error . report_error (crate :: ParseError :: new ("keys cannot be multi-line literal strings") . with_expected (& [Expected :: Description ("basic string") , Expected :: Description ("literal string") ,]) . with_unexpected (Span :: new_unchecked (0 , self . len ())) ,) ; crate :: decoder :: string :: decode_ml_literal_string (* self , output , & mut error) ; } Some (Encoding :: MlBasicString) => { error . report_error (crate :: ParseError :: new ("keys cannot be multi-line basic strings") . with_expected (& [Expected :: Description ("basic string") , Expected :: Description ("literal string") ,]) . with_unexpected (Span :: new_unchecked (0 , self . len ())) ,) ; crate :: decoder :: string :: decode_ml_basic_string (* self , output , & mut error) ; } None => crate :: decoder :: string :: decode_unquoted_key (* self , output , & mut error) , } } # [must_use] pub fn decode_scalar (& self , output : & mut dyn StringBuilder < 'i > , error : & mut dyn ErrorSink ,) -> crate :: decoder :: scalar :: ScalarKind { let mut error = | err : crate :: ParseError | { error . report_error (err . rebase_spans (self . span . start)) ; } ; match self . encoding { Some (Encoding :: LiteralString) => { crate :: decoder :: string :: decode_literal_string (* self , output , & mut error) ; crate :: decoder :: scalar :: ScalarKind :: String } Some (Encoding :: BasicString) => { crate :: decoder :: string :: decode_basic_string (* self , output , & mut error) ; crate :: decoder :: scalar :: ScalarKind :: String } Some (Encoding :: MlLiteralString) => { crate :: decoder :: string :: decode_ml_literal_string (* self , output , & mut error) ; crate :: decoder :: scalar :: ScalarKind :: String } Some (Encoding :: MlBasicString) => { crate :: decoder :: string :: decode_ml_basic_string (* self , output , & mut error) ; crate :: decoder :: scalar :: ScalarKind :: String } None => crate :: decoder :: scalar :: decode_unquoted_scalar (* self , output , & mut error) , } } pub fn decode_whitespace (& self , _error : & mut dyn ErrorSink) { } pub fn decode_comment (& self , error : & mut dyn ErrorSink) { let mut error = | err : crate :: ParseError | { error . report_error (err . rebase_spans (self . span . start)) ; } ; crate :: decoder :: ws :: decode_comment (* self , & mut error) ; } pub fn decode_newline (& self , error : & mut dyn ErrorSink) { let mut error = | err : crate :: ParseError | { error . report_error (err . rebase_spans (self . span . start)) ; } ; crate :: decoder :: ws :: decode_newline (* self , & mut error) ; } pub fn as_str (& self) -> & 'i str { self . raw } pub fn as_bytes (& self) -> & 'i [u8] { self . raw . as_bytes () } pub fn len (& self) -> usize { self . raw . len () } pub fn is_empty (& self) -> bool { self . raw . is_empty () } }
    };
}

impl_27!();