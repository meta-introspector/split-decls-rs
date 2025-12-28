macro_rules! deps {
    () => {
        DataFormat!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl From < & std :: path :: Path > for DataFormat { fn from (path : & std :: path :: Path) -> Self { let file_name = path . file_name () . and_then (| e | e . to_str ()) . unwrap_or_default () ; let mut ext = file_name . strip_prefix ('.') . unwrap_or (file_name) ; while let Some ((_ , new_ext)) = ext . split_once ('.') { ext = new_ext ; match ext { # [cfg (feature = "json")] "json" => { return DataFormat :: Json ; } # [cfg (feature = "json")] "jsonl" => { return DataFormat :: JsonLines ; } # [cfg (feature = "term-svg")] "term.svg" => { return Self :: TermSvg ; } _ => { } } } DataFormat :: Text } }
    };
}

impl_61!()