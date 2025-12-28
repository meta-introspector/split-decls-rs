macro_rules! deps {
    () => {
        NotDefinedHere!();
    };
}

macro_rules! impl_352 {
    () => {
        deps!();
        # [cfg (feature = "net")] impl mio :: event :: Source for NotDefinedHere { fn register (& mut self , _registry : & mio :: Registry , _token : mio :: Token , _interests : mio :: Interest ,) -> std :: io :: Result < () > { Ok (()) } fn reregister (& mut self , _registry : & mio :: Registry , _token : mio :: Token , _interests : mio :: Interest ,) -> std :: io :: Result < () > { Ok (()) } fn deregister (& mut self , _registry : & mio :: Registry) -> std :: io :: Result < () > { Ok (()) } }
    };
}

impl_352!()