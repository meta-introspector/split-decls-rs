macro_rules! deps {
    () => {
        Expect!();
        NewSpan!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl Expect { pub (crate) fn bad (& self , name : impl AsRef < str > , what : fmt :: Arguments < '_ >) { let name = name . as_ref () ; match self { Expect :: Event (e) => panic ! ("\n[{}] expected event {}\n[{}] but instead {}" , name , e , name , what ,) , Expect :: FollowsFrom { consequence , cause } => panic ! ("\n[{}] expected consequence {} to follow cause {} but instead {}" , name , consequence , cause , what ,) , Expect :: Enter (e) => panic ! ("\n[{}] expected to enter {}\n[{}] but instead {}" , name , e , name , what ,) , Expect :: Exit (e) => panic ! ("\n[{}] expected to exit {}\n[{}] but instead {}" , name , e , name , what ,) , Expect :: CloneSpan (e) => { panic ! ("\n[{}] expected to clone {}\n[{}] but instead {}" , name , e , name , what ,) } Expect :: DropSpan (e) => { panic ! ("\n[{}] expected to drop {}\n[{}] but instead {}" , name , e , name , what ,) } Expect :: Visit (e , fields) => panic ! ("\n[{}] expected {} to record {}\n[{}] but instead {}" , name , e , fields , name , what ,) , Expect :: NewSpan (e) => panic ! ("\n[{}] expected {}\n[{}] but instead {}" , name , e , name , what) , Expect :: Nothing => panic ! ("\n[{}] expected nothing else to happen\n[{}] but {} instead" , name , name , what ,) , } } }
    };
}

impl_23!();