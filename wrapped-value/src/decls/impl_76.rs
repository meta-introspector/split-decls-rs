macro_rules! deps {
    () => {
        Variables!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        impl Display for Variables { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { f . write_str ("{") ? ; for (i , (name , value)) in self . 0 . iter () . enumerate () { write ! (f , "{}{}: {}" , if i == 0 { "" } else { ", " } , name , value) ? ; } f . write_str ("}") } }
    };
}

impl_76!();