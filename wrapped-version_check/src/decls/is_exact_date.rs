macro_rules! is_exact_date {
    () => {
        # [doc = " Checks that the running or installed `rustc` was released **exactly** on"] # [doc = " some date."] # [doc = ""] # [doc = " The format of `date` must be YYYY-MM-DD. For instance: `2016-12-20` or"] # [doc = " `2017-01-09`."] # [doc = ""] # [doc = " If the date cannot be retrieved or parsed, or if `date` could not be parsed,"] # [doc = " returns `None`. Otherwise returns `true` if the installed `rustc` was"] # [doc = " release on `date` and `false` otherwise."] pub fn is_exact_date (date : & str) -> Option < bool > { match (Date :: read () , Date :: parse (date)) { (Some (rustc_date) , Some (date)) => Some (rustc_date == date) , _ => None , } }
    };
}

is_exact_date!()