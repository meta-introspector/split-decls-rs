macro_rules! PATH_ENCODE_SET {
    () => {
        # [doc = " https://url.spec.whatwg.org/#path-percent-encode-set"] # [cfg (feature = "urlencode")] const PATH_ENCODE_SET : & AsciiSet = & FRAGMENT_ENCODE_SET . add (b'#') . add (b'?') . add (b'{') . add (b'}') ;
    };
}

PATH_ENCODE_SET!()