macro_rules! deps {
    () => {
        State!();
        InflateFlush!();
        Mode!();
        Code!();
        Codes!();
        Flags!();
        Crc32Fold!();
        Table!();
    };
}

macro_rules! impl_236 {
    () => {
        deps!();
        impl < 'a > State < 'a > { fn new (reader : & 'a [u8] , writer : Writer < 'a >) -> Self { let in_available = reader . len () ; let out_available = writer . capacity () ; Self { flush : InflateFlush :: NoFlush , flags : Flags :: default () , wrap : 0 , mode : Mode :: Head , length : 0 , len_table : Table :: default () , dist_table : Table :: default () , wbits : 0 , offset : 0 , extra : 0 , back : 0 , was : 0 , chunksize : 0 , in_available , out_available , bit_reader : BitReader :: new (reader) , writer , total : 0 , window : Window :: empty () , head : None , lens : [0u16 ; 320] , work : [0u16 ; 288] , ncode : 0 , nlen : 0 , ndist : 0 , have : 0 , next : 0 , error_message : None , checksum : 0 , crc_fold : Crc32Fold :: new () , dmax : 0 , gzip_flags : 0 , codes_codes : [Code :: default () ; crate :: ENOUGH_LENS] , len_codes : [Code :: default () ; crate :: ENOUGH_LENS] , dist_codes : [Code :: default () ; crate :: ENOUGH_DISTS] , } } fn len_table_ref (& self) -> & [Code] { match self . len_table . codes { Codes :: Fixed => & self :: inffixed_tbl :: LENFIX , Codes :: Codes => & self . codes_codes , Codes :: Len => & self . len_codes , Codes :: Dist => & self . dist_codes , } } fn dist_table_ref (& self) -> & [Code] { match self . dist_table . codes { Codes :: Fixed => & self :: inffixed_tbl :: DISTFIX , Codes :: Codes => & self . codes_codes , Codes :: Len => & self . len_codes , Codes :: Dist => & self . dist_codes , } } fn len_table_get (& self , index : usize) -> Code { self . len_table_ref () [index] } fn dist_table_get (& self , index : usize) -> Code { self . dist_table_ref () [index] } }
    };
}

impl_236!();