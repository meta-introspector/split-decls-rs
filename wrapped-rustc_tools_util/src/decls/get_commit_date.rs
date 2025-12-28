macro_rules! get_commit_date {
    () => {
        # [must_use] pub fn get_commit_date () -> Option < String > { get_output ("git" , & ["log" , "-1" , "--date=short" , "--pretty=format:%cd"]) }
    };
}

get_commit_date!();