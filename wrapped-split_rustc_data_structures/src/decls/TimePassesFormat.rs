macro_rules! TimePassesFormat {
    () => {
        # [doc = " Which format to use for `-Z time-passes`"] # [derive (Clone , Copy , PartialEq , Hash , Debug)] pub enum TimePassesFormat { # [doc = " Emit human readable text"] Text , # [doc = " Emit structured JSON"] Json , }
    };
}

TimePassesFormat!();