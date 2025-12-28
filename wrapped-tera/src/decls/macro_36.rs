macro_rules! macro_36 {
    () => {
        lazy_static ! { static ref STRIPTAGS_RE : Regex = Regex :: new (r"(<!--.*?-->|<[^>]*>)") . unwrap () ; static ref WORDS_RE : Regex = Regex :: new (r"\b(?P<first>[\w'])(?P<rest>[\w']*)\b") . unwrap () ; static ref SPACELESS_RE : Regex = Regex :: new (r">\s+<") . unwrap () ; }
    };
}

macro_36!()