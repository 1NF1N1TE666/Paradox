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

/*
WuBoy's VarModule Constant Overhaul!
The way our variable constants are labelled is changing.

Variables now have two categories:
INSTANCE, which persists until manually changed. Represented by 0x0XXX.
STATUS, which is automatically reset when the status changes. Represented by 0x1XXX.

In addition, there are two sub-categories.
Common, which is shared by every fighter. Represented by 0xX0XX.
Agent, which is specific to a certain fighter/agent. Represented by 0xX1XX.

This means for each combination, you have access to 256 common variables and 256 agent variables.
That's a LOT of space, and I don't anticipate it all gets used up with proper variable application.
*/

pub mod vars {
    pub mod common {
        pub mod instance {
            pub const HITSTUN_START: i32 = 0x0000;
            pub const IS_IN_HITSTUN: i32 = 0x0001;
            pub const CSTICK_OVERRIDE: i32 = 0x0002;
            pub const CSTICK_OVERRIDE_SECOND: i32 = 0x0003;
            pub const OMNI_FLOAT: i32 = 0x0005;
            pub const AERIAL_NO_FLOAT: i32 = 0x0006;
            pub const FLOAT_PAUSE_AERIAL: i32 = 0x0007;
            pub const SIDE_SPECIAL_CANCEL: i32 = 0x0008;
            pub const UP_SPECIAL_CANCEL: i32 = 0x0009;
            pub const JAB_DA_CHECKS: i32 = 0x000A;
            pub const TILT_CHECKS: i32 = 0x000B;
            pub const AERIAL_CHECKS: i32 = 0x000C;
            pub const SMASH_CHECKS: i32 = 0x000D;
            pub const SPECIAL_STALL: i32 = 0x000E;
            pub const SPECIAL_STALL_USED: i32 = 0x000F;
            pub const ENABLE_AIR_ESCAPE_MAGNET: i32 = 0x0010;
            pub const FOOTSTOOL_AIRDODGE_LOCKOUT: i32 = 0x0012;
            pub const CAN_ESCAPE_TUMBLE: i32 = 0x0013;
            pub const SPECIAL_WALL_JUMP: i32 = 0x0014;
            pub const TETHER_HOGGED: i32 = 0x0015;
            pub const B_REVERSED: i32 = 0x0016; // Converted for now, but will likely get removed when B Reverse Reimplementation happens
            pub const TUMBLE_KB: i32 = 0x0017;
            pub const CAN_GLIDE_TOSS: i32 = 0x0019;
            pub const IS_MOTION_BASED_ATTACK: i32 = 0x001A;
            pub const PREV_FLAG_DISABLE_ESCAPE_AIR: i32 = 0x001B;
            pub const ENABLE_WAVELAND_PLATDROP: i32 = 0x001C;
            pub const IS_DACUS: i32 = 0x001D;
            pub const IS_STICKY_WALK: i32 = 0x001E;
            pub const ENABLE_BOOST_RUN: i32 = 0x001F;
            pub const PERFECT_WAVEDASH: i32 = 0x0020;
            pub const JUMP_NEXT: i32 = 0x0021;
            pub const SHOULD_TRUMP_TETHER: i32 = 0x0022;
            pub const UP_SPECIAL_INTERRUPT: i32 = 0x0023; // Ness and Lucas use this
            pub const UP_SPECIAL_INTERRUPT_AIRTIME: i32 = 0x0024; // Ness and Lucas use this
            pub const SPECIAL_PROJECTILE_SPAWNED: i32 = 0x0025; // Luigi, Ivysaur, and Young Link use this
            pub const STALL_PREVENTION: i32 = 0x0027; //Ness and Lucas down b stall prevention
            pub const SPIN_ATTACK_LAND_CANCEL: i32 = 0x003E; // Link and Mii Sword use this
            pub const SIDE_SPECIAL_CANCEL_NO_HIT: i32 = 0x004D; // Used by Kazuya and Sora
            pub const IS_LATE_PIVOT: i32 = 0x004E;
            pub const CAN_PERFECT_PIVOT: i32 = 0x004F;
            pub const IS_SMASH_TURN: i32 = 0x0050;
            pub const ENABLE_AIR_ESCAPE_JUMPSQUAT: i32 = 0x0051;
            pub const IS_KNOCKDOWN_THROW: i32 = 0x0052;
            pub const IS_HEAVY_ATTACK: i32 = 0x0053;
            pub const IS_CC_NON_TUMBLE: i32 = 0x0054;
            pub const IS_GETTING_POSITION_FOR_ECB: i32 = 0x0055;
            pub const CHECK_CHANGE_MOTION_ONLY: i32 = 0x0056;
            pub const BEFORE_GROUND_COLLISION: i32 = 0x0057;
            pub const IS_IGNORED_STATUS_FRAME_0: i32 = 0x0058;
            pub const FLUSH_EFFECT_ACMD: i32 = 0x0059;
            pub const IS_PARRY_FOR_GUARD_OFF: i32 = 0x0060;
            pub const TEMPORARY_CLIFF_STOP: i32 = 0x0061;
            pub const ENABLE_FRAME_DATA_DEBUG: i32 = 0x0062;
            pub const IS_ATTACK_CANCEL: i32 = 0x0063;
            pub const DISABLE_CSTICK_BUFFER_ROLL_OOS: i32 = 0x0064;
            pub const IS_INIT: i32 = 0x0065;
            pub const IS_FLOAT: i32 = 0x0066;
            pub const WEIRD_ASS_TURN_RUN_ANIMATION: i32 = 0x0067;
            pub const ACMD_EFFECT: i32 = 0x0068;
            pub const WAS_PREV_STATUS_CANCELABLE: i32 = 0x0069;
            pub const IS_ENTER_DASH_CANCEL: i32 = 0x006A;
            pub const DOWN_DISABLE_PASSIVE: i32 = 0x006B;
            pub const DOWN_DISABLE_A_LAND: i32 = 0x006C;
            pub const IS_KILLING_BLOW: i32 = 0x006D;
            pub const IS_DITCIT: i32 = 0x006E;

            pub const PARADOX_BURST_LIMIT: i32 = 0x006F;
            pub const DISABLE_CLUTCH: i32 = 0x0070;

            // ints

            pub const LAST_ATTACK_RECEIVER_ENTRY_ID: i32 = 0x0000;
            pub const COSTUME_SLOT_NUMBER: i32 = 0x0001;
            pub const FLOAT_DURATION: i32 = 0x0002;
            pub const FLOAT_STATUS_KIND: i32 = 0x0003;
            pub const HITFALL_BUFFER: i32 = 0x0004;
            pub const FLY_NEXT_FRAME: i32 = 0x0005;
            pub const GIMMICK_TIMER: i32 = 0x0006;
            pub const AIR_ESCAPE_MAGNET_FRAME: i32 = 0x0007;
            pub const CSTICK_LIFE: i32 = 0x0008;
            pub const AGT_USED_COUNTER: i32 = 0x0009;
            pub const CLIFF_XLU_FRAME: i32 = 0x000A;
            pub const LAST_ATTACK_HITBOX_ID: i32 = 0x000B;
            pub const SHIELD_EFFECT_HANDLE: i32 = 0x000C;
            pub const FRAME_COUNTER: i32 = 0x000D;
            pub const LEFT_STICK_FLICK_X: i32 = 0x000E;
            pub const LEFT_STICK_FLICK_Y: i32 = 0x000F;
            pub const RIGHT_STICK_FLICK_X: i32 = 0x0010;
            pub const RIGHT_STICK_FLICK_Y: i32 = 0x0011;
            pub const PREV_STATUS_TRANSITION_FRAME: i32 = 0x0012;
            pub const ATTACK_LR_CHECK: i32 = 0x0013;

            pub const AIR_TIME: i32 = 0x0014;

            // floats

            pub const LAST_ATTACK_DAMAGE_DEALT: i32 = 0x0000;
            pub const CURRENT_MOMENTUM: i32 = 0x0001;
            pub const JUMPSQUAT_VELOCITY: i32 = 0x0002;
            pub const JUMP_SPEED_RATIO: i32 = 0x0003;
            pub const DOUBLE_JUMP_FRAME: i32 = 0x0004;
            pub const GROUND_VEL: i32 = 0x0005;
            pub const RAR_LENIENCY: i32 = 0x0006;
            pub const CURRENT_MOMENTUM_SPECIALS: i32 = 0x0007;
            pub const DOUBLE_JUMP_TIMER: i32 = 0x0008;
            pub const ROLL_SPEED: i32 = 0x0009;
            pub const LAST_GROUNDED_POS: i32 = 0x000A;
            pub const GET_DIST_TO_FLOOR: i32 = 0x000D;
            pub const ECB_BOTTOM_Y_OFFSET: i32 = 0x000E;
            pub const CURR_DASH_SPEED: i32 = 0x000F;
            pub const MOONWALK_SPEED: i32 = 0x0010;
            pub const ESCAPE_AIR_SLIDE_SPEED_X: i32 = 0x0011;
            pub const ESCAPE_AIR_SLIDE_SPEED_Y: i32 = 0x0012;
            pub const Y_POS: i32 = 0x0013;
            pub const JUMP_SPEED_MAX_MUL: i32 = 0x0014;
            pub const ECB_TOP_Y_OFFSET: i32 = 0x0015;
            pub const LAST_ATTACK_HIT_LOCATION: i32 = 0x0016;
            pub const LAST_ATTACK_HIT_LOCATION_X: i32 = 0x0016;
            pub const LAST_ATTACK_HIT_LOCATION_Y: i32 = 0x0017;
            pub const LAST_ATTACK_HIT_LOCATION_Z: i32 = 0x0018;
            pub const ECB_CENTER_Y_OFFSET: i32 = 0x0019;
            pub const DASH_HIP_OFFSET_X: i32 = 0x0020;
            pub const RUN_HIP_OFFSET_X: i32 = 0x0021;
            pub const ATTACK_S3_CSTICK_X: i32 = 0x0022;
            pub const LAST_RECEIVED_ATTACK_HIT_LOCATION: i32 = 0x0023;
            pub const LAST_RECEIVED_ATTACK_HIT_LOCATION_X: i32 = 0x0023;
            pub const LAST_RECEIVED_ATTACK_HIT_LOCATION_Y: i32 = 0x0024;
            pub const LAST_RECEIVED_ATTACK_HIT_LOCATION_Z: i32 = 0x0025;
            pub const LAND_CANCEL_LAG: i32 = 0x0026;

        }
        pub mod status {
            // flags
            pub const FAF_REACHED: i32 = 0x10FD;
            pub const PREV_AUTOCANCEL_FLAG: i32 = 0x10FE;
            pub const DISABLE_ECB_SHIFT: i32 = 0x10FF;
            pub const IS_DASH_TO_RUN_FRAME: i32 = 0x1000;
            pub const IS_AFTER_DASH_TO_RUN_FRAME: i32 = 0x1001;
            pub const APPLY_DASH_END_SPEED_MUL: i32 = 0x1002;
            pub const ATTACK_DASH_CANCEL_DISABLE: i32 = 0x1000;
            pub const ATTACK_DASH_ENABLE_AIR_FALL: i32 = 0x1001;
            pub const ATTACK_DASH_ENABLE_AIR_CONTINUE: i32 = 0x1002;
            pub const ATTACK_DASH_ENABLE_AIR_DRIFT: i32 = 0x1003;
            pub const ATTACK_DASH_AIR_DRIFT_ENABLED: i32 = 0x1004;
            pub const ATTACK_DASH_ENABLE_AIR_LANDING: i32 = 0x1005;
            pub const SHOULD_WAVELAND: i32 = 0x1000;
            pub const IS_JAB_LOCK_ROLL: i32 = 0x1000;
            pub const IS_SPIKE: i32 = 0x1001;
            pub const DAMAGE_FLY_RESET_TRIGGER: i32 = 0x1002;
            pub const SUICIDE_THROW_CAN_CLATTER: i32 = 0x1000;
            pub const ENABLE_UCF: i32 = 0x1000;
            pub const PUMMEL_OVERRIDE_GLOBAL_STATS: i32 = 0x1000;
            pub const CSTICK_IRAR: i32 = 0x1000;
            pub const FLOAT_INHERIT_AERIAL: i32 = 0x1000;
            pub const IS_TELEPORT_WALL_RIDE: i32 = 0x1000; // Mewtwo, Palutena, Sheik, and Zelda use this
            pub const ENABLE_SPECIAL_WALLJUMP: i32 = 0x1050;
            pub const HIT_EFFECT_DROP_ITEM: i32 = 0x1051;
            pub const SHOULD_HITFALL: i32 = 0x1006;
            pub const NO_POCKET: i32 = 0x1052;
            pub const IS_DASH_CANCEL: i32 = 0x1055;

            // ints

            pub const DOWN_STAND_FB_KIND: i32 = 0x1000;
            pub const FLOAT_FRAME: i32 = 0x1000;
            pub const FLOAT_ENABLE_UNIQ: i32 = 0x1001;
            pub const FLOAT_MTRANS: i32 = 0x1002;
            pub const WARP_EFF_HANDLER: i32 = 0x1000;

            // floats

            pub const INITIAL_KNOCKBACK_VEL_X: i32 = 0x1000;
            pub const INITIAL_KNOCKBACK_VEL_Y: i32 = 0x1001;
            pub const RESTING_HIP_OFFSET_Y: i32 = 0x1000;
            pub const TELEPORT_INITIAL_SPEED_X: i32 = 0x1000;
            pub const TELEPORT_INITIAL_SPEED_Y: i32 = 0x1001;
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
            pub const DISABLE_SPECIAL_LW: i32 = 0x0100;
            pub const CANCEL_SPECIAL_LW: i32 = 0x0101;
            pub const AURA_MAXIMUS: i32 = 0x0102;
            
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

    pub mod mariod_drcapsule {
        pub mod instance {
            // ints
            pub const DAMAGE: i32 = 0x0100;
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

    pub mod mario {
        pub const CAPJUMP: i32 = 0x1E3;
        pub const CAPDIVE: i32 = 0x1E4;
        pub const CAPCATCH: i32 = 0x1E5;
    }

    pub mod mario_captoss {
        pub const START: i32 = 0x0;
        pub const FLY: i32 = 0x1;
        pub const TURN: i32 = 0x2;
        pub const HOP: i32 = 0x3;
        pub const HOLD: i32 = 0x4;
        pub const SWALLOWED: i32 = 0x5;
        pub const JUMP: i32 = 0x6;
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
    pub mod mario {
        pub const CAPTOSS: i32 = 0x6;
    }
}