macro_rules! deps {
    () => {
        Stream!();
        Result!();
    };
}

macro_rules! process_single_io {
    () => {
        deps!();
        # [cfg (feature = "cmd")] fn process_single_io (child : & mut std :: process :: Child , stdout : os_pipe :: PipeReader , input : Option < Vec < u8 > > ,) -> std :: io :: Result < Stream > { use std :: io :: Write ; let stdin = input . and_then (| i | { child . stdin . take () . map (| mut stdin | std :: thread :: spawn (move | | stdin . write_all (& i))) }) ; let stdout = threaded_read (stdout) ; debug_assert ! (child . stdout . is_none ()) ; debug_assert ! (child . stderr . is_none ()) ; stdin . and_then (| t | t . join () . unwrap () . ok ()) ; Ok (stdout) }
    };
}

process_single_io!();