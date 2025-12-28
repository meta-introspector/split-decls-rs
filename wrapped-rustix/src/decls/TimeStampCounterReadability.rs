macro_rules! TimeStampCounterReadability {
    () => {
        # [doc = " `PR_TSC_*` values for use with [`time_stamp_counter_readability`] and"] # [doc = " [`set_time_stamp_counter_readability`]."] # [derive (Copy , Clone , Debug , Eq , PartialEq)] # [repr (u32)] pub enum TimeStampCounterReadability { # [doc = " Allow the use of the timestamp counter."] Readable = PR_TSC_ENABLE , # [doc = " Throw a [`Signal::SEGV`] signal instead of reading the TSC."] RaiseSIGSEGV = PR_TSC_SIGSEGV , }
    };
}

TimeStampCounterReadability!()