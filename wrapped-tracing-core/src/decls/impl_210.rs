macro_rules! deps {
    () => {
        ParseLevelFilterError!();
        LevelFilter!();
    };
}

macro_rules! impl_210 {
    () => {
        deps!();
        impl FromStr for LevelFilter { type Err = ParseLevelFilterError ; fn from_str (from : & str) -> Result < Self , Self :: Err > { from . parse :: < usize > () . ok () . and_then (| num | match num { 0 => Some (LevelFilter :: OFF) , 1 => Some (LevelFilter :: ERROR) , 2 => Some (LevelFilter :: WARN) , 3 => Some (LevelFilter :: INFO) , 4 => Some (LevelFilter :: DEBUG) , 5 => Some (LevelFilter :: TRACE) , _ => None , }) . or_else (| | match from { "" => Some (LevelFilter :: ERROR) , s if s . eq_ignore_ascii_case ("error") => Some (LevelFilter :: ERROR) , s if s . eq_ignore_ascii_case ("warn") => Some (LevelFilter :: WARN) , s if s . eq_ignore_ascii_case ("info") => Some (LevelFilter :: INFO) , s if s . eq_ignore_ascii_case ("debug") => Some (LevelFilter :: DEBUG) , s if s . eq_ignore_ascii_case ("trace") => Some (LevelFilter :: TRACE) , s if s . eq_ignore_ascii_case ("off") => Some (LevelFilter :: OFF) , _ => None , }) . ok_or (ParseLevelFilterError (())) } }
    };
}

impl_210!();