macro_rules! GitDaemon {
    () => {
        # [doc = " A wrapper for a running git-daemon process which is killed automatically on drop."] # [doc = ""] # [doc = " Note that we will swallow any errors, assuming that the test would have failed if the daemon crashed."] pub struct GitDaemon { child : std :: process :: Child , # [doc = " The base url under which all repositories are hosted, typically `git://127.0.0.1:port`."] pub url : String , }
    };
}

GitDaemon!();