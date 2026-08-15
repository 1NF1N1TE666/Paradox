use super::*;
use globals::*;

pub extern "C" fn left_stick_flick_counter(fighter: &mut L2CFighterCommon) {
    unsafe {
        if fighter.left_stick_x() == 0.0 {
            VarModule::set_int(fighter.battle_object, vars::common::instance::LEFT_STICK_FLICK_X, u8::MAX as i32 - 1);
        } else if fighter.left_stick_x().signum() != fighter.prev_left_stick_x().signum() || fighter.prev_left_stick_x() == 0.0 {
            VarModule::set_int(fighter.battle_object, vars::common::instance::LEFT_STICK_FLICK_X, 0);
        } else {
            VarModule::inc_int(fighter.battle_object, vars::common::instance::LEFT_STICK_FLICK_X);
        }
        
        if fighter.left_stick_y() == 0.0 {
            VarModule::set_int(fighter.battle_object, vars::common::instance::LEFT_STICK_FLICK_Y, u8::MAX as i32 - 1);
        } else if fighter.left_stick_y().signum() != fighter.prev_left_stick_y().signum()
        || fighter.prev_left_stick_y() == 0.0 {
            VarModule::set_int(fighter.battle_object, vars::common::instance::LEFT_STICK_FLICK_Y, 0);
        } else {
            VarModule::inc_int(fighter.battle_object, vars::common::instance::LEFT_STICK_FLICK_Y);
        }
    }
}

pub extern "C" fn right_stick_flick_counter(fighter: &mut L2CFighterCommon) {
    unsafe {
        if fighter.right_stick_x() == 0.0 {
            VarModule::set_int(fighter.battle_object, vars::common::instance::RIGHT_STICK_FLICK_X, u8::MAX as i32 - 1);
        } else if fighter.right_stick_x().signum() != fighter.prev_right_stick_x().signum() || fighter.prev_right_stick_x() == 0.0 {
            VarModule::set_int(fighter.battle_object, vars::common::instance::RIGHT_STICK_FLICK_X, 0);
        } else {
            VarModule::inc_int(fighter.battle_object, vars::common::instance::RIGHT_STICK_FLICK_X);
        }
        
        if fighter.right_stick_y() == 0.0 {
            VarModule::set_int(fighter.battle_object, vars::common::instance::RIGHT_STICK_FLICK_Y, u8::MAX as i32 - 1);
        } else if fighter.right_stick_y().signum() != fighter.prev_right_stick_y().signum() || fighter.prev_right_stick_y() == 0.0 {
            VarModule::set_int(fighter.battle_object, vars::common::instance::RIGHT_STICK_FLICK_Y, 0);
        } else {
            VarModule::inc_int(fighter.battle_object, vars::common::instance::RIGHT_STICK_FLICK_Y);
        }
    }
}

pub unsafe extern "C" fn init(fighter: &mut L2CFighterCommon) {
    VarModule::off_flag(fighter.object(), vars::common::instance::IS_INIT);
    VarModule::off_flag(fighter.object(), vars::common::instance::BURST_LIMIT);
}

pub unsafe extern "C" fn opff(fighter: &mut L2CFighterCommon) {
    fighter.paradox_funcs();
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sys_line_system_control_fighter)]
pub unsafe fn sys_line_system_control_fighter_hook(fighter: &mut L2CFighterCommon) -> L2CValue {
    left_stick_flick_counter(fighter);
    right_stick_flick_counter(fighter);

    original!()(fighter)
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sys_line_system_control_fighter_hook
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
    smashline::Agent::new("fighter")
        .on_start(init)
        .on_line(Main, opff)
        .install();
}