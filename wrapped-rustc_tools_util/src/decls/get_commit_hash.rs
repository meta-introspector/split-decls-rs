macro_rules! get_commit_hash {
    () => {
        # [must_use] pub fn get_commit_hash () -> Option < String > { let mut stdout = get_output ("git" , & ["rev-parse" , "HEAD"]) ? ; stdout . truncate (10) ; Some (stdout) }
    };
}

get_commit_hash!()