macro_rules! from_linger {
    () => {
        const fn from_linger (linger : sys :: linger) -> Option < Duration > { if linger . l_onoff == 0 { None } else { Some (Duration :: from_secs (linger . l_linger as u64)) } }
    };
}

from_linger!()