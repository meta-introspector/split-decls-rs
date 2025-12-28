macro_rules! deps {
    () => {
        Result!();
        Command!();
        Error!();
        OutputAssert!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        # [doc = " # Run Command"] impl Command { # [doc = " Run the command and assert on the results"] # [doc = ""] # [doc = " ```rust"] # [doc = " use snapbox::cmd::Command;"] # [doc = ""] # [doc = " let mut cmd = Command::new(\"cat\")"] # [doc = "     .arg(\"-et\")"] # [doc = "     .stdin(\"42\")"] # [doc = "     .assert()"] # [doc = "     .stdout_eq(\"42\");"] # [doc = " ```"] # [track_caller] pub fn assert (self) -> OutputAssert { let config = self . config . clone () ; match self . output () { Ok (output) => OutputAssert :: new (output) . with_assert (config) , Err (err) => { panic ! ("Failed to spawn: {}" , err) } } } # [doc = " Run the command and capture the `Output`"] # [cfg (feature = "cmd")] pub fn output (self) -> Result < std :: process :: Output , std :: io :: Error > { if self . _stderr_to_stdout { self . single_output () } else { self . split_output () } } # [cfg (not (feature = "cmd"))] pub fn output (self) -> Result < std :: process :: Output , std :: io :: Error > { self . split_output () } # [cfg (feature = "cmd")] fn single_output (mut self) -> Result < std :: process :: Output , std :: io :: Error > { self . cmd . stdin (std :: process :: Stdio :: piped ()) ; let (reader , writer) = os_pipe :: pipe () ? ; let writer_clone = writer . try_clone () ? ; self . cmd . stdout (writer) ; self . cmd . stderr (writer_clone) ; let mut child = self . cmd . spawn () ? ; drop (self . cmd) ; let stdin = self . stdin . as_ref () . map (| d | d . to_bytes ()) . transpose () . map_err (| e | std :: io :: Error :: new (std :: io :: ErrorKind :: NotFound , e)) ? ; let stdout = process_single_io (& mut child , reader , stdin) ? ; let status = wait (child , self . timeout) ? ; let stdout = stdout . join () . unwrap () . ok () . unwrap_or_default () ; Ok (std :: process :: Output { status , stdout , stderr : Default :: default () , }) } fn split_output (mut self) -> Result < std :: process :: Output , std :: io :: Error > { self . cmd . stdin (std :: process :: Stdio :: piped ()) ; self . cmd . stdout (std :: process :: Stdio :: piped ()) ; self . cmd . stderr (std :: process :: Stdio :: piped ()) ; let mut child = self . cmd . spawn () ? ; let stdin = self . stdin . as_ref () . map (| d | d . to_bytes ()) . transpose () . map_err (| e | std :: io :: Error :: new (std :: io :: ErrorKind :: NotFound , e)) ? ; let (stdout , stderr) = process_split_io (& mut child , stdin) ? ; let status = wait (child , self . timeout) ? ; let stdout = stdout . and_then (| t | t . join () . unwrap () . ok ()) . unwrap_or_default () ; let stderr = stderr . and_then (| t | t . join () . unwrap () . ok ()) . unwrap_or_default () ; Ok (std :: process :: Output { status , stdout , stderr , }) } }
    };
}

impl_38!()