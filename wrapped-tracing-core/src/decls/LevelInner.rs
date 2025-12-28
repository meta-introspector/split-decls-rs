macro_rules! LevelInner {
    () => {
        # [repr (usize)] # [derive (Copy , Clone , Debug , Hash , Eq , PartialEq)] enum LevelInner { # [doc = " The \"trace\" level."] # [doc = ""] # [doc = " Designates very low priority, often extremely verbose, information."] Trace = 0 , # [doc = " The \"debug\" level."] # [doc = ""] # [doc = " Designates lower priority information."] Debug = 1 , # [doc = " The \"info\" level."] # [doc = ""] # [doc = " Designates useful information."] Info = 2 , # [doc = " The \"warn\" level."] # [doc = ""] # [doc = " Designates hazardous situations."] Warn = 3 , # [doc = " The \"error\" level."] # [doc = ""] # [doc = " Designates very serious errors."] Error = 4 , }
    };
}

LevelInner!();