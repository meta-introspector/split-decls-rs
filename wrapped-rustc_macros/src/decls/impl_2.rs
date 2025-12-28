macro_rules! deps {
    () => {
        RustcVersion!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl RustcVersion { fn parse_cfg_release (env_var : & str) -> Result < Self , Box < dyn std :: error :: Error > > { let value = proc_macro :: tracked_env :: var (env_var) ? ; Self :: parse_str (& value) . ok_or_else (| | format ! ("failed to parse rustc version: {:?}" , value) . into ()) } fn parse_str (value : & str) -> Option < Self > { let mut components = value . split ('-') . next () . unwrap () . splitn (3 , '.') ; let major = components . next () ? . parse () . ok () ? ; let minor = components . next () ? . parse () . ok () ? ; let patch = components . next () . unwrap_or ("0") . parse () . ok () ? ; Some (RustcVersion { major , minor , patch }) } }
    };
}

impl_2!();