macro_rules! deps {
    () => {
        NotifyHandle!();
        Message!();
        NotifyActor!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl loader :: Handle for NotifyHandle { fn spawn (sender : loader :: Sender) -> NotifyHandle { let actor = NotifyActor :: new (sender) ; let (sender , receiver) = unbounded :: < Message > () ; let thread = stdx :: thread :: Builder :: new (stdx :: thread :: ThreadIntent :: Worker , "VfsLoader") . spawn (move | | actor . run (receiver)) . expect ("failed to spawn thread") ; NotifyHandle { sender , _thread : thread } } fn set_config (& mut self , config : loader :: Config) { self . sender . send (Message :: Config (config)) . unwrap () ; } fn invalidate (& mut self , path : AbsPathBuf) { self . sender . send (Message :: Invalidate (path)) . unwrap () ; } fn load_sync (& mut self , path : & AbsPath) -> Option < Vec < u8 > > { read (path) } }
    };
}

impl_2!();