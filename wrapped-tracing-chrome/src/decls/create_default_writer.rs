macro_rules! create_default_writer {
    () => {
        fn create_default_writer () -> Box < dyn Write + Send > { Box :: new (std :: fs :: File :: create (format ! ("./trace-{}.json" , std :: time :: SystemTime :: UNIX_EPOCH . elapsed () . unwrap () . as_micros ())) . expect ("Failed to create trace file.") ,) }
    };
}

create_default_writer!()