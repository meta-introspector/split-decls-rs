macro_rules! deps {
    () => {
        Result!();
        GitDaemon!();
    };
}

macro_rules! spawn_git_daemon {
    () => {
        deps!();
        # [doc = " Spawn a git daemon process to host all repository at or below `working_dir`."] pub fn spawn_git_daemon (working_dir : impl AsRef < Path >) -> std :: io :: Result < GitDaemon > { let mut ports : Vec < _ > = (9419u16 .. 9419 + 100) . collect () ; fastrand :: shuffle (& mut ports) ; let addr_at = | port | std :: net :: SocketAddr :: from (([127 , 0 , 0 , 1] , port)) ; let free_port = { let listener = std :: net :: TcpListener :: bind (ports . into_iter () . map (addr_at) . collect :: < Vec < _ > > () . as_slice ()) ? ; listener . local_addr () . expect ("listener address is available") . port () } ; let child = std :: process :: Command :: new (GIT_CORE_DIR . join (if cfg ! (windows) { "git-daemon.exe" } else { "git-daemon" })) . current_dir (working_dir) . args (["--verbose" , "--base-path=." , "--export-all" , "--user-path"]) . arg (format ! ("--port={free_port}")) . spawn () ? ; let server_addr = addr_at (free_port) ; for time in gix_lock :: backoff :: Quadratic :: default_with_random () { std :: thread :: sleep (time) ; if std :: net :: TcpStream :: connect (server_addr) . is_ok () { break ; } } Ok (GitDaemon { child , url : format ! ("git://{server_addr}") , }) }
    };
}

spawn_git_daemon!()