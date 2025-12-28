macro_rules! FIELD_NAMES {
    () => {
        static FIELD_NAMES : & [& str] = & ["message" , "log.target" , "log.module_path" , "log.file" , "log.line" ,] ;
    };
}

FIELD_NAMES!();