macro_rules! deps {
    () => {
        JodChild!();
    };
}

macro_rules! spawn_with_streaming_output {
    () => {
        deps!();
        # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if `cmd` is not configured to have `stdout` and `stderr` as `piped`."] pub fn spawn_with_streaming_output (mut cmd : Command , on_stdout_line : & mut dyn FnMut (& str) , on_stderr_line : & mut dyn FnMut (& str) ,) -> io :: Result < Output > { let cmd = cmd . stdout (Stdio :: piped ()) . stderr (Stdio :: piped ()) . stdin (Stdio :: null ()) ; let mut child = JodChild (cmd . spawn () ?) ; let (stdout , stderr) = streaming_output (child . stdout . take () . unwrap () , child . stderr . take () . unwrap () , on_stdout_line , on_stderr_line , & mut | | () ,) ? ; let status = child . wait () ? ; Ok (Output { status , stdout , stderr }) }
    };
}

spawn_with_streaming_output!();