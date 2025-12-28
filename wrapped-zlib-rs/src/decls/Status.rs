macro_rules! Status {
    () => {
        # [repr (u8)] # [derive (Debug , Clone , Copy , PartialEq , Eq)] enum Status { Init = 1 , GZip = 4 , Extra = 5 , Name = 6 , Comment = 7 , Hcrc = 8 , Busy = 2 , Finish = 3 , }
    };
}

Status!();