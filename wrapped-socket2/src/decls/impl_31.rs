macro_rules! deps {
    () => {
        MsgHdr!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        # [cfg (not (target_os = "redox"))] impl < 'name , 'bufs , 'control > fmt :: Debug for MsgHdr < 'name , 'bufs , 'control > { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { "MsgHdr" . fmt (fmt) } }
    };
}

impl_31!()