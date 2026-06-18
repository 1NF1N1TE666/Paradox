use super::*;

pub unsafe extern "C" fn special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.is_flag(*FIGHTER_FOX_INSTANCE_WORK_ID_FLAG_REFLECTOR_LANDING) {
        let stop_y_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw"), hash40("reflector_air_stop_y_frame"));
        fighter.set_int(stop_y_frame, *FIGHTER_FOX_REFLECTOR_STATUS_WORK_ID_INT_STOP_Y_FRAME);
    } else {
        fighter.set_int(0, *FIGHTER_FOX_REFLECTOR_STATUS_WORK_ID_INT_STOP_Y_FRAME);
    }

    special_lw_motion_helper(fighter);
    fighter.main_shift(special_lw_main_loop)
}

unsafe extern "C" fn special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.check_dash_cancel() || fighter.check_jump_cancel(true) || fighter.check_airdash_cancel() {
        return 0.into();
    }

    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_req(*FIGHTER_WOLF_STATUS_KIND_SPECIAL_LW_LOOP, false);
    }

    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        special_lw_motion_helper(fighter);
    }

    0.into()
}

unsafe extern "C" fn special_lw_motion_helper(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_FOX_REFLECTOR_STATUS_WORK_ID_FLAG_CONTINUE) {
            if PostureModule::lr(fighter.module_accessor) == -1.0 {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_start_l"), 0.0, 1.0, false, 0.0, false, false);
            } else {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_start"), 0.0, 1.0, false, 0.0, false, false);
            }
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_FOX_REFLECTOR_STATUS_WORK_ID_FLAG_CONTINUE);
        } else {
            if PostureModule::lr(fighter.module_accessor) == -1.0 {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw_start_l"), -1.0, 1.0, 0.0, false, false);
            } else {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw_start"), -1.0, 1.0, 0.0, false, false);
            }
        }
        fighter.set_situation_keep(L2CValue::I32(*SITUATION_KIND_AIR), 1.into());
    } else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_FOX_REFLECTOR_STATUS_WORK_ID_FLAG_CONTINUE) {    
            if PostureModule::lr(fighter.module_accessor) == -1.0 {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_start_l"), 0.0, 1.0, false, 0.0, false, false);
            } else {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_start"), 0.0, 1.0, false, 0.0, false, false);
            }
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_FOX_REFLECTOR_STATUS_WORK_ID_FLAG_CONTINUE);
        } else {
            if PostureModule::lr(fighter.module_accessor) == -1.0 {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw_start_l"), -1.0, 1.0, 0.0, false, false);
            } else {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw_start"), -1.0, 1.0, 0.0, false, false);
            }
        }
        fighter.set_situation_keep(L2CValue::I32(*SITUATION_KIND_GROUND), 1.into());
    }
}

pub unsafe extern "C" fn special_lw_hit_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ControlModule::reset_flick_x(fighter.module_accessor);
    ControlModule::reset_flick_y(fighter.module_accessor);
    fighter.on_flag(*FIGHTER_FOX_REFLECTOR_STATUS_WORK_ID_FLAG_HIT_TO_RESTART);
    special_lw_hit_motion_helper(fighter);
    fighter.main_shift(special_lw_hit_main_loop)
}

unsafe extern "C" fn special_lw_hit_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.check_dash_cancel() || fighter.check_jump_cancel(true) || fighter.check_airdash_cancel() {
            return 0.into();
        }
        if fighter.is_button_off(Buttons::Special) {
            fighter.change_status(FIGHTER_WOLF_STATUS_KIND_SPECIAL_LW_END.into(), false.into()); 
            return 0.into();
        }
        fighter.change_status_req(*FIGHTER_WOLF_STATUS_KIND_SPECIAL_LW_LOOP, false);
        return 0.into();
    }
    
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        special_lw_hit_motion_helper(fighter);
    }

    return 0.into();
}

unsafe extern "C" fn special_lw_hit_motion_helper(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_FOX_REFLECTOR_STATUS_WORK_ID_FLAG_CONTINUE) {
            if PostureModule::lr(fighter.module_accessor) == -1.0 {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_hit_l"), 0.0, 1.0, false, 0.0, false, false);
            } else {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_hit"), 0.0, 1.0, false, 0.0, false, false);
            }
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_FOX_REFLECTOR_STATUS_WORK_ID_FLAG_CONTINUE);
        } else {
            if PostureModule::lr(fighter.module_accessor) == -1.0 {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw_hit_l"), -1.0, 1.0, 0.0, false, false);
            } else {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw_hit"), -1.0, 1.0, 0.0, false, false);
            }
        }
    } else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_FOX_REFLECTOR_STATUS_WORK_ID_FLAG_CONTINUE) {    
            if PostureModule::lr(fighter.module_accessor) == -1.0 {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_hit_l"), 0.0, 1.0, false, 0.0, false, false);
            } else {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_hit"), 0.0, 1.0, false, 0.0, false, false);
            }
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_FOX_REFLECTOR_STATUS_WORK_ID_FLAG_CONTINUE);
        } else {
            if PostureModule::lr(fighter.module_accessor) == -1.0 {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw_hit_l"), -1.0, 1.0, 0.0, false, false);
            } else {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw_hit"), -1.0, 1.0, 0.0, false, false);
            }
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_main);
    agent.status(Main, *FIGHTER_WOLF_STATUS_KIND_SPECIAL_LW_HIT, special_lw_hit_main);
}