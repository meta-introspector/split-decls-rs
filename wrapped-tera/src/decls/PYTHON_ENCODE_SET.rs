macro_rules! PYTHON_ENCODE_SET {
    () => {
        # [doc = " Same as Python quote"] # [doc = " https://github.com/python/cpython/blob/da27d9b9dc44913ffee8f28d9638985eaaa03755/Lib/urllib/parse.py#L787"] # [doc = " with `/` not escaped"] # [cfg (feature = "urlencode")] const PYTHON_ENCODE_SET : & AsciiSet = & USERINFO_ENCODE_SET . remove (b'/') . add (b':') . add (b'?') . add (b'#') . add (b'[') . add (b']') . add (b'@') . add (b'!') . add (b'$') . add (b'&') . add (b'\'') . add (b'(') . add (b')') . add (b'*') . add (b'+') . add (b',') . add (b';') . add (b'=') ;
    };
}

PYTHON_ENCODE_SET!()