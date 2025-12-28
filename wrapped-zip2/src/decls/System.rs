macro_rules! System {
    () => {
        # [derive (Clone , Copy , Debug , PartialEq , Eq , Default)] # [repr (u8)] pub enum System { Dos = 0 , Unix = 3 , # [default] Unknown , }
    };
}

System!()