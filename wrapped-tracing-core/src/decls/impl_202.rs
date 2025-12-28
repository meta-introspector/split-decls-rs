macro_rules! deps {
    () => {
        Level!();
        ParseLevelError!();
    };
}

macro_rules! impl_202 {
    () => {
        deps!();
        impl FromStr for Level { type Err = ParseLevelError ; fn from_str (s : & str) -> Result < Self , ParseLevelError > { s . parse :: < usize > () . map_err (| _ | ParseLevelError { _p : () }) . and_then (| num | match num { 1 => Ok (Level :: ERROR) , 2 => Ok (Level :: WARN) , 3 => Ok (Level :: INFO) , 4 => Ok (Level :: DEBUG) , 5 => Ok (Level :: TRACE) , _ => Err (ParseLevelError { _p : () }) , }) . or_else (| _ | match s { s if s . eq_ignore_ascii_case ("error") => Ok (Level :: ERROR) , s if s . eq_ignore_ascii_case ("warn") => Ok (Level :: WARN) , s if s . eq_ignore_ascii_case ("info") => Ok (Level :: INFO) , s if s . eq_ignore_ascii_case ("debug") => Ok (Level :: DEBUG) , s if s . eq_ignore_ascii_case ("trace") => Ok (Level :: TRACE) , _ => Err (ParseLevelError { _p : () }) , }) } }
    };
}

impl_202!()