macro_rules! deps {
    () => {
        SpecialCodeIndex!();
    };
}

macro_rules! impl_1210 {
    () => {
        deps!();
        # [rustfmt :: skip] impl SpecialCodeIndex { # [doc = " `VINTR`"] pub const VINTR : Self = Self (c :: VINTR as usize) ; # [doc = " `VQUIT`"] pub const VQUIT : Self = Self (c :: VQUIT as usize) ; # [doc = " `VERASE`"] pub const VERASE : Self = Self (c :: VERASE as usize) ; # [doc = " `VKILL`"] pub const VKILL : Self = Self (c :: VKILL as usize) ; # [doc = " `VEOF`"] pub const VEOF : Self = Self (c :: VEOF as usize) ; # [doc = " `VTIME`"] pub const VTIME : Self = Self (c :: VTIME as usize) ; # [doc = " `VMIN`"] pub const VMIN : Self = Self (c :: VMIN as usize) ; # [doc = " `VSWTC`"] # [cfg (not (any (bsd , solarish , target_os = "aix" , target_os = "haiku" , target_os = "hurd" , target_os = "nto" ,)))] pub const VSWTC : Self = Self (c :: VSWTC as usize) ; # [doc = " `VSTART`"] pub const VSTART : Self = Self (c :: VSTART as usize) ; # [doc = " `VSTOP`"] pub const VSTOP : Self = Self (c :: VSTOP as usize) ; # [doc = " `VSUSP`"] pub const VSUSP : Self = Self (c :: VSUSP as usize) ; # [doc = " `VEOL`"] pub const VEOL : Self = Self (c :: VEOL as usize) ; # [doc = " `VREPRINT`"] # [cfg (not (target_os = "haiku"))] pub const VREPRINT : Self = Self (c :: VREPRINT as usize) ; # [doc = " `VDISCARD`"] # [cfg (not (any (target_os = "aix" , target_os = "haiku")))] pub const VDISCARD : Self = Self (c :: VDISCARD as usize) ; # [doc = " `VWERASE`"] # [cfg (not (any (target_os = "aix" , target_os = "haiku")))] pub const VWERASE : Self = Self (c :: VWERASE as usize) ; # [doc = " `VLNEXT`"] # [cfg (not (target_os = "haiku"))] pub const VLNEXT : Self = Self (c :: VLNEXT as usize) ; # [doc = " `VEOL2`"] pub const VEOL2 : Self = Self (c :: VEOL2 as usize) ; # [doc = " `VSWTCH`"] # [cfg (any (solarish , target_os = "haiku" , target_os = "nto"))] pub const VSWTCH : Self = Self (c :: VSWTCH as usize) ; # [doc = " `VDSUSP`"] # [cfg (any (bsd , solarish , target_os = "aix" , target_os = "hurd" , target_os = "nto"))] pub const VDSUSP : Self = Self (c :: VDSUSP as usize) ; # [doc = " `VSTATUS`"] # [cfg (any (bsd , target_os = "hurd" , target_os = "illumos"))] pub const VSTATUS : Self = Self (c :: VSTATUS as usize) ; # [doc = " `VERASE2`"] # [cfg (any (freebsdlike , target_os = "illumos"))] pub const VERASE2 : Self = Self (c :: VERASE2 as usize) ; }
    };
}

impl_1210!();