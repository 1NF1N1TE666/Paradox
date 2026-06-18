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
        if param_type == hash40("common") {
            if param_hash == hash40("ground_speed_limit") {
                return WorkModule::get_param_float(boma_reference, hash40("run_speed_max"), 0);
            }
            if param_hash == hash40("air_speed_x_limit") {
                return if boma_reference.kind() == *FIGHTER_KIND_LITTLEMAC {
                    WorkModule::get_param_float(boma_reference, hash40("run_speed_max"), 0)
                } else if WorkModule::get_param_float(boma_reference, hash40("jump_speed_x_max"), 0) > WorkModule::get_param_float(boma_reference, hash40("air_speed_x_stable"), 0) {
                    WorkModule::get_param_float(boma_reference, hash40("jump_speed_x_max"), 0)
                } else {
                    WorkModule::get_param_float(boma_reference, hash40("air_speed_x_stable"), 0)
                };
            }
            if param_hash == hash40("air_speed_y_limit") {
                return WorkModule::get_param_float(boma_reference, hash40("dive_speed_y"), 0);
            }
        }
        if boma_reference.kind() == *FIGHTER_KIND_IKE {
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
                    return 0.03;
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
            if param_type == hash40("jump_initial_y") {
                if VarModule::is_flag(boma_reference.object(), vars::samus::instance::SPEEDBOOSTER_ON) {
                    return 15.0;
                }
            }
            if param_type == hash40("jump_y") {
                if VarModule::is_flag(boma_reference.object(), vars::samus::instance::SPEEDBOOSTER_ON) {
                    return 30.0;
                }
            }
            if param_type == hash40("mini_jump_y") {
                if VarModule::is_flag(boma_reference.object(), vars::samus::instance::SPEEDBOOSTER_ON) {
                    return 15.0;
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
                    return 0.03;
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
            if param_type == hash40("damage_fly_top_air_accel_y") {
                if VarModule::is_flag(boma_reference.object(), vars::samus::instance::SPEEDBOOSTER_ON) {
                    return 0.15;
                }
            }
            if param_type == hash40("damage_fly_top_speed_y_stable") {
                if VarModule::is_flag(boma_reference.object(), vars::samus::instance::SPEEDBOOSTER_ON) {
                    return 3.0;
                }
            }
            if param_type == hash40("air_brake_y") {
                if VarModule::is_flag(boma_reference.object(), vars::samus::instance::SPEEDBOOSTER_ON) {
                    return 0.03;
                }
            }
            if param_type == hash40("dive_speed_y") {
                if VarModule::is_flag(boma_reference.object(), vars::samus::instance::SPEEDBOOSTER_ON) {
                    return 6.0;
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
                        return 60.0;
                    }
                }
            }
        }
    } else if boma_reference.is_weapon() {
        if boma_reference.kind() == *WEAPON_KIND_SAMUS_BOMB {
            if param_type == hash40("param_bomb") {
                if param_hash == hash40("firing_speed_y") {
                    if VarModule::is_flag(boma_reference.get_owner_boma().object(), vars::samus::instance::ICE_MODE) {
                        return 1.0;
                    }
                }
                if param_hash == hash40("accele_gravity") {
                    if VarModule::is_flag(boma_reference.get_owner_boma().object(), vars::samus::instance::ICE_MODE) {
                        return 0.025;
                    }
                }
                if param_hash == hash40("0x1c309814d2") {
                    if VarModule::is_flag(boma_reference.get_owner_boma().object(), vars::samus::instance::ICE_MODE) {
                        return 1.0;
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

    if boma_reference.is_fighter() {} else if boma_reference.is_weapon() {
        if boma_reference.kind() == *WEAPON_KIND_SAMUS_BOMB {
            if param_type == hash40("param_bomb") {
                if param_hash == hash40("life") {
                    if VarModule::is_flag(boma_reference.get_owner_boma().object(), vars::samus::instance::ICE_MODE) {
                        return 120;
                    }
                }
                if param_hash == hash40("life_flash") {
                    if VarModule::is_flag(boma_reference.get_owner_boma().object(), vars::samus::instance::ICE_MODE) {
                        return 90;
                    }
                }
            }
        }
    } else {}

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