macro_rules! is_datetime {
    () => {
        # [doc = " Check if serializing a [`Datetime`][crate::Datetime]"] pub fn is_datetime (name : & 'static str) -> bool { crate :: datetime :: is_datetime (name) }
    };
}

is_datetime!()