macro_rules! deps {
    () => {
        ErrorKind!();
        Error!();
        Timestamp!();
    };
}

macro_rules! impl_80 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl TryFrom < std :: time :: SystemTime > for Timestamp { type Error = crate :: Error ; # [doc = " Perform the conversion."] # [doc = ""] # [doc = " This method will fail if the system time is earlier than the Unix Epoch."] # [doc = " On some platforms it may panic instead."] fn try_from (st : std :: time :: SystemTime) -> Result < Self , Self :: Error > { let dur = st . duration_since (std :: time :: UNIX_EPOCH) . map_err (| _ | { crate :: Error (crate :: error :: ErrorKind :: InvalidSystemTime ("unable to convert the system tie into a Unix timestamp" ,)) }) ? ; Ok (Self :: from_unix_time (dur . as_secs () , dur . subsec_nanos () , 0 , 0 ,)) } }
    };
}

impl_80!()