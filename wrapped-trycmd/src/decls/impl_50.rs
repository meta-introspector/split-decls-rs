macro_rules! deps {
    () => {
        Case!();
        Mode!();
        BinRegistry!();
        Runner!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl Runner { pub (crate) fn new () -> Self { Self { cases : Default :: default () , } } pub (crate) fn case (& mut self , case : Case) { self . cases . push (case) ; } pub (crate) fn run (& self , mode : & Mode , bins : & crate :: BinRegistry , substitutions : & snapbox :: Redactions ,) { # ! [allow (unexpected_cfgs)] let palette = snapbox :: report :: Palette :: color () ; if self . cases . is_empty () { eprintln ! ("{}" , palette . warn ("There are no trycmd tests enabled yet")) ; } else { let failures : Vec < _ > = self . cases . par_iter () . flat_map (| c | { let results = c . run (mode , bins , substitutions) ; let stderr = stderr () ; let mut stderr = stderr . lock () ; results . into_iter () . filter_map (| s | { snapbox :: debug ! ("Case: {:#?}" , s) ; match s { Ok (status) => { let _ = write ! (stderr , "{} {} ... {}" , palette . hint ("Testing") , status . name () , status . spawn . status . summary () ,) ; if let Some (duration) = status . duration { let _ = write ! (stderr , " {}" , palette . hint (humantime :: format_duration (duration)) ,) ; } let _ = writeln ! (stderr) ; if ! status . is_ok () { let _ = write ! (stderr , "{}" , & status) ; } None } Err (status) => { let _ = write ! (stderr , "{} {} ... {}" , palette . hint ("Testing") , status . name () , palette . error ("failed") ,) ; if let Some (duration) = status . duration { let _ = write ! (stderr , " {}" , palette . hint (humantime :: format_duration (duration)) ,) ; } let _ = writeln ! (stderr) ; let _ = write ! (stderr , "{}" , & status) ; Some (status) } } }) . collect :: < Vec < _ > > () }) . collect () ; if ! failures . is_empty () { let stderr = stderr () ; let mut stderr = stderr . lock () ; let _ = writeln ! (stderr , "{}" , palette . hint ("Update snapshots with `TRYCMD=overwrite`") ,) ; let _ = writeln ! (stderr , "{}" , palette . hint ("Debug output with `TRYCMD=dump`") ,) ; panic ! ("{} of {} tests failed" , failures . len () , self . cases . len ()) ; } } } }
    };
}

impl_50!();