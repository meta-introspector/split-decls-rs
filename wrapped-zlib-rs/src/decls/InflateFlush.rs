macro_rules! InflateFlush {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Default)] pub enum InflateFlush { # [default] NoFlush = 0 , SyncFlush = 2 , Finish = 4 , Block = 5 , Trees = 6 , }
    };
}

InflateFlush!();