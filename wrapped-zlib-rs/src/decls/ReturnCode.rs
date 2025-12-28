macro_rules! ReturnCode {
    () => {
        # [derive (Debug , Copy , Clone , PartialEq , Eq)] # [repr (i32)] pub enum ReturnCode { Ok = 0 , StreamEnd = 1 , NeedDict = 2 , ErrNo = - 1 , StreamError = - 2 , DataError = - 3 , MemError = - 4 , BufError = - 5 , VersionError = - 6 , }
    };
}

ReturnCode!()