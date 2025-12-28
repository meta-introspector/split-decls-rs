macro_rules! deps {
    () => {
        SignalsInfo!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl < E : Exfiltrator > SignalsInfo < E > { fn has_signals (read : & mut UnixStream , ctx : & mut Context < '_ >) -> Result < bool , Error > { let mut buf = [0u8] ; let mut read_buf = ReadBuf :: new (& mut buf) ; match Pin :: new (read) . poll_read (ctx , & mut read_buf) { Poll :: Pending => Ok (false) , Poll :: Ready (Ok (())) => Ok (true) , Poll :: Ready (Err (error)) => Err (error) , } } }
    };
}

impl_3!()