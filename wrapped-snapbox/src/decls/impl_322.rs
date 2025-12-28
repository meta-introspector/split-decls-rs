macro_rules! deps {
    () => {
        Error!();
        Data!();
        FilterPaths!();
        DataInner!();
        Filter!();
    };
}

macro_rules! impl_322 {
    () => {
        deps!();
        impl Filter for FilterPaths { fn filter (& self , data : Data) -> Data { let source = data . source ; let filters = data . filters ; let inner = match data . inner { DataInner :: Error (err) => DataInner :: Error (err) , DataInner :: Binary (bin) => DataInner :: Binary (bin) , DataInner :: Text (text) => { let lines = normalize_paths (& text) ; DataInner :: Text (lines) } # [cfg (feature = "json")] DataInner :: Json (value) => { let mut value = value ; normalize_json_string (& mut value , & normalize_paths) ; DataInner :: Json (value) } # [cfg (feature = "json")] DataInner :: JsonLines (value) => { let mut value = value ; normalize_json_string (& mut value , & normalize_paths) ; DataInner :: JsonLines (value) } # [cfg (feature = "term-svg")] DataInner :: TermSvg (text) => { let lines = normalize_paths (& text) ; DataInner :: TermSvg (lines) } } ; Data { inner , source , filters , } } }
    };
}

impl_322!()