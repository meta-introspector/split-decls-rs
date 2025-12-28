macro_rules! FRAGMENT_ENCODE_SET {
    () => {
        # [doc = " https://url.spec.whatwg.org/#fragment-percent-encode-set"] # [cfg (feature = "urlencode")] const FRAGMENT_ENCODE_SET : & AsciiSet = & percent_encoding :: CONTROLS . add (b' ') . add (b'"') . add (b'<') . add (b'>') . add (b'`') ;
    };
}

FRAGMENT_ENCODE_SET!()