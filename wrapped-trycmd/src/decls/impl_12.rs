macro_rules! deps {
    () => {
        CommandStatus!();
        Step!();
        Bin!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl Step { pub (crate) fn to_command (& self , cwd : Option < & std :: path :: Path > ,) -> Result < snapbox :: cmd :: Command , crate :: Error > { let bin = match & self . bin { Some (Bin :: Path (path)) => Ok (path . clone ()) , Some (Bin :: Name (name)) => Err (format ! ("Unknown bin.name = {name}") . into ()) , Some (Bin :: Ignore) => Err ("Internal error: tried to run an ignored bin" . into ()) , Some (Bin :: Error (err)) => Err (err . clone ()) , None => Err ("No bin specified" . into ()) , } ? ; if ! bin . exists () { return Err (format ! ("Bin doesn't exist: {}" , bin . display ()) . into ()) ; } let mut cmd = snapbox :: cmd :: Command :: new (bin) . args (& self . args) ; if let Some (cwd) = cwd { cmd = cmd . current_dir (cwd) ; } if let Some (stdin) = & self . stdin { cmd = cmd . stdin (stdin) ; } if self . stderr_to_stdout { cmd = cmd . stderr_to_stdout () ; } if let Some (timeout) = self . timeout { cmd = cmd . timeout (timeout) ; } cmd = self . env . apply (cmd) ; Ok (cmd) } pub (crate) fn expected_status (& self) -> CommandStatus { self . expected_status . unwrap_or_default () } }
    };
}

impl_12!();