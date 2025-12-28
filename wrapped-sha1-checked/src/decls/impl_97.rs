macro_rules! deps {
    () => {
        DetectionState!();
        Sha1!();
        Builder!();
    };
}

macro_rules! impl_97 {
    () => {
        deps!();
        impl Builder { # [doc = " Should we detect collisions at all? Default: true"] pub fn detect_collision (mut self , detect : bool) -> Self { self . detect_collision = detect ; self } # [doc = " Should a fix be automatically be applied, or the original hash be returned? Default: true"] pub fn safe_hash (mut self , safe_hash : bool) -> Self { self . safe_hash = safe_hash ; self } # [doc = " Should unavoidable bitconditions be used to speed up the check? Default: true"] pub fn use_ubc (mut self , ubc : bool) -> Self { self . ubc_check = ubc ; self } # [doc = " Should reduced round collisions be used? Default: false"] pub fn reduced_round_collision (mut self , reduced : bool) -> Self { self . reduced_round_collision = reduced ; self } fn into_detection_state (self) -> Option < DetectionState > { if self . detect_collision { Some (DetectionState { safe_hash : self . safe_hash , reduced_round_collision : self . reduced_round_collision , ubc_check : self . ubc_check , found_collision : false , ihv1 : Default :: default () , ihv2 : Default :: default () , m1 : [0 ; 80] , m2 : [0 ; 80] , state_58 : Default :: default () , state_65 : Default :: default () , }) } else { None } } # [doc = " Create a Sha1 with a specific collision detection configuration."] pub fn build (self) -> Sha1 { let detection = self . into_detection_state () ; Sha1 { h : INITIAL_H , block_len : 0 , detection , buffer : Default :: default () , } } }
    };
}

impl_97!()