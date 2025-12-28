macro_rules! SPACE_SEPARATOR {
    () => {
        pub const SPACE_SEPARATOR : & 'static [(u32 , u32)] = & [(32 , 32) , (160 , 160) , (5760 , 5760) , (8192 , 8202) , (8239 , 8239) , (8287 , 8287) , (12288 , 12288) ,] ;
    };
}

SPACE_SEPARATOR!()