macro_rules! USERINFO_ENCODE_SET {
    () => {
        # [doc = " https://url.spec.whatwg.org/#userinfo-percent-encode-set"] # [cfg (feature = "urlencode")] const USERINFO_ENCODE_SET : & AsciiSet = & PATH_ENCODE_SET . add (b'/') . add (b':') . add (b';') . add (b'=') . add (b'@') . add (b'[') . add (b'\\') . add (b']') . add (b'^') . add (b'|') ;
    };
}

USERINFO_ENCODE_SET!();