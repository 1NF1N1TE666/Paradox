use super::*;

unsafe extern "C" fn special_lw_start_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_NONE as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        0,
        0,
        0,
        *FS_SUCCEEDS_KEEP_VISIBILITY
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_SHOOT) as u64,
        *FIGHTER_STATUS_ATTR_DISABLE_SWIM as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );

    0.into()
}

unsafe extern "C" fn special_lw_start_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    if VarModule::is_flag(fighter.battle_object, vars::samus::instance::SPEEDBOOSTER_ON) {
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 3.0, 0.0);
    }
    let speed = if VarModule::is_flag(fighter.battle_object, vars::samus::instance::SPEEDBOOSTER_ON) {3.0} else {1.5} as f32;
    sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, speed, 0.0);
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw"), 0.0, 1.0, false, 0.0, false, false);
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        sv_kinetic_energy!(clear_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    } else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw"), 0.0, 1.0, false, 0.0, false, false);
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    VarModule::off_flag(fighter.battle_object, vars::samus::status::SPECIAL_LW_BOMB_JUMP_ON);
    VarModule::off_flag(fighter.battle_object, vars::samus::status::SPECIAL_LW_BOMB_JUMP_HOP);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_lw_start_main_loop as *const () as _))
}

pub unsafe fn special_lw_start_main_loop(fighter: &mut L2CFighterCommon) -> bool {
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW.into(), true.into()); //loop
        return true.into()
    } else {
        if VarModule::is_flag(fighter.battle_object, vars::samus::status::SPECIAL_LW_BOMB_JUMP_HOP) {
            VarModule::off_flag(fighter.battle_object, vars::samus::status::SPECIAL_LW_BOMB_JUMP_HOP);
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("special_air_lw"), -1.0, 1.0, 0.0);
                GroundModule::set_attach_ground(fighter.module_accessor, false);
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                fighter.set_situation(SITUATION_KIND_AIR.into());
                KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                let brake = if VarModule::is_flag(fighter.battle_object, vars::samus::instance::SPEEDBOOSTER_ON) {0.15} else {0.075} as f32;
                sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, brake, 0.0);
                let accel_x = if VarModule::is_flag(fighter.battle_object, vars::samus::instance::SPEEDBOOSTER_ON) {0.15} else {0.075} as f32;
                sv_kinetic_energy!(set_accel_x_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, accel_x);
            }
            let jump = if VarModule::is_flag(fighter.battle_object, vars::samus::instance::SPEEDBOOSTER_ON) {1.5} else {3.0} as f32;
            sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, jump, 0.0);
        } else if fighter.global_table[SITUATION_KIND].get_i32() != fighter.global_table[PREV_SITUATION_KIND].get_i32() {
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("special_lw"), -1.0, 1.0, 0.0);
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                let brake = if VarModule::is_flag(fighter.battle_object, vars::samus::instance::SPEEDBOOSTER_ON) {0.15} else {0.075} as f32;
                sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, brake, 0.0);
                let accel_x = if VarModule::is_flag(fighter.battle_object, vars::samus::instance::SPEEDBOOSTER_ON) {0.15} else {0.075} as f32;
                sv_kinetic_energy!(set_accel_x_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, accel_x);
            } else {
                MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("special_air_lw"), -1.0, 1.0, 0.0);
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                sv_kinetic_energy!(clear_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                let brake = if VarModule::is_flag(fighter.battle_object, vars::samus::instance::SPEEDBOOSTER_ON) {0.15} else {0.075} as f32;
                sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, brake, 0.0);
                let accel_x = if VarModule::is_flag(fighter.battle_object, vars::samus::instance::SPEEDBOOSTER_ON) {0.15} else {0.075} as f32;
                sv_kinetic_energy!(set_accel_x_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, accel_x);
            }
        } else {}
        return false.into()
    }
}

unsafe extern "C" fn special_lw_start_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn special_lw_loop_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_NONE as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        0,
        0,
        0,
        *FS_SUCCEEDS_KEEP_VISIBILITY
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_SHOOT) as u64,
        *FIGHTER_STATUS_ATTR_DISABLE_SWIM as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn special_lw_loop_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {
        fighter.set_situation(SITUATION_KIND_GROUND.into());
    }
    let speed_x = if VarModule::is_flag(fighter.battle_object, vars::samus::instance::SPEEDBOOSTER_ON) {3.0 * fighter.lr()} else {KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN)};
    let motion_rate = speed_x * fighter.lr() * 15.0;
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_loop"), 0.0, motion_rate, false, 0.0, false, false);
    sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST, 0.0, 0.0, 0.0, 0.0);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        sv_kinetic_energy!(clear_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        let brake = if VarModule::is_flag(fighter.battle_object, vars::samus::instance::SPEEDBOOSTER_ON) {0.15} else {0.075} as f32;
        sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, brake, 0.0);
        let accel_x = if VarModule::is_flag(fighter.battle_object, vars::samus::instance::SPEEDBOOSTER_ON) {0.15} else {0.075} as f32;
        sv_kinetic_energy!(set_accel_x_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, accel_x);
    } else {
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        let brake = if VarModule::is_flag(fighter.battle_object, vars::samus::instance::SPEEDBOOSTER_ON) {0.15} else {0.075} as f32;
        sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, brake, 0.0);
        let accel_x = if VarModule::is_flag(fighter.battle_object, vars::samus::instance::SPEEDBOOSTER_ON) {0.15} else {0.075} as f32;
        sv_kinetic_energy!(set_accel_x_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, accel_x);
    }
    if VarModule::is_flag(fighter.battle_object, vars::samus::instance::SPEEDBOOSTER_ON) {
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 3.0, 0.0);
    } else {
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 1.5, 0.0);
    }
    let gravity = if VarModule::is_flag(fighter.battle_object, vars::samus::instance::SPEEDBOOSTER_ON) {0.15} else {0.075} as f32;
    sv_kinetic_energy!(set_accel_x_add, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0);
    sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -gravity);
    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, speed_x, 0.0);
    VarModule::on_flag(fighter.battle_object, vars::samus::status::SPECIAL_LW_BOMB_JUMP_ON);
    VarModule::off_flag(fighter.battle_object, vars::samus::status::SPECIAL_LW_BOMB_JUMP_HOP);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_lw_loop_main_loop as *const () as _))
}

pub unsafe fn special_lw_loop_main_loop(fighter: &mut L2CFighterCommon) -> bool {
    let motion_rate_prev = MotionModule::rate(fighter.module_accessor);
    let motion_frame_end = MotionModule::end_frame(fighter.module_accessor);
    let motion_frame_prev = MotionModule::prev_frame(fighter.module_accessor);
    let motion_frame_curr = MotionModule::frame(fighter.module_accessor);
    let mut motion_rate;
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        motion_rate = fighter.lr() * 15.0 * KineticModule::get_sum_speed(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    } else {
        let motion_rate_target = fighter.stick_x() * 15.0 * fighter.lr();

        if motion_rate_target > motion_rate_prev {
            if motion_rate_prev > 0.0 {
                motion_rate = motion_rate_prev + 0.5;
            } else {
                motion_rate = motion_rate_prev + 0.25;
            }
            if motion_rate_target < motion_rate {
                motion_rate = motion_rate_target;
            }
        } else if motion_rate_target < motion_rate_prev {
            if motion_rate_prev < 0.0 {
                motion_rate = motion_rate_prev - 0.5;
            } else {
                motion_rate = motion_rate_prev - 0.25;
            }
            if motion_rate_target > motion_rate {
                motion_rate = motion_rate_target;
            }
        } else {
            motion_rate = motion_rate_prev;
        }
    }
    MotionModule::set_rate(fighter.module_accessor, motion_rate);
    if motion_rate > 0.0 && motion_frame_curr >= motion_frame_end {
        let frame = motion_frame_prev + motion_rate_prev + -motion_frame_end;
        MotionModule::set_frame(fighter.module_accessor, frame, false);
    } else if motion_rate < 0.0 && motion_frame_curr <= 0.0 {
        let frame = motion_frame_prev + motion_rate_prev + motion_frame_end;
        MotionModule::set_frame(fighter.module_accessor, frame, false);
    }
    if VarModule::is_flag(fighter.battle_object, vars::samus::status::SPECIAL_LW_BOMB_JUMP_HOP) {
        VarModule::off_flag(fighter.battle_object, vars::samus::status::SPECIAL_LW_BOMB_JUMP_HOP);
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.set_situation(SITUATION_KIND_AIR.into());
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            GroundModule::set_attach_ground(fighter.module_accessor, false);
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            let brake = if VarModule::is_flag(fighter.battle_object, vars::samus::instance::SPEEDBOOSTER_ON) {0.15} else {0.075} as f32;
            sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, brake, 0.0);
            let accel_x = if VarModule::is_flag(fighter.battle_object, vars::samus::instance::SPEEDBOOSTER_ON) {0.15} else {0.075} as f32;
            sv_kinetic_energy!(set_accel_x_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, accel_x);
        }
        let speed = if VarModule::is_flag(fighter.battle_object, vars::samus::instance::SPEEDBOOSTER_ON) {1.5} else {3.0} as f32;
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, speed, 0.0);
    } else if (fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND || fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND) && fighter.is_button_trigger(Buttons::Jump) {
        LANDING_EFFECT(fighter, Hash40::new("sys_jump_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::set_attach_ground(fighter.module_accessor, false);
        let speed = if VarModule::is_flag(fighter.battle_object, vars::samus::instance::SPEEDBOOSTER_ON) {1.5} else {3.0} as f32;
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, speed, 0.0);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        let brake = if VarModule::is_flag(fighter.battle_object, vars::samus::instance::SPEEDBOOSTER_ON) {0.15} else {0.075} as f32;
        sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, brake, 0.0);
        let accel_x = if VarModule::is_flag(fighter.battle_object, vars::samus::instance::SPEEDBOOSTER_ON) {0.15} else {0.075} as f32;
        sv_kinetic_energy!(set_accel_x_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, accel_x);
    } else if fighter.global_table[SITUATION_KIND].get_i32() != fighter.global_table[PREV_SITUATION_KIND].get_i32() {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            PLAY_LANDING_SE(fighter, Hash40::new("se_samus_landing01"));
            LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 5, 0, 0, 0, false);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            let brake = if VarModule::is_flag(fighter.battle_object, vars::samus::instance::SPEEDBOOSTER_ON) {0.15} else {0.075} as f32;
            sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, brake, 0.0);
            let accel_x = if VarModule::is_flag(fighter.battle_object, vars::samus::instance::SPEEDBOOSTER_ON) {0.15} else {0.075} as f32;
            sv_kinetic_energy!(set_accel_x_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, accel_x);
        } else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            sv_kinetic_energy!(clear_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            let brake = if VarModule::is_flag(fighter.battle_object, vars::samus::instance::SPEEDBOOSTER_ON) {0.15} else {0.075} as f32;
            sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, brake, 0.0);
            let accel_x = if VarModule::is_flag(fighter.battle_object, vars::samus::instance::SPEEDBOOSTER_ON) {0.15} else {0.075} as f32;
            sv_kinetic_energy!(set_accel_x_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, accel_x);
        }
    } else {}
    if fighter.stick_x().abs() > 0.5 
    && GroundModule::get_correct(fighter.module_accessor) == *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    } else if GroundModule::get_correct(fighter.module_accessor) == *GROUND_CORRECT_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
    } else {}
    if fighter.is_button_trigger(Buttons::Attack) {
        if ArticleModule::get_active_num(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_BOMB) < fighter.get_param_int("param_special_lw", "bomb_max_req") as u64 {
            ArticleModule::generate_article_enable(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_BOMB, false, -1);
            ArticleModule::shoot_exist(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_BOMB, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        }
    }
    if fighter.is_button_trigger(Buttons::Special) {
        fighter.change_status(FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW.into(), true.into());
        return true.into()
    } else {
        return false.into()
    }
}

unsafe extern "C" fn special_lw_loop_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn special_lw_end_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_NONE as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        0,
        0,
        0,
        *FS_SUCCEEDS_KEEP_VISIBILITY
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW) as u64,
        *FIGHTER_STATUS_ATTR_DISABLE_SWIM as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
    let jump_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
    VarModule::set_int(fighter.battle_object, vars::samus::status::SPECIAL_LW_JUMP_COUNT_FIX, jump_count);
    0.into()
}

unsafe extern "C" fn special_lw_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) == false {
        fighter.set_situation(SITUATION_KIND_AIR.into());
        let jump_count = VarModule::get_int(fighter.battle_object, vars::samus::status::SPECIAL_LW_JUMP_COUNT_FIX);
        WorkModule::set_int(fighter.module_accessor, jump_count, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_end"), 0.0, 1.0, false, 0.0, false, false);
    } else {
        fighter.set_situation(SITUATION_KIND_AIR.into());
        let control_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        let control_speed_x = lua_bind::KineticEnergy::get_speed_x(std::mem::transmute::<u64, &mut smash::app::KineticEnergy>(control_energy));
        let gravity_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        let gravity_speed_y = lua_bind::KineticEnergy::get_speed_y(std::mem::transmute::<u64, &mut smash::app::KineticEnergy>(gravity_energy));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        if VarModule::is_flag(fighter.battle_object, vars::samus::instance::SPEEDBOOSTER_ON) {
            sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 3.0, 0.0);
            sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, control_speed_x, 0.0);
        }
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, gravity_speed_y);

        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_end"), 0.0, 1.0, false, 0.0, false, false);
    }
    if ControlModule::get_stick_x(fighter.module_accessor) * PostureModule::lr(fighter.module_accessor) < 0.0 {
        PostureModule::reverse_lr(fighter.module_accessor);
        PostureModule::update_rot_y_lr(fighter.module_accessor);
        if VarModule::is_flag(fighter.battle_object, vars::samus::instance::SPEEDBOOSTER_ON) {
            crate::status::speedbooster_shinespark::speedbooster_off(fighter);
        }
    }
    ControlModule::clear_command(fighter.module_accessor, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_lw_end_main_loop as *const () as _))
}

pub unsafe fn special_lw_end_main_loop(fighter: &mut L2CFighterCommon) -> bool {
    if fighter.global_table[SITUATION_KIND].get_i32() != fighter.global_table[PREV_SITUATION_KIND].get_i32() {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("special_lw_end"), -1.0, 1.0, 0.0);
        } else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("special_air_lw_end"), -1.0, 1.0, 0.0);
        }
    }
    if VarModule::is_flag(fighter.battle_object, vars::samus::status::SPECIAL_LW_BOMB_JUMP_HOP) {
        VarModule::off_flag(fighter.battle_object, vars::samus::status::SPECIAL_LW_BOMB_JUMP_HOP);
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.set_situation(SITUATION_KIND_AIR.into());
            GroundModule::set_attach_ground(fighter.module_accessor, false);
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            let brake = if VarModule::is_flag(fighter.battle_object, vars::samus::instance::SPEEDBOOSTER_ON) {0.15} else {0.075} as f32;
            sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, brake, 0.0);
            let accel_x = if VarModule::is_flag(fighter.battle_object, vars::samus::instance::SPEEDBOOSTER_ON) {0.15} else {0.075} as f32;
            sv_kinetic_energy!(set_accel_x_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, accel_x);
        }
        let speed = if VarModule::is_flag(fighter.battle_object, vars::samus::instance::SPEEDBOOSTER_ON) {1.5} else {3.0} as f32;
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, speed, 0.0);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_SQUAT_WAIT.into(), true.into());
        } else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), true.into());
        }
        return true.into()
    } else if CancelModule::is_enable_cancel(fighter.module_accessor) {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT);
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() || fighter.sub_air_check_fall_common().get_bool() {
            return true.into()
        }
    }
    return false.into()
}

unsafe extern "C" fn special_lw_end_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn burst_attack_init(weapon: &mut L2CFighterCommon) -> L2CValue {
    smashline::original_status(Main, weapon, *WEAPON_SAMUS_BOMB_STATUS_KIND_BURST_ATTACK)(weapon);
    weapon.set_int(0, *WEAPON_SAMUS_BOMB_INSTANCE_WORK_ID_INT_BOMBJUMP);
    let fighter_pos_x = PostureModule::pos_x(weapon.get_owner_boma());
    let fighter_pos_y = PostureModule::pos_y(weapon.get_owner_boma());
    let weapon_pos_x = PostureModule::pos_x(weapon.module_accessor);
    let weapon_pos_y = PostureModule::pos_y(weapon.module_accessor);
    let distance = sv_math::vec2_distance(fighter_pos_x, fighter_pos_y, weapon_pos_x, weapon_pos_y);
    let param_bomb_jump_hit_size = weapon.get_owner_boma().get_param_float("param_special_lw", "sp_lw_bj_hit_size");
    if distance <= param_bomb_jump_hit_size
    && weapon.get_owner_boma().is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW,
        *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW
    ]) && VarModule::is_flag(weapon.get_owner_boma().object(), vars::samus::status::SPECIAL_LW_BOMB_JUMP_ON) {
        VarModule::on_flag(weapon.get_owner_boma().object(), vars::samus::status::SPECIAL_LW_BOMB_JUMP_HOP);
    }
    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_start_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_start_main);
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_start_end);
    agent.status(Pre, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW, special_lw_loop_pre);
    agent.status(Main, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW, special_lw_loop_main);
    agent.status(End, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW, special_lw_loop_end);
    agent.status(Pre, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW, special_lw_end_pre);
    agent.status(Main, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW, special_lw_end_main);
    agent.status(End, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW, special_lw_end_end);

    Agent::new("samus_bomb")
        .status(Init, *WEAPON_SAMUS_BOMB_STATUS_KIND_BURST_ATTACK, burst_attack_init)
        .install();
}