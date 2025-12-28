macro_rules! deps {
    () => {
        FseTables!();
    };
}

macro_rules! impl_264 {
    () => {
        deps!();
        impl FseTables { pub fn new () -> Self { Self { ll_default : default_ll_table () , ll_previous : None , ml_default : default_ml_table () , ml_previous : None , of_default : default_of_table () , of_previous : None , } } }
    };
}

impl_264!();