macro_rules! deps {
    () => {
        RefSignalIterator!();
        Exfiltrator!();
    };
}

macro_rules! Forever {
    () => {
        deps!();
        # [doc = " An infinite iterator of arriving signals."] pub struct Forever < 'a , E : Exfiltrator > (RefSignalIterator < 'a , UnixStream , E >) ;
    };
}

Forever!();