macro_rules! PointerMachina {
    () => {
        # [doc = " following iterator immitates regex::Regex::new(r#\"\"[^\"]*\"|[^.\\[\\]]+\"#) but also strips `\"` and `'`"] struct PointerMachina < 'a > { pointer : & 'a str , single_quoted : bool , dual_quoted : bool , escaped : bool , last_position : usize , }
    };
}

PointerMachina!()