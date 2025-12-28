macro_rules! deps {
    () => {
        ScalarKind!();
        ErrorSink!();
        ParseError!();
        Span!();
        Raw!();
    };
}

macro_rules! ensure_float {
    () => {
        deps!();
        # [doc = " ```abnf"] # [doc = " float = float-int-part ( exp / frac [ exp ] )"] # [doc = ""] # [doc = " float-int-part = dec-int"] # [doc = " frac = decimal-point zero-prefixable-int"] # [doc = " decimal-point = %x2E               ; ."] # [doc = " zero-prefixable-int = DIGIT *( DIGIT / underscore DIGIT )"] # [doc = ""] # [doc = " exp = \"e\" float-exp-part"] # [doc = " float-exp-part = [ minus / plus ] zero-prefixable-int"] # [doc = " ```"] pub (crate) fn ensure_float < 'i > (mut value : & 'i str , raw : Raw < 'i > , error : & mut dyn ErrorSink) { ensure_dec_uint (& mut value , raw , false , "invalid mantissa" , error) ; if value . starts_with (".") { let _ = value . next_token () ; ensure_dec_uint (& mut value , raw , true , "invalid fraction" , error) ; } if value . starts_with (['e' , 'E']) { let _ = value . next_token () ; if value . starts_with (['+' , '-']) { let _ = value . next_token () ; } ensure_dec_uint (& mut value , raw , true , "invalid exponent" , error) ; } if ! value . is_empty () { let start = value . offset_from (& raw . as_str ()) ; let end = raw . len () ; error . report_error (ParseError :: new (ScalarKind :: Float . invalid_description ()) . with_context (Span :: new_unchecked (0 , raw . len ())) . with_expected (& []) . with_unexpected (Span :: new_unchecked (start , end)) ,) ; } }
    };
}

ensure_float!()