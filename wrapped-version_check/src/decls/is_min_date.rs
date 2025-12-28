macro_rules! deps {
    () => {
        Date!();
    };
}

macro_rules! is_min_date {
    () => {
        deps!();
        # [doc = " Checks that the running or installed `rustc` was released **on or after**"] # [doc = " some date."] # [doc = ""] # [doc = " The format of `min_date` must be YYYY-MM-DD. For instance: `2016-12-20` or"] # [doc = " `2017-01-09`."] # [doc = ""] # [doc = " If the date cannot be retrieved or parsed, or if `min_date` could not be"] # [doc = " parsed, returns `None`. Otherwise returns `true` if the installed `rustc`"] # [doc = " was release on or after `min_date` and `false` otherwise."] pub fn is_min_date (min_date : & str) -> Option < bool > { match (Date :: read () , Date :: parse (min_date)) { (Some (rustc_date) , Some (min_date)) => Some (rustc_date >= min_date) , _ => None , } }
    };
}

is_min_date!();