macro_rules! deps {
    () => {
        Data!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl Visit for Data { fn record_debug (& mut self , field : & Field , value : & dyn fmt :: Debug) { self . kvs . push ((field . name () , format ! ("{:?}" , value))) } }
    };
}

impl_4!()