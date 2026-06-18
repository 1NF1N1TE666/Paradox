use super::*;

unsafe extern "C" fn attack_dash_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.stick_y() < -0.4 {
        fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_LW3.into(), true.into());
        0.into()
    } else {
        smashline::original_status(Pre, fighter, *FIGHTER_STATUS_KIND_ATTACK_DASH)(fighter)
    }
}

unsafe extern "C" fn attack_lw3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_lw3"), 0.0, 1.0, false, 0.0, false, false);
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    let lr = fighter.lr();
    if VarModule::is_flag(fighter.battle_object, vars::samus::instance::SPEEDBOOSTER_ON) {
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 3.0, 0.0);
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 3.0 * lr, 0.0);
        JostleModule::set_status(fighter.module_accessor, false);
        VarModule::off_flag(fighter.battle_object, vars::samus::status::ATTACK_LW3_CHECK_CEIL);
        VarModule::set_int(fighter.object(), vars::samus::instance::SPEEDBOOSTER_STICK_TIMER, 20);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(attack_lw3_main_loop as *const () as _))
}

pub unsafe fn attack_lw3_main_loop(fighter: &mut L2CFighterCommon) -> bool {
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            fighter.change_status(FIGHTER_STATUS_KIND_SQUAT_WAIT.into(), true.into());
        } else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), true.into());
        }
        return true.into()
    } else if fighter.is_situation(*SITUATION_KIND_GROUND) && GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_UP as u32) && VarModule::is_flag(fighter.battle_object, vars::samus::status::ATTACK_LW3_CHECK_CEIL)  {
        fighter.change_status(FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW.into(), true.into());
        return true.into()
    } else if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() || fighter.sub_air_check_fall_common().get_bool() {
            return true.into()
        }
    } else {}
    if fighter.global_table[SITUATION_KIND].get_i32() != fighter.global_table[PREV_SITUATION_KIND].get_i32() {
        if !fighter.is_situation(*SITUATION_KIND_GROUND) {
            MotionModule::add_motion_2nd(fighter.module_accessor, Hash40::new("fall"), 0.0, 0.0, false, 0.0);
            MotionModule::set_weight(fighter.module_accessor, 1.0, false);
            AttackModule::clear_all(fighter.module_accessor);
            HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
            slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 0);
            let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN);
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            fighter.set_situation(SITUATION_KIND_AIR.into());
            if VarModule::is_flag(fighter.battle_object, vars::samus::instance::SPEEDBOOSTER_ON) {
                sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 3.0, 0.0);
                sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, speed_x, 0.0);
            }
        } else {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), true.into());
            return true.into()
        }
    } else if !fighter.is_situation(*SITUATION_KIND_GROUND) && !fighter.is_prev_situation(*SITUATION_KIND_GROUND) && MotionModule::frame(fighter.module_accessor) > 8.0 {
        let weight = MotionModule::weight(fighter.module_accessor) -0.066;
        if weight > 0.0 {
            MotionModule::set_weight(fighter.module_accessor, weight, false);
        } else if weight <= 0.0 {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), true.into());
            return true.into()
        } else {}
    } else {}
    return false.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_ATTACK_DASH, attack_dash_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK_LW3, attack_lw3_main);
}