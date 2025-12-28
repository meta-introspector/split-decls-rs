macro_rules! SerdeAdapter {
    () => {
        pub trait SerdeAdapter : Send + Sync { fn to_string_pretty < T : ? Sized + Debug + Serialize > (& self , value : & T) -> Result < String , String > ; fn from_str < 'a , T : Debug + Deserialize < 'a > > (& self , s : & 'a str) -> Result < T , String > ; }
    };
}

SerdeAdapter!();