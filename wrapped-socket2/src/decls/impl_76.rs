macro_rules! deps {
    () => {
        MsgHdrMut!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        # [cfg (not (target_os = "redox"))] impl < 'name , 'bufs , 'control > fmt :: Debug for MsgHdrMut < 'name , 'bufs , 'control > { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { "MsgHdrMut" . fmt (fmt) } }
    };
}

impl_76!();