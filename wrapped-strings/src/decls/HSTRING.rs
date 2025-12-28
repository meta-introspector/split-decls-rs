macro_rules! deps {
    () => {
        HStringHeader!();
    };
}

macro_rules! HSTRING {
    () => {
        deps!();
        # [doc = " An ([HSTRING](https://docs.microsoft.com/en-us/windows/win32/winrt/hstring))"] # [doc = " is a reference-counted and immutable UTF-16 string type."] # [repr (transparent)] pub struct HSTRING (pub (crate) * mut HStringHeader) ;
    };
}

HSTRING!()