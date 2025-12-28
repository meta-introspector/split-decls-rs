macro_rules! deps {
    () => {
        LocationMap!();
    };
}

macro_rules! impl_194 {
    () => {
        deps!();
        impl < T > Index < Location > for LocationMap < T > { type Output = T ; fn index (& self , index : Location) -> & Self :: Output { & self . map [index . block] [index . statement_index] } }
    };
}

impl_194!()