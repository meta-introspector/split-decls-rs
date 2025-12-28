macro_rules! deps {
    () => {
        Exfiltrator!();
    };
}

macro_rules! SignalOnly {
    () => {
        deps!();
        # [doc = " An [`Exfiltrator`] providing just the signal numbers."] # [doc = ""] # [doc = " This is the basic exfiltrator for most needs. For that reason, there's the"] # [doc = " [`crate::iterator::Signals`] type alias, to simplify the type names for usual needs."] # [derive (Clone , Copy , Debug , Default)] pub struct SignalOnly ;
    };
}

SignalOnly!()