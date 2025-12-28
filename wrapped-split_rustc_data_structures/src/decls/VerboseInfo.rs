macro_rules! deps {
    () => {
        TimePassesFormat!();
    };
}

macro_rules! VerboseInfo {
    () => {
        deps!();
        struct VerboseInfo { start_time : Instant , start_rss : Option < usize > , message : String , format : TimePassesFormat , }
    };
}

VerboseInfo!();