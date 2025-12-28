macro_rules! deps {
    () => {
        RedactedValueInner!();
    };
}

macro_rules! impl_227 {
    () => {
        deps!();
        impl RedactedValueInner { fn find_in (& self , buffer : & str) -> Option < std :: ops :: Range < usize > > { match self { Self :: Str (s) => buffer . find (s) . map (| offset | offset .. (offset + s . len ())) , Self :: String (s) => buffer . find (s) . map (| offset | offset .. (offset + s . len ())) , Self :: Path { native , normalized } => { match (buffer . find (native) , buffer . find (normalized)) { (Some (native_offset) , Some (normalized_offset)) => { if native_offset <= normalized_offset { Some (native_offset .. (native_offset + native . len ())) } else { Some (normalized_offset .. (normalized_offset + normalized . len ())) } } (Some (offset) , None) => Some (offset .. (offset + native . len ())) , (None , Some (offset)) => Some (offset .. (offset + normalized . len ())) , (None , None) => None , } } # [cfg (feature = "regex")] Self :: Regex (r) => { let captures = r . captures (buffer) ? ; let m = captures . name ("redacted") . or_else (| | captures . get (0)) ? ; Some (m . range ()) } } } fn as_cmp (& self) -> (usize , std :: cmp :: Reverse < usize > , & str) { match self { Self :: Str (s) => (0 , std :: cmp :: Reverse (s . len ()) , s) , Self :: String (s) => (0 , std :: cmp :: Reverse (s . len ()) , s) , Self :: Path { normalized : s , .. } => (0 , std :: cmp :: Reverse (s . len ()) , s) , # [cfg (feature = "regex")] Self :: Regex (r) => { let s = r . as_str () ; (1 , std :: cmp :: Reverse (s . len ()) , s) } } } }
    };
}

impl_227!();