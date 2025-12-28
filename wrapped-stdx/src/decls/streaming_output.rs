macro_rules! streaming_output {
    () => {
        pub fn streaming_output (out : ChildStdout , err : ChildStderr , on_stdout_line : & mut dyn FnMut (& str) , on_stderr_line : & mut dyn FnMut (& str) , on_eof : & mut dyn FnMut () ,) -> io :: Result < (Vec < u8 > , Vec < u8 >) > { let mut stdout = Vec :: new () ; let mut stderr = Vec :: new () ; imp :: read2 (out , err , & mut | is_out , data , eof | { let idx = if eof { data . len () } else { match data . iter () . rposition (| & b | b == b'\n') { Some (i) => i + 1 , None => return , } } ; { let new_lines = { let dst = if is_out { & mut stdout } else { & mut stderr } ; let start = dst . len () ; let data = data . drain (.. idx) ; dst . extend (data) ; & dst [start ..] } ; for line in String :: from_utf8_lossy (new_lines) . lines () { if is_out { on_stdout_line (line) ; } else { on_stderr_line (line) ; } } if eof { on_eof () ; } } }) ? ; Ok ((stdout , stderr)) }
    };
}

streaming_output!();