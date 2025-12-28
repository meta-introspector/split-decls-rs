macro_rules! deps {
    () => {
        LocationMap!();
    };
}

macro_rules! impl_195 {
    () => {
        deps!();
        impl < T > IndexMut < Location > for LocationMap < T > { fn index_mut (& mut self , index : Location) -> & mut Self :: Output { & mut self . map [index . block] [index . statement_index] } }
    };
}

impl_195!();