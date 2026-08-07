use super::*;
use globals::*;

pub static mut FLOAT_OFFSET: usize = 0x4E53C0;

pub static FLOAT_SEARCH_CODE: &[u8] = &[
    0x00, 0x1c, 0x40, 0xf9, 0x08, 0x00, 0x40, 0xf9, 0x03, 0x19, 0x40, 0xf9,
];

pub static mut INT_OFFSET : usize = 0x4E5380;

pub static INT_SEARCH_CODE: &[u8] = &[
    0x00, 0x1c, 0x40, 0xf9, 0x08, 0x00, 0x40, 0xf9, 0x03, 0x11, 0x40, 0xf9,
];

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

#[skyline::hook(offset=FLOAT_OFFSET)]
pub unsafe fn get_param_float_replace(module_accessor: u64, param_type: u64, param_hash: u64) -> f32 {
	let mut boma = *((module_accessor as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor;
	let boma_reference: &mut BattleObjectModuleAccessor = &mut *boma;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if boma_reference.is_fighter() {
        if param_type == hash40("walk_slow_speed_mul") {
            return 0.25;
        }
        if param_type == hash40("walk_accel_add") {
            return 0.0;
        }
        if param_type == hash40("run_accel_add") {
            return 0.0;
        }
        if param_type == hash40("air_accel_x_add") {
            return 0.0;
        }
        if param_type == hash40("damage_fly_top_air_accel_y") { 
            return WorkModule::get_param_float(boma_reference, hash40("air_accel_y"), 0);
        }
        if param_type == hash40("damage_fly_top_speed_y_stable") { 
            return WorkModule::get_param_float(boma_reference, hash40("air_speed_y_stable"), 0);
        }
        if param_type == hash40("dive_speed_y") { 
            return WorkModule::get_param_float(boma_reference, hash40("air_speed_y_stable"), 0);
        }
        if param_type == hash40("air_ground_speed_brake") {
            return 0.0;
        }
        if param_type == hash40("landing_attack_air_frame_n") {
            return 1.0;
        }
        if param_type == hash40("landing_attack_air_frame_f") {
            return 1.0;
        }
        if param_type == hash40("landing_attack_air_frame_b") {
            return 1.0;
        }
        if param_type == hash40("landing_attack_air_frame_hi") {
            return 1.0;
        }
        if param_type == hash40("landing_attack_air_frame_lw") {
            return 1.0;
        }
        if param_type == hash40("landing_frame") {
            return 1.0;
        }
        if param_type == hash40("param_motion") {
            if param_hash == hash40("escape_n_hit_xlu_frame") {
                return 1.0;
            }
            if param_hash == hash40("escape_n_hit_normal_frame") {
                return 20.0;
            }
            if param_hash == hash40("escape_n_penalty_hit_xlu_frame") {
                return 1.0;
            }
            if param_hash == hash40("escape_n_penalty_hit_normal_frame") {
                return 20.0;
            }
            if param_hash == hash40("escape_attack_frame") {
                return 999999.0;
            }
            if param_hash == hash40("escape_n_cancel_frame") {
                return 21.0;
            }
            if param_hash == hash40("escape_f_hit_xlu_frame") {
                return 1.0;
            }
            if param_hash == hash40("escape_f_hit_normal_frame") {
                return 20.0;
            }
            if param_hash == hash40("escape_f_penalty_hit_xlu_frame") {
                return 1.0;
            }
            if param_hash == hash40("escape_f_penalty_hit_normal_frame") {
                return 20.0;
            }
            if param_hash == hash40("escape_f_cancel_frame") {
                return 21.0;
            }
            if param_hash == hash40("escape_b_hit_xlu_frame") {
                return 1.0;
            }
            if param_hash == hash40("escape_b_hit_normal_frame") {
                return 20.0;
            }
            if param_hash == hash40("escape_b_penalty_hit_xlu_frame") {
                return 1.0;
            }
            if param_hash == hash40("escape_b_penalty_hit_normal_frame") {
                return 20.0;
            }
            if param_hash == hash40("escape_b_cancel_frame") {
                return 21.0;
            }
            if param_hash == hash40("escape_air_hit_xlu_frame") {
                return 0.0;
            }
            if param_hash == hash40("escape_air_hit_normal_frame") {
                return 0.0;
            }
            if param_hash == hash40("escape_air_penalty_hit_xlu_frame") {
                return 0.0;
            }
            if param_hash == hash40("escape_air_penalty_hit_normal_frame") {
                return 0.0;
            }
            if param_hash == hash40("escape_air_stiff_start_frame") {
                return 999999.0;
            }
            if param_hash == hash40("escape_air_cancel_frame") {
                return -1.0;
            }
            if param_hash == hash40("escape_air_slide_hit_xlu_frame") {
                return 0.0;
            }
            if param_hash == hash40("escape_air_slide_hit_normal_frame") {
                return 0.0;
            }
            if param_hash == hash40("escape_air_slide_penalty_hit_xlu_frame") {
                return 0.0;
            }
            if param_hash == hash40("escape_air_slide_penalty_hit_normal_frame") {
                return 0.0;
            }
            if param_hash == hash40("escape_air_slide_back_distance") {
                return 0.0;
            }
            if param_hash == hash40("escape_air_slide_speed") {
                return if WorkModule::get_param_float(boma_reference, hash40("run_speed_max"), 0) > WorkModule::get_param_float(boma_reference, hash40("air_speed_x_stable"), 0) {WorkModule::get_param_float(boma_reference, hash40("run_speed_max"), 0)} else {WorkModule::get_param_float(boma_reference, hash40("air_speed_x_stable"), 0)};
            }
            if param_hash == hash40("escape_air_slide_distance") {
                return (if WorkModule::get_param_float(boma_reference, hash40("run_speed_max"), 0) > WorkModule::get_param_float(boma_reference, hash40("air_speed_x_stable"), 0) {WorkModule::get_param_float(boma_reference, hash40("run_speed_max"), 0)} else {WorkModule::get_param_float(boma_reference, hash40("air_speed_x_stable"), 0)}) * 10.0;
            }
            if param_hash == hash40("escape_air_slide_penalty_speed") {
                return if WorkModule::get_param_float(boma_reference, hash40("run_speed_max"), 0) > WorkModule::get_param_float(boma_reference, hash40("air_speed_x_stable"), 0) {WorkModule::get_param_float(boma_reference, hash40("run_speed_max"), 0)} else {WorkModule::get_param_float(boma_reference, hash40("air_speed_x_stable"), 0)};
            }
            if param_hash == hash40("escape_air_slide_penalty_distance") {
                return (if WorkModule::get_param_float(boma_reference, hash40("run_speed_max"), 0) > WorkModule::get_param_float(boma_reference, hash40("air_speed_x_stable"), 0) {WorkModule::get_param_float(boma_reference, hash40("run_speed_max"), 0)} else {WorkModule::get_param_float(boma_reference, hash40("air_speed_x_stable"), 0)}) * 10.0;
            }
            if param_hash == hash40("escape_air_slide_end_speed") {
                return WorkModule::get_param_float(boma_reference, hash40("air_speed_x_stable"), 0);
            }
            if param_hash == hash40("escape_air_slide_stiff_start_frame") {
                return 999999.0;
            }
            if param_hash == hash40("escape_air_slide_cancel_frame") {
                return -1.0;
            }
            if param_hash == hash40("landing_frame_escape_air_slide_max") {
                return 10.0;
            }
            if param_hash == hash40("landing_frame_escape_air_slide") {
                return 1.0;
            }
            if param_hash == hash40("landing_speed_mul_escape_air_slide") {
                return 1.0;
            }
        }
        if param_type == hash40("common") {
            if param_hash == hash40("ground_speed_limit") {
                if boma_reference.is_status(*FIGHTER_STATUS_KIND_LANDING) {
                    return WorkModule::get_param_float(boma_reference, hash40("run_speed_max"), 0);
                }
            }
        }
        if boma_reference.kind() == *FIGHTER_KIND_IKE {
            if param_type == hash40("param_special_s") { 
                if param_hash == hash40("special_s_ground_dash_spd_mul") {
                    if VarModule::is_flag(boma_reference.object(), vars::ike::instance::STORED_AETHER) {
                        return 0.075;
                    }
                }
                if param_hash == hash40("special_s_air_dash_spd_mul") {
                    if VarModule::is_flag(boma_reference.object(), vars::ike::instance::STORED_AETHER) {
                        return 0.075;
                    }
                }
                if param_hash == hash40("special_s_ground_dash_brake_x") {
                    if VarModule::is_flag(boma_reference.object(), vars::ike::instance::STORED_AETHER) {
                        return 0.075;
                    }
                }
                if param_hash == hash40("special_s_air_dash_brake_x") {
                    if VarModule::is_flag(boma_reference.object(), vars::ike::instance::STORED_AETHER) {
                        return 0.075;
                    }
                }
                if param_hash == hash40("special_s_atk_power_charge_mul") {
                    if VarModule::is_flag(boma_reference.object(), vars::ike::instance::STORED_AETHER) {
                        return 0.15;
                    }
                }
            }
            if param_type == hash40("param_special_hi") {
                if param_hash == hash40("turn_after_flip_y_mul") {
                    if VarModule::is_flag(boma_reference.object(), vars::ike::instance::STORED_AETHER) {
                        return 1.5;
                    }
                }
                if param_hash == hash40("jump_speed_mul") {
                    if VarModule::is_flag(boma_reference.object(), vars::ike::instance::STORED_AETHER) {
                        return 1.5;
                    }
                }
            }
            if param_type == hash40("param_special_lw") {
                if param_hash == hash40("special_lw_attack_power_limit") {
                    if VarModule::is_flag(boma_reference.object(), vars::ike::instance::STORED_AETHER) {
                        return 24.0;
                    }
                }
            }
        }
        if boma_reference.kind() == *FIGHTER_KIND_SAMUS {
            if param_type == hash40("ground_brake") {
                if VarModule::is_flag(boma_reference.object(), vars::samus::instance::SPEEDBOOSTER_ON) {
                    return 0.15;
                }
            }
            if param_type == hash40("dash_speed") {
                if VarModule::is_flag(boma_reference.object(), vars::samus::instance::SPEEDBOOSTER_ON) {
                    return 3.0;
                }
            }
            if param_type == hash40("run_accel_mul") {
                if VarModule::is_flag(boma_reference.object(), vars::samus::instance::SPEEDBOOSTER_ON) {
                    return 3.0;
                }
            }
            if param_type == hash40("run_speed_max") {
                if VarModule::is_flag(boma_reference.object(), vars::samus::instance::SPEEDBOOSTER_ON) {
                    return 3.0;
                }
            }
            if param_type == hash40("jump_speed_x_max") {
                if VarModule::is_flag(boma_reference.object(), vars::samus::instance::SPEEDBOOSTER_ON) {
                    return 3.0;
                }
            }
            if param_type == hash40("jump_y") {
                if VarModule::is_flag(boma_reference.object(), vars::samus::instance::SPEEDBOOSTER_ON) {
                    return 30.0;
                }
            }
            if param_type == hash40("mini_jump_y") {
                if VarModule::is_flag(boma_reference.object(), vars::samus::instance::SPEEDBOOSTER_ON) {
                    return 30.0;
                }
            }
            if param_type == hash40("jump_aerial_y") {
                if VarModule::is_flag(boma_reference.object(), vars::samus::instance::SPEEDBOOSTER_ON) {
                    return 30.0;
                }
            }
            if param_type == hash40("air_accel_x_mul") {
                if VarModule::is_flag(boma_reference.object(), vars::samus::instance::SPEEDBOOSTER_ON) {
                    return 0.15;
                }
            }
            if param_type == hash40("air_speed_x_stable") {
                if VarModule::is_flag(boma_reference.object(), vars::samus::instance::SPEEDBOOSTER_ON) {
                    return 3.0;
                }
            }
            if param_type == hash40("air_brake_x") {
                if VarModule::is_flag(boma_reference.object(), vars::samus::instance::SPEEDBOOSTER_ON) {
                    return 0.15;
                }
            }
            if param_type == hash40("air_accel_y") {
                if VarModule::is_flag(boma_reference.object(), vars::samus::instance::SPEEDBOOSTER_ON) {
                    return 0.15;
                }
            }
            if param_type == hash40("air_speed_y_stable") {
                if VarModule::is_flag(boma_reference.object(), vars::samus::instance::SPEEDBOOSTER_ON) {
                    return 3.0;
                }
            }
            if param_type == hash40("air_brake_y") {
                if VarModule::is_flag(boma_reference.object(), vars::samus::instance::SPEEDBOOSTER_ON) {
                    return 0.15;
                }
            }
            if param_type == hash40("weight") {
                if VarModule::is_flag(boma_reference.object(), vars::samus::instance::SPEEDBOOSTER_ON) {
                    return 60.0;
                }
            }
            if param_type == hash40("cliff_jump_speed_x") {
                if VarModule::is_flag(boma_reference.object(), vars::samus::instance::SPEEDBOOSTER_ON) {
                    return 3.0;
                }
            }
            if param_type == hash40("cliff_jump_y") {
                if VarModule::is_flag(boma_reference.object(), vars::samus::instance::SPEEDBOOSTER_ON) {
                    return 30.0;
                }
            }
            if param_type == hash40("param_special_n") {
                if param_hash == hash40("cshot_charge_frame") {
                    if VarModule::is_flag(boma_reference.object(), vars::samus::instance::ICE_MODE) {
                        return 30.0;
                    }
                }
            }
        }
    } else if boma_reference.is_weapon() {
        if boma_reference.kind() == *WEAPON_KIND_SAMUS_BOMB {
            if param_type == hash40("param_bomb") {
                if param_hash == hash40("accele_gravity") {
                    if VarModule::is_flag(boma_reference.get_owner_boma().object(), vars::samus::instance::ICE_MODE) {
                        return 0.015;
                    }
                }
            }
        }
    } else {}

    original!()(module_accessor, param_type, param_hash)
}

#[skyline::hook(offset=INT_OFFSET)]
pub unsafe fn get_param_int_replace(module_accessor: u64, param_type: u64, param_hash: u64) -> i32 {
	let mut boma = *((module_accessor as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor;
	let boma_reference = &mut *boma;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if boma_reference.is_fighter() {
        if param_type == hash40("jump_squat_frame") {
            return 1;
        }
        if param_type == hash40("landing_heavy_frame") {
            return 1;
        }
        if param_type == hash40("param_motion") {
            if param_hash == hash40("escape_air_slide_back_end_frame") {
                return 0;
            }
        }
    } else if boma_reference.is_weapon() {} else {}

    original!()(module_accessor, param_type, param_hash)
}

pub fn install() {
	unsafe {
        let text_ptr = getRegionAddress(Region::Text) as *const u8;
        let text_size = (getRegionAddress(Region::Rodata) as usize) - (text_ptr as usize);
        let text = std::slice::from_raw_parts(text_ptr, text_size);
        if let Some(offset) = find_subsequence(text, FLOAT_SEARCH_CODE) {
            FLOAT_OFFSET = offset;
        }
        if let Some(offset) = find_subsequence(text, INT_SEARCH_CODE) {
            INT_OFFSET = offset;
        }
    }
	skyline::install_hooks!(get_param_float_replace);
	skyline::install_hooks!(get_param_int_replace);
}