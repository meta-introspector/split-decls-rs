macro_rules! deps {
    () => {
        Action!();
        Mock!();
    };
}

macro_rules! Builder {
    () => {
        deps!();
        # [doc = " Builds `Mock` instances."] # [derive (Debug , Clone , Default)] pub struct Builder { actions : VecDeque < Action > , name : String , }
    };
}

Builder!()