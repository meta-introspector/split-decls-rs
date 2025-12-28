macro_rules! deps {
    () => {
        Output!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl std :: fmt :: Display for Output { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { self . spawn . fmt (f) ? ; if let Some (stdout) = & self . stdout { stdout . fmt (f) ? ; } if let Some (stderr) = & self . stderr { stderr . fmt (f) ? ; } self . fs . fmt (f) ? ; Ok (()) } }
    };
}

impl_56!()