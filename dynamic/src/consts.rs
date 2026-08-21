pub mod globals {
    // 0x1
    pub const FIGHTER_KIND: i32 = 0x2;
    pub const OBJECT_ID: i32 = 0x3;
    pub const FIGHTER: i32 = 0x4;
    pub const MODULE_ACCESSOR: i32 = 0x5;
    // 0x6
    pub const INIT_STATUS_FUNC: i32 = 0x7;
    pub const IS_STOPPING: i32 = 0x8;
    pub const STATUS_KIND_INTERRUPT: i32 = 0x9;
    pub const PREV_STATUS_KIND: i32 = 0xA;
    pub const STATUS_KIND: i32 = 0xB;
    pub const STATUS_COUNT: i32 = 0xC;
    // 0xD
    pub const CURRENT_FRAME: i32 = 0xE;
    pub const CURRENT_FRAME2: i32 = 0xF;
    // 0x10
    // 0x11 func ptr
    // 0x12
    pub const SUB_STATUS3: i32 = 0x13;
    pub const SUB_STATUS2: i32 = 0x14;
    pub const SUB_STATUS: i32 = 0x15;
    pub const SITUATION_KIND: i32 = 0x16;
    pub const PREV_SITUATION_KIND: i32 = 0x17;
    pub const PREV_STATUS_FRAME: i32 = 0x18;
    // 0x19
    pub const STICK_X: i32 = 0x1A;
    pub const STICK_Y: i32 = 0x1B;
    pub const FLICK_X: i32 = 0x1C;
    pub const FLICK_Y: i32 = 0x1D;
    pub const FLICK_Y_DIR: i32 = 0x1E;
    pub const PAD_FLAG: i32 = 0x1F;
    pub const CMD_CAT1: i32 = 0x20;
    pub const CMD_CAT2: i32 = 0x21;
    pub const CMD_CAT3: i32 = 0x22;
    pub const CMD_CAT4: i32 = 0x23;
    // 0x24
    // 0x25
    // 0x26
    // 0x27
    // 0x28 some substatus
    pub const DASH_CALLBACK: i32 = 0x29;
    // 0x2A
    pub const CUSTOM_ROUTINE: i32 = 0x2B;
    // 0x2C
    // 0x2D
    // 0x2E
    // 0x2F
    // 0x30
    // 0x31
    // 0x32 some substatus
    pub const USE_SPECIAL_N_CALLBACK: i32 = 0x38;
    pub const USE_SPECIAL_S_CALLBACK: i32 = 0x39;
    pub const USE_SPECIAL_HI_CALLBACK: i32 = 0x3A;
    pub const USE_SPECIAL_LW_CALLBACK: i32 = 0x3B;
    pub const CHECK_SPECIAL_COMMAND: i32 = 0x3C;
    pub const WAZA_CUSTOMIZE_CONTROL: i32 = 0x3D;
    pub const STATUS_CHANGE_CALLBACK: i32 = 0x3E;
    pub const DAMAGE_MOTION_KIND_CALLBACK: i32 = 0x42;
    pub const DASH_POST_TRANSITION_CALLBACK: i32 = 0x57;
}

pub mod vars {
    pub mod common {
        pub mod instance {
            // flags
            pub const CSTICK_OVERRIDE: i32 = 0x0001;
            pub const CSTICK_OVERRIDE_SECOND: i32 = 0x0002;
            pub const ENABLE_AIR_ESCAPE_MAGNET: i32 = 0x003;
            pub const SPECIAL_WALL_JUMP: i32 = 0x0004;
            pub const IS_DACUS: i32 = 0x0005;
            pub const PERFECT_WAVEDASH: i32 = 0x0006;
            pub const IS_LATE_PIVOT: i32 = 0x0007;
            pub const CAN_PERFECT_PIVOT: i32 = 0x0008;
            pub const IS_SMASH_TURN: i32 = 0x0009;
            pub const ENABLE_AIR_ESCAPE_JUMPSQUAT: i32 = 0x000A;
            pub const IS_KNOCKDOWN_THROW: i32 = 0x000B;
            pub const IS_CC_NON_TUMBLE: i32 = 0x000C;
            pub const IS_GETTING_POSITION_FOR_ECB: i32 = 0x000D;
            pub const CHECK_CHANGE_MOTION_ONLY: i32 = 0x000E;
            pub const IS_INIT: i32 = 0x000F;
            pub const WEIRD_ASS_TURN_RUN_ANIMATION: i32 = 0x0010;
            pub const WAS_PREV_STATUS_CANCELABLE: i32 = 0x0011;
            pub const IS_ENTER_DASH_CANCEL: i32 = 0x0012;
            pub const IS_DITCIT: i32 = 0x0013;
            pub const NO_GROUND_BOUNCE: i32 = 0x0014;
            pub const BURST_LIMIT: i32 = 0x0015;

            // ints
            pub const COSTUME_SLOT_NUMBER: i32 = 0x0001;
            pub const CLIFF_XLU_FRAME: i32 = 0x0002;
            pub const LEFT_STICK_FLICK_X: i32 = 0x0003;
            pub const LEFT_STICK_FLICK_Y: i32 = 0x0004;
            pub const RIGHT_STICK_FLICK_X: i32 = 0x0005;
            pub const RIGHT_STICK_FLICK_Y: i32 = 0x0006;
            pub const PREV_STATUS_TRANSITION_FRAME: i32 = 0x0007;
            pub const ATTACK_LR_CHECK: i32 = 0x0008;
            pub const STALL_TIMER: i32 = 0x0009;

            // floats
            pub const CURRENT_MOMENTUM: i32 = 0x0001;
            pub const JUMPSQUAT_VELOCITY: i32 = 0x0002;
            pub const GROUND_VEL: i32 = 0x0003;
            pub const RAR_LENIENCY: i32 = 0x0004;
            pub const CURRENT_MOMENTUM_SPECIALS: i32 = 0x0005;
            pub const ECB_BOTTOM_Y_OFFSET: i32 = 0x0006;
            pub const CURR_DASH_SPEED: i32 = 0x0007;
            pub const ECB_CENTER_Y_OFFSET: i32 = 0x0008;
            pub const DASH_HIP_OFFSET_X: i32 = 0x0009;
            pub const RUN_HIP_OFFSET_X: i32 = 0x000A;
            pub const LAND_CANCEL_LAG: i32 = 0x000B;
            pub const ATTACK_S3_CSTICK_X: i32 = 0x000C;

        }
        pub mod status {
            // flags
            pub const DISABLE_ECB_SHIFT: i32 = 0x1001;
            pub const IS_DASH_TO_RUN_FRAME: i32 = 0x1002;
            pub const IS_AFTER_DASH_TO_RUN_FRAME: i32 = 0x1003;
            pub const APPLY_DASH_END_SPEED_MUL: i32 = 0x1004;
            pub const ATTACK_DASH_CANCEL_DISABLE: i32 = 0x1005;
            pub const ATTACK_DASH_ENABLE_AIR_FALL: i32 = 0x1006;
            pub const ATTACK_DASH_ENABLE_AIR_CONTINUE: i32 = 0x1007;
            pub const ATTACK_DASH_ENABLE_AIR_DRIFT: i32 = 0x1008;
            pub const ATTACK_DASH_AIR_DRIFT_ENABLED: i32 = 0x1009;
            pub const ATTACK_DASH_ENABLE_AIR_LANDING: i32 = 0x100A;
            pub const SHOULD_WAVELAND: i32 = 0x100B;
            pub const DAMAGE_FLY_RESET_TRIGGER: i32 = 0x100C;
            pub const CSTICK_IRAR: i32 = 0x100D;
            pub const ENABLE_SPECIAL_WALLJUMP: i32 = 0x100E;
            pub const NO_POCKET: i32 = 0x100F;
            pub const IS_DASH_CANCEL: i32 = 0x1010;
            pub const CHECK_HOLD_INPUT: i32 = 0x1011;

            // ints

            // floats
            pub const INITIAL_KNOCKBACK_VEL_X: i32 = 0x1001;
            pub const INITIAL_KNOCKBACK_VEL_Y: i32 = 0x1002;
        }
    }

    pub mod falco {
        pub mod instance {
            // flags
            pub const SPECIAL_LW_DISABLE_STALL: i32 = 0x0100;
        }
        pub mod status {
            // flags
            pub const SPECIAL_LW_SET_ATTACK: i32 = 0x1100;
            pub const SPECIAL_LW_SET_EFFECT: i32 = 0x1101;
            pub const SPECIAL_LW_CONTINUE_MOTION: i32 = 0x1102;

            // ints
            pub const SPECIAL_LW_STOP_Y_FRAME: i32 = 0x1100;
        }
    }

    pub mod fox {

    }

    pub mod iceclimbers {
        pub mod instance {
            // flags
            pub const IS_VOLUNTARY_SOPO_A: i32 = 0x0100;
            pub const IS_VOLUNTARY_SOPO_B: i32 = 0x0101;
            pub const SPECIAL_AIR_N: i32 = 0x0102;
            pub const SPECIAL_AIR_N_HOP: i32 = 0x0103;
            pub const SPECIAL_AIR_N_SPECIAL_FALL: i32 = 0x0104;
            
            // floats
            pub const LIMIT_GAUGE: i32 = 0x0100;
        }
        pub mod status {
            
        }
    }

    pub mod ike {
        pub mod instance {
            // flags
            pub const STORED_AETHER: i32 = 0x0100;
            pub const STORED_AETHER_EFFECT_DISABLE: i32 = 0x0101;

            // ints
            pub const STORED_AETHER_EFFECT_COUNT: i32 = 0x0100;
            pub const STORED_AETHER_DAMAGE_TIMER: i32 = 0x0101;

            // floats
            pub const SPECIAL_N_CHARGE_COUNT: i32 = 0x0100;
        }
        pub mod status {
            // flags
            pub const SPECIAL_S_GROUND_START: i32 = 0x1101;
        }
    }

    pub mod littlemac {
        
    }

    pub mod lucario {
        pub mod instance {
            // flags
            pub const AURA_MAXIMUS: i32 = 0x0100;
            
            // floats
            pub const PREV_DAMAGE_STORAGE: i32 = 0x0100;
            pub const PREV_SPEED_X: i32 = 0x0101;
            pub const PREV_SPEED_Y: i32 = 0x0102;
            pub const PREV_LR: i32 = 0x0103;
        }
        pub mod status {
            // ints
            pub const SPECIAL_S_ROT_ANGLE: i32 = 0x1100;
        }
    }

    pub mod metaknight {
        pub mod instance {
            // flags
            pub const SPECIAL_S_HIT: i32 = 0x0100;
        }
    }

    pub mod ridley {
        pub mod instance {
            // flags
            pub const SPECIAL_N_ATTACK: i32 = 0x0104;
            pub const SPECIAL_LW_IS_SKEWER: i32 = 0x0105;
        }
        pub mod status {
            // flags
            pub const SPECIAL_HI_HOVER_DECIDE_STICK: i32 = 0x1100;
            pub const SPECIAL_LW_POGO_ENABLE_LANDING: i32 = 0x1101;
            pub const SPECIAL_LW_POGO_CHECK_BOUNCE: i32 = 0x1102;

            // floats
            pub const SPECIAL_HI_HOVER_DECIDE_STICK_X: i32 = 0x1100;
            pub const SPECIAL_HI_HOVER_DECIDE_STICK_Y: i32 = 0x1101;
            pub const SPECIAL_HI_CHARGE_DIR: i32 = 0x1102;
            pub const SPECIAL_LW_STICK_Y: i32 = 0x1103;
            pub const SPECIAL_LW_POGO_CHECK_PREV_X: i32 = 0x01104;
            pub const SPECIAL_LW_POGO_CHECK_PREV_Y: i32 = 0x01105;
        }
    }

    pub mod samus {
        pub mod instance {
            // flags
            pub const ICE_MODE: i32 = 0x0100;
            pub const SPEEDBOOSTER_ON: i32 = 0x0101;
            pub const SHINESPARK_ON: i32 = 0x0102;
            pub const SPECIAL_HI_HOP_DISABLE: i32 = 0x0103;

            // ints
            pub const SPEEDBOOSTER_STICK_TIMER: i32 = 0x0100;
            pub const SPEEDBOOSTER_EFFECT_TIMER: i32 = 0x0101;
            pub const SHINESPARK_CHARGE_TIMER: i32 = 0x0102;
            pub const SHINESPARK_EFFECT_TIMER: i32 = 0x0103;

            // floats
            pub const AIM_ANGLE: i32 = 0x0100;
            pub const SPECIAL_N_THROW_LW_CHARGE_STORAGE: i32 = 0x0101;
        }
        pub mod status {
            // flags 
            pub const SHINESPARK_IS_SPECIAL_LW: i32 = 0x1100;
            pub const SHINESPARK_ENABLE_GRAVITY: i32 = 0x1101;
            pub const SHINESPARK_ENABLE_CONTROL: i32 = 0x1102;
            pub const ATTACK_LW3_CHECK_CEIL: i32 = 0x1103;
            pub const SPECIAL_HI_LOCK_ANGLE: i32 = 0x1104;
            pub const SPECIAL_HI_FIX_GBEAM_POS: i32 = 0x1105;
            pub const SPECIAL_LW_BOMB_JUMP_ON: i32 = 0x1106;
            pub const SPECIAL_LW_BOMB_JUMP_HOP: i32 = 0x1107;

            // ints 
            pub const SHINESPARK_AIM_TIMER: i32 = 0x1100;
            pub const SHINESPARK_AIM_EFFECT_TIMER: i32 = 0x1101;
            pub const SHINESPARK_LOOP_TIMER: i32 = 0x1102;
            pub const SPECIAL_LW_JUMP_COUNT_FIX: i32 = 0x1103;

            // floats
            pub const SPECIAL_HI_ANGLE: i32 = 0x1100;
        }
    }

    pub mod wolf {
        pub mod instance {
            
        }
        pub mod status {
            // flags
            pub const SPECIAL_S_DISABLE: i32 = 0x1100;
        }
    }
}

pub mod statuses {
    pub mod falco {
        pub const SPECIAL_LW_LOOP: i32 = 0x1e8;
        pub const SPECIAL_LW_END: i32 = 0x1e9;
        pub const SPECIAL_LW_HIT: i32 = 0x1ea;
    }

    pub mod ridley {
        pub const SPECIAL_LW_POGO: i32 = 0x203;
        pub const SPECIAL_LW_LANDING: i32 = 0x204;
    }

    pub mod wolf {
        pub const SPECIAL_S_RUSH: i32 = 0x1EA;
        pub const SPECIAL_S_END: i32 = 0x1EB;
    }
}

pub mod articles {
    
}