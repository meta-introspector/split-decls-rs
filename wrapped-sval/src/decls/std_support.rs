macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! std_support {
    () => {
        deps!();
        # [cfg (feature = "std")] mod std_support { use super :: * ; use crate :: std :: error ; impl error :: Error for Error { } }
    };
}

std_support!()