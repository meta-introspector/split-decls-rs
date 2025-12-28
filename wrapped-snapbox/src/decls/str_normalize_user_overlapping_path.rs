macro_rules! deps {
    () => {
        NormalizeToExpected!();
        Redactions!();
    };
}

macro_rules! str_normalize_user_overlapping_path {
    () => {
        deps!();
        # [test] fn str_normalize_user_overlapping_path () { let input = "\
a: /home/epage
b: /home/epage/snapbox" ; let pattern = "\
a: [A]
b: [B]" ; let mut sub = Redactions :: new () ; let sep = std :: path :: MAIN_SEPARATOR . to_string () ; let redacted = PathBuf :: from (& sep) . join ("home") . join ("epage") ; sub . insert ("[A]" , redacted) . unwrap () ; let redacted = PathBuf :: from (sep) . join ("home") . join ("epage") . join ("snapbox") ; sub . insert ("[B]" , redacted) . unwrap () ; let actual = NormalizeToExpected :: new () . redact_with (& sub) . unordered () . normalize (input . into () , & pattern . into ()) ; assert_eq ! (actual , pattern . into_data ()) ; }
    };
}

str_normalize_user_overlapping_path!();