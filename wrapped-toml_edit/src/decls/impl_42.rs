macro_rules! deps {
    () => {
        DocumentMut!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl std :: ops :: DerefMut for DocumentMut { fn deref_mut (& mut self) -> & mut Self :: Target { self . as_table_mut () } }
    };
}

impl_42!();