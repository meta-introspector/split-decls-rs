macro_rules! deps {
    () => {
        Spawn!();
        SpawnStatus!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        impl std :: fmt :: Display for Spawn { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { let palette = snapbox :: report :: Palette :: color () ; match & self . status { SpawnStatus :: Ok => { if let Some (exit) = self . exit { if exit . success () { writeln ! (f , "Exit: {}" , palette . info ("success")) ? ; } else if let Some (code) = exit . code () { writeln ! (f , "Exit: {}" , palette . error (code)) ? ; } else { writeln ! (f , "Exit: {}" , palette . error ("interrupted")) ? ; } } } SpawnStatus :: Skipped => { writeln ! (f , "{}" , palette . warn ("Skipped")) ? ; } SpawnStatus :: Failure (msg) => { writeln ! (f , "Failed: {}" , palette . error (msg)) ? ; } SpawnStatus :: Expected (expected) => { if let Some (exit) = self . exit { if exit . success () { writeln ! (f , "Expected {}, was {}" , palette . info (expected) , palette . error ("success")) ? ; } else { writeln ! (f , "Expected {}, was {}" , palette . info (expected) , palette . error (snapbox :: cmd :: display_exit_status (exit))) ? ; } } } } Ok (()) } }
    };
}

impl_60!()