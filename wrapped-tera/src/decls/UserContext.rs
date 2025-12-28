macro_rules! deps {
    () => {
        Context!();
    };
}

macro_rules! UserContext {
    () => {
        deps!();
        # [doc = " Contains the user data and allows no mutation"] # [derive (Debug)] pub struct UserContext < 'a > { # [doc = " Read-only context"] inner : & 'a Context , }
    };
}

UserContext!();