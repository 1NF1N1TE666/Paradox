use super::*;

unsafe extern "C" fn special_lw_stab_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !VarModule::is_flag(fighter.object(), vars::ridley::instance::SPECIAL_LW_IS_SKEWER) {
        fighter.change_status(statuses::ridley::SPECIAL_LW_POGO.into(), false.into());
        return true.into()
    }
    VarModule::off_flag(fighter.object(), vars::ridley::instance::SPECIAL_LW_IS_SKEWER);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_lw_stab") as i64, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_KIND);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_air_lw_stab") as i64, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_AIR_KIND);
    let motion;
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        motion = Hash40::new("special_lw_stab");
    } else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        motion = Hash40::new("special_air_lw_stab");
    }
    MotionModule::change_motion(fighter.module_accessor, motion, 0.0, 1.0, false, 0.0, false, false);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_LW_FLAG_TO_FINISH);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_lw_stab_main_loop as *const () as _))
}

pub unsafe fn special_lw_stab_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        } else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }

        return true.into()
    }

    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        let motion = MotionModule::motion_kind(fighter.module_accessor);
        let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor, Hash40::new_raw(motion), false);
        if MotionModule::frame(fighter.module_accessor) >= cancel_frame {
            if fighter.sub_wait_ground_check_common(false.into()).get_bool()
            || fighter.sub_air_check_fall_common().get_bool() {
                return true.into()
            }
        }
    }

    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_LW_FLAG_TO_FINISH) {
        fighter.change_status(FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_LW_FINISH.into(), false.into());
        return true.into()
    }

    if fighter.global_table[SITUATION_KIND].get_i32() != fighter.global_table[PREV_SITUATION_KIND].get_i32() {
        let motion;
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            fighter.set_situation(SITUATION_KIND_GROUND.into());
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            motion = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_KIND);
        } else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            fighter.set_situation(SITUATION_KIND_AIR.into());
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            motion = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_AIR_KIND);
        }

        MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new_raw(motion), -1.0, 1.0, 0.0);
    }

    false.into()
}

unsafe extern "C" fn special_lw_pogo_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_NONE,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn special_lw_pogo_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_pogo"), 0.0, 1.0, false, 0.0, false, false);
        sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 2);
    } else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_pogo"), 0.0, 1.0, false, 0.0, false, false);
    }
    VarModule::off_flag(fighter.object(), vars::ridley::status::SPECIAL_LW_POGO_CHECK_BOUNCE);
    VarModule::off_flag(fighter.object(), vars::ridley::status::SPECIAL_LW_POGO_ENABLE_LANDING);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_lw_pogo_main_loop as *const () as _))
}

pub unsafe fn special_lw_pogo_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return true.into()
    }

    if CancelModule::is_enable_cancel(fighter.module_accessor) 
    && fighter.sub_air_check_fall_common().get_bool() {
        return true.into()
    }

    if fighter.status_frame() >= 3 
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        if VarModule::is_flag(fighter.object(), vars::ridley::status::SPECIAL_LW_POGO_ENABLE_LANDING) {
            fighter.change_status(statuses::ridley::SPECIAL_LW_LANDING.into(), false.into());
        } else {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
        }
        return true.into()
    }

    let v3f_tail_pos = &mut Vector3f{x: 0.0, y: 0.0, z: 0.0};
    ModelModule::joint_global_position(fighter.module_accessor, Hash40::new("tail8"), v3f_tail_pos, false);
    let pos_x_global = PostureModule::pos_x(fighter.module_accessor);
    let pos_y_global = PostureModule::pos_y(fighter.module_accessor);
    let offset_x_prev = VarModule::get_float(fighter.object(), vars::ridley::status::SPECIAL_LW_POGO_CHECK_PREV_X);
    let offset_y_prev = VarModule::get_float(fighter.object(), vars::ridley::status::SPECIAL_LW_POGO_CHECK_PREV_Y);
    VarModule::set_float(fighter.object(), vars::ridley::status::SPECIAL_LW_POGO_CHECK_PREV_X, v3f_tail_pos.x - pos_x_global);
    VarModule::set_float(fighter.object(), vars::ridley::status::SPECIAL_LW_POGO_CHECK_PREV_Y, v3f_tail_pos.y - pos_y_global);
    if VarModule::is_flag(fighter.object(), vars::ridley::status::SPECIAL_LW_POGO_CHECK_BOUNCE) {
        let lr = PostureModule::lr(fighter.module_accessor);
        let ground_hit_pos = &mut Vector2f{x: 0.0, y: 0.0};
        let ground_hit_rot = &mut Vector2f{x: 0.0, y: 0.0};
        let pos_start_x = offset_x_prev+pos_x_global;
        let pos_start_y = offset_y_prev+pos_y_global;
        let offset_end_x = (v3f_tail_pos.x-pos_start_x) +6.0*lr;
        let offset_end_y = (v3f_tail_pos.y-pos_start_y) -9.0;
        if GroundModule::ray_check_hit_pos_normal(
            fighter.module_accessor,
            &Vector2f{x:pos_start_x, y: pos_start_y},
            &Vector2f{x: offset_end_x, y: offset_end_y},
            ground_hit_pos,
            ground_hit_rot,
            true
        ) == 1 {
            VarModule::off_flag(fighter.object(), vars::ridley::status::SPECIAL_LW_POGO_CHECK_BOUNCE);
            EffectModule::req(fighter.module_accessor, Hash40::new("sys_crown"), &Vector3f{x: ground_hit_pos.x, y: ground_hit_pos.y, z: 0.0}, &Vector3f{x: 0.0, y: 0.0, z: -ground_hit_rot.x}, 0.2, 0, 0, false, 0);
            EffectModule::req(fighter.module_accessor, Hash40::new("sys_quake"), &Vector3f{x: ground_hit_pos.x, y: ground_hit_pos.y, z: 0.0}, &Vector3f{x: 0.0, y: 0.0, z: -ground_hit_rot.x}, 0.5, 0, 0, false, 0);
            SoundModule::play_se(fighter.module_accessor, Hash40::new("se_ridley_special_h03"), true, false, false, false, enSEType(0));
            CameraModule::req_quake(fighter.module_accessor, *CAMERA_QUAKE_KIND_S);
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_impact"), 0, false, 0);
            let bounce_speed_max = 4.0;
            let bounce_speed_min = 2.0;
            let speed_y = bounce_speed_max - (((pos_y_global - ground_hit_pos.y).clamp(0.0, 30.0) / 30.0) * (bounce_speed_max - bounce_speed_min));
            let speed_x = lr*KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_MOTION, speed_x);
            sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, speed_y);
        } else if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            VarModule::off_flag(fighter.object(), vars::ridley::status::SPECIAL_LW_POGO_CHECK_BOUNCE);
            let speed_x = lr*KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_MOTION, speed_x);
            sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 2);
        }
    }

    false.into()
}

unsafe extern "C" fn special_lw_pogo_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn special_lw_landing_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_NONE,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW) as u64,
        0,
        0,
        0
    );
    0.into()
}

unsafe extern "C" fn special_lw_landing_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_landing"), 0.0, 1.0, false, 0.0, false, false);
    fighter.set_situation(SITUATION_KIND_GROUND.into());
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_lw_landing_main_loop as *const () as _))
}

pub unsafe fn special_lw_landing_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        return true.into()
    }

    if CancelModule::is_enable_cancel(fighter.module_accessor) 
    && fighter.sub_wait_ground_check_common(false.into()).get_bool() {
        return true.into()
    }

    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return true.into()
    }
    false.into()
}

unsafe extern "C" fn special_lw_landing_end(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_stab_main);
    agent.status(Pre, statuses::ridley::SPECIAL_LW_POGO, special_lw_pogo_pre);
    agent.status(Main, statuses::ridley::SPECIAL_LW_POGO, special_lw_pogo_main);
    agent.status(End, statuses::ridley::SPECIAL_LW_POGO, special_lw_pogo_end);
    agent.status(Pre, statuses::ridley::SPECIAL_LW_LANDING, special_lw_landing_pre);
    agent.status(Main, statuses::ridley::SPECIAL_LW_LANDING, special_lw_landing_main);
    agent.status(End, statuses::ridley::SPECIAL_LW_LANDING, special_lw_landing_end);
}