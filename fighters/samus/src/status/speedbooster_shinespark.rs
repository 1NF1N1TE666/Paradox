use super::*;

pub unsafe fn speedbooster_on(fighter: &mut L2CFighterCommon) {
    shinespark_off(fighter);
    VarModule::on_flag(fighter.battle_object, vars::samus::instance::SPEEDBOOSTER_ON);
    VarModule::set_int(fighter.object(), vars::samus::instance::SPEEDBOOSTER_STICK_TIMER, 20);
    VarModule::set_int(fighter.battle_object, vars::samus::instance::SPEEDBOOSTER_EFFECT_TIMER, 8);
    FLASH(fighter, 0.1, 0.2, 3.5, 0.1);
    BURN_COLOR(fighter, 0.4, 1.1, 6.0, 0.5);
    EFFECT_FOLLOW(fighter, Hash40::new("samus_jump_jet"), Hash40::new("bust"), 0, 0, 0, -90, -90, 0, 1, true);
    EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_elec"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 0.5, true);
    EFFECT(fighter, Hash40::new("sys_jump_aerial"), Hash40::new("top"), 0, 15, 0, -90, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    PLAY_SE(fighter, Hash40::new("se_item_magicball_warpin"));
}

pub unsafe fn speedbooster_off(fighter: &mut L2CFighterCommon) {
    VarModule::off_flag(fighter.battle_object, vars::samus::instance::SPEEDBOOSTER_ON);
    JostleModule::set_status(fighter.module_accessor, true);
    if fighter.is_status(*FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW)
    || !fighter.is_situation(*SITUATION_KIND_GROUND) {
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 1.5, 0.0);
    } else if fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_DASH,
        *FIGHTER_STATUS_KIND_ATTACK_LW3,
        *FIGHTER_STATUS_KIND_ATTACK_DASH
    ]) {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_RUN_STOP);
    } else {}
    COL_NORMAL(fighter);
    BURN_COLOR_NORMAL(fighter);
}

pub unsafe fn shinespark_on(fighter: &mut L2CFighterCommon) {
    VarModule::on_flag(fighter.battle_object, vars::samus::instance::SHINESPARK_ON);
    VarModule::set_int(fighter.battle_object, vars::samus::instance::SHINESPARK_EFFECT_TIMER, 0);
}

pub unsafe fn shinespark_off(fighter: &mut L2CFighterCommon) {
    VarModule::off_flag(fighter.battle_object, vars::samus::instance::SHINESPARK_ON);
    COL_NORMAL(fighter);
    BURN_COLOR_NORMAL(fighter);
}

pub unsafe fn speedbooster_effect(fighter: &mut L2CFighterCommon) {
    if VarModule::get_int(fighter.battle_object, vars::samus::instance::SPEEDBOOSTER_EFFECT_TIMER) <= 0 {
        VarModule::set_int(fighter.battle_object, vars::samus::instance::SPEEDBOOSTER_EFFECT_TIMER, 8);
        FLASH(fighter, 0.2, 0.5, 3.0, 0.7);
        BURN_COLOR(fighter, 0.2, 0.4, 7.1, 0.3);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_elec"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 0.5, true);
        if fighter.is_status(*FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW) && fighter.is_situation(*SITUATION_KIND_GROUND) {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_spark"), Hash40::new("top"), 0, 0, -1, 135, 25, 0, 0.5, true);
        }
    } else {
        if VarModule::get_int(fighter.battle_object, vars::samus::instance::SPEEDBOOSTER_EFFECT_TIMER) == 4 {
            FLASH(fighter, 0.1, 0.2, 3.5, 0.3);
            BURN_COLOR(fighter, 0.4, 1.1, 6.0, 0.7);
            if fighter.is_status(*FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW) && fighter.is_situation(*SITUATION_KIND_GROUND) {
                EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_spark"), Hash40::new("top"), 0, 0, -1, 135, 25, 0, 0.5, true);
            }
        }
        VarModule::dec_int(fighter.battle_object, vars::samus::instance::SPEEDBOOSTER_EFFECT_TIMER);
    }
}

pub unsafe fn shinespark_effect(fighter: &mut L2CFighterCommon) {
    if VarModule::get_int(fighter.battle_object, vars::samus::instance::SHINESPARK_EFFECT_TIMER) <= 0 {
        VarModule::set_int(fighter.battle_object, vars::samus::instance::SHINESPARK_EFFECT_TIMER, 8);
        FLASH(fighter, 1.0, 0.0, 4.0, 0.5);
        BURN_COLOR(fighter, 1.0, 0.0, 4.0, 0.1);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_elec"), Hash40::new("bust"), 0, 0, 0, 0, 0, 0, 0.8, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_elec"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 0.8, true);
    } else {
        if VarModule::get_int(fighter.battle_object, vars::samus::instance::SHINESPARK_EFFECT_TIMER) == 4 {
            FLASH(fighter, 1.0, 0.0, 4.0, 0.1);
            BURN_COLOR(fighter, 1.0, 0.0, 4.0, 0.5);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_elec"), Hash40::new("bust"), 0, 0, 0, 0, 0, 0, 0.8, true);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_elec"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 0.8, true);
        }
        VarModule::dec_int(fighter.battle_object, vars::samus::instance::SHINESPARK_EFFECT_TIMER);
    }
}

pub unsafe fn request_wall_effect(boma: *mut BattleObjectModuleAccessor) {
    let touch_pos = &mut Vector2f{x:0.0, y:0.0};
    let touch_rot = &mut Vector2f{x:0.0, y:0.0};
    FighterUtil::get_air_ground_touch_info(boma, touch_pos, touch_rot);
    let pos_z = GroundModule::get_z(boma);
    let rot = (-touch_rot.x).atan2(touch_rot.y);
    EffectModule::req(boma, Hash40::new("sys_crown"), &Vector3f{x:touch_pos.x, y:touch_pos.y, z:pos_z}, &Vector3f{x:0.0, y:0.0, z:rot}, 1.0, 0, -1, false, 0);
}

pub unsafe fn shinespark_end(fighter: &mut L2CFighterCommon) {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("shinespark_end"), 0.0, 1.0, false, 0.0, false, false);
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
    fighter.set_situation(SITUATION_KIND_GROUND.into());
    sv_kinetic_energy!(clear_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_JOSTLE);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_JOSTLE);
    sv_kinetic_energy!(clear_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
    sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
    sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.25);
}

pub unsafe fn shinespark_air_end(fighter: &mut L2CFighterCommon) {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("shinespark_air_end"), 0.0, 1.0, false, 0.0, false, false);
    sv_kinetic_energy!(clear_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_JOSTLE);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_JOSTLE);
    sv_kinetic_energy!(clear_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
    let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
    let air_speed_y_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_y_stable"), 0);
    sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, air_speed_x_stable, air_speed_y_stable);
    sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.3, 0.3);
    sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -0.3);
}

pub unsafe fn shinespark_air_ceil(fighter: &mut L2CFighterCommon) {
    request_wall_effect(fighter.module_accessor);
    let touch_pos = &mut Vector2f{x:0.0, y:0.0};
    FighterUtil::get_air_ground_touch_info(fighter.module_accessor, touch_pos, &mut Vector2f{x:0.0, y:0.0});
    let pos_x = PostureModule::pos_x(fighter.module_accessor);
    let pos_z = PostureModule::pos_z(fighter.module_accessor);
    PostureModule::set_pos(fighter.module_accessor, &Vector3f{x: pos_x, y: touch_pos.y - 17.0, z: pos_z});
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("shinespark_air_ceil"), 0.0, 1.0, false, 0.0, false, false);
    sv_kinetic_energy!(clear_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_JOSTLE);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_JOSTLE);
    sv_kinetic_energy!(clear_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
    sv_kinetic_energy!(clear_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
}

pub unsafe fn shinespark_wall(fighter: &mut L2CFighterCommon) {
    let pos_x = PostureModule::pos_x(fighter.module_accessor);
    let pos_y = PostureModule::pos_y(fighter.module_accessor);
    let pos_z = GroundModule::get_z(fighter.module_accessor);
    let ground_dist = GroundModule::get_distance_to_floor(fighter.module_accessor, &Vector3f{x: pos_x - (5.5 * fighter.lr()), y: pos_y, z: pos_z}, 90.0, true);
    let hit_pos = &mut Vector2f{x: 0.0, y: 0.0};
    let hit_rot = &mut Vector2f{x: 0.0, y: 0.0};
    let is_hit = GroundModule::ray_check_hit_pos_normal(fighter.module_accessor, &Vector2f{x: pos_x, y: pos_y + 7.0}, &Vector2f{x: 10.0 * fighter.lr(), y: 0.1}, hit_pos, hit_rot, true);
    if is_hit == 1 {
        PostureModule::set_pos(fighter.module_accessor, &Vector3f{x: hit_pos.x - (5.5 * fighter.lr()), y: pos_y - ground_dist, z: pos_z});
        let rot = (-hit_rot.x).atan2(hit_rot.y);
        EffectModule::req(fighter.module_accessor, Hash40::new("sys_crown"), &Vector3f{x: hit_pos.x, y: hit_pos.y, z: pos_z}, &Vector3f{x: 0.0, y: 0.0, z: rot}, 1.0, 0, -1, false, 0);
    } else {
        PostureModule::set_pos(fighter.module_accessor, &Vector3f{x: pos_x, y: pos_y - ground_dist, z: pos_z});
        EffectModule::req(fighter.module_accessor, Hash40::new("sys_crown"), &Vector3f{x: pos_x + 5.5, y: pos_y + 7.0, z: pos_z}, &Vector3f{x: 0.0, y: 0.0, z: 90.0}, 1.0, 0, -1, false, 0);
    }
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("shinespark_wall"), 0.0, 1.0, false, 0.0, false, false);
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
    fighter.set_situation(SITUATION_KIND_GROUND.into());
    sv_kinetic_energy!(clear_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
}

pub unsafe fn shinespark_air_wall(fighter: &mut L2CFighterCommon) {
    request_wall_effect(fighter.module_accessor);
    let touch_pos = &mut Vector2f{x:0.0, y:0.0};
    FighterUtil::get_air_ground_touch_info(fighter.module_accessor, touch_pos, &mut Vector2f{x:0.0, y:0.0});
    let pos_y = PostureModule::pos_y(fighter.module_accessor);
    let pos_z = PostureModule::pos_z(fighter.module_accessor);
    PostureModule::set_pos(fighter.module_accessor, &Vector3f{x: touch_pos.x - (5.5 * fighter.lr()), y: pos_y, z: pos_z});
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("shinespark_air_wall"), 0.0, 1.0, false, 0.0, false, false);
    sv_kinetic_energy!(clear_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_JOSTLE);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_JOSTLE);
    sv_kinetic_energy!(clear_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
    sv_kinetic_energy!(clear_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
}

pub unsafe fn shinespark_landing(fighter: &mut L2CFighterCommon) {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("shinespark_landing"), 0.0, 1.0, false, 0.0, false, false);
    sv_kinetic_energy!(clear_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
}

pub unsafe fn shinespark_special_lw_wall(fighter: &mut L2CFighterCommon) {
    let frame_2nd = MotionModule::frame_2nd(fighter.module_accessor);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("shinespark_air_wall"), 0.0, 1.0, false, 0.0, false, false);
    MotionModule::add_motion_2nd(fighter.module_accessor, Hash40::new("special_lw_loop"), frame_2nd, 0.0, false, 0.0);
    MotionModule::set_weight(fighter.module_accessor, 0.0, false);
    sv_kinetic_energy!(clear_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_JOSTLE);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_JOSTLE);
    sv_kinetic_energy!(clear_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
    sv_kinetic_energy!(clear_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
}

pub unsafe fn shinespark_special_lw_end(fighter: &mut L2CFighterCommon) {
    let frame_2nd = MotionModule::frame_2nd(fighter.module_accessor);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("shinespark_air_end"), 0.0, 1.0, false, 0.0, false, false);
    MotionModule::add_motion_2nd(fighter.module_accessor, Hash40::new("special_lw_loop"), frame_2nd, 40.0, false, 0.0);
    MotionModule::set_weight(fighter.module_accessor, 0.0, false);
    sv_kinetic_energy!(clear_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_JOSTLE);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_JOSTLE);
    sv_kinetic_energy!(clear_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
    let stable_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_y_stable"), 0);
    sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 1.5, stable_y);
    sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.3, 0.3);
    sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -0.3);
}

unsafe extern "C" fn walk_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.battle_object, vars::samus::instance::SPEEDBOOSTER_ON)
    && fighter.is_prev_status_one_of(&[
        *FIGHTER_STATUS_KIND_ATTACK_LW3,
        *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW
    ]) {
        fighter.change_status(FIGHTER_STATUS_KIND_DASH.into(), true.into());
        0.into()
    } else {
        smashline::original_status(Pre, fighter, *FIGHTER_STATUS_KIND_WALK)(fighter)
    }
}

unsafe extern "C" fn jump_squat_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.battle_object, vars::samus::instance::SHINESPARK_ON) && fighter.stick_x().abs() < 0.4 {
        fighter.change_status(FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_A.into(), true.into());
        0.into()
    } else {
        smashline::original_status(Pre, fighter, *FIGHTER_STATUS_KIND_JUMP_SQUAT)(fighter)
    }
}

unsafe extern "C" fn jump_aerial_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.battle_object, vars::samus::instance::SHINESPARK_ON) && fighter.stick_x() < 0.4 {
        fighter.dec_int(*FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
        fighter.change_status(FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_A.into(), true.into());
        0.into()
    } else {
        smashline::original_status(Pre, fighter, *FIGHTER_STATUS_KIND_JUMP_AERIAL)(fighter)
    }
}

unsafe extern "C" fn wall_jump_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_WALL_JUMP)(fighter);
    if VarModule::is_flag(fighter.battle_object, vars::samus::instance::SPEEDBOOSTER_ON) {
        fighter.set_int(0, *FIGHTER_INSTANCE_WORK_ID_INT_WALL_JUMP_PAST_FRAME);
        fighter.set_int(0, *FIGHTER_INSTANCE_WORK_ID_INT_WALL_JUMP_COUNT);
        fighter.set_int(0, *FIGHTER_STATUS_PASSIVE_WALL_WORK_INT_STOP_FRAME);
        CancelModule::enable_cancel(fighter.module_accessor);
        MotionModule::set_frame(fighter.module_accessor, 6.0, false);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_WALL_JUMP);
    }
    0.into()
}

unsafe extern "C" fn landing_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.battle_object, vars::samus::instance::SPEEDBOOSTER_ON) && fighter.stick_x() * fighter.lr() > 0.4  {
        fighter.change_status(FIGHTER_STATUS_KIND_DASH.into(), true.into());
        0.into()
    } else {
        smashline::original_status(Pre, fighter, *FIGHTER_STATUS_KIND_LANDING)(fighter)
    }
}

unsafe extern "C" fn landing_light_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.battle_object, vars::samus::instance::SPEEDBOOSTER_ON) && fighter.stick_x() * fighter.lr() > 0.4 {
        fighter.change_status(FIGHTER_STATUS_KIND_DASH.into(), true.into());
        0.into()
    } else {
        smashline::original_status(Pre, fighter, *FIGHTER_STATUS_KIND_LANDING_LIGHT)(fighter)
    }
}

unsafe extern "C" fn shinespark_ready_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_GROUND_STOP,
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        0,
        0,
        0,
        0
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_NONE | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_NO_ATTACK_SPECIAL | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        *FIGHTER_STATUS_ATTR_DISABLE_SWIM as u32,
        0,
        0
    );

    0.into()
}

unsafe extern "C" fn shinespark_ready_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn shinespark_ready_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("shinespark_ready"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(shinespark_ready_main_loop as *const () as _))
}

pub unsafe fn shinespark_ready_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), true.into());
        return true.into()
    } else if !fighter.is_situation(*SITUATION_KIND_GROUND) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), true.into());
        return true.into()
    } else if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return true.into()
        }
    } else if MotionModule::frame(fighter.module_accessor) <= 3.0 {
        if fighter.is_button_on(Buttons::Special) {
            VarModule::on_flag(fighter.battle_object, vars::samus::instance::SPEEDBOOSTER_ON);
            fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), true.into());
            return true.into()
        }
    } else if MotionModule::frame(fighter.module_accessor) > 3.0 && VarModule::is_flag(fighter.battle_object, vars::samus::instance::SPEEDBOOSTER_ON) {
        shinespark_off(fighter);
    } else {}
    return false.into()
}

unsafe extern "C" fn shinespark_ready_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn shinespark_ready_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_LW) {
        if CancelModule::is_enable_cancel(fighter.module_accessor) {
            shinespark_on(fighter);
        }
    } else {
        shinespark_on(fighter);
    }
    0.into()
}

unsafe extern "C" fn shinespark_start_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        0
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_ADDITIONS_ATTACK_01 | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        *FIGHTER_STATUS_ATTR_DISABLE_SWIM as u32,
        0,
        0
    );

    0.into()
}

unsafe extern "C" fn shinespark_start_init(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn shinespark_start_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_prev_status(*FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW) {
        VarModule::on_flag(fighter.battle_object, vars::samus::status::SHINESPARK_IS_SPECIAL_LW);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("shinespark_air_start"), 0.0, 1.0, false, 0.0, false, false);
        MotionModule::add_motion_2nd(fighter.module_accessor, Hash40::new("special_lw_loop"), 0.0, 1.0, false, 0.0);
        MotionModule::set_weight(fighter.module_accessor, 0.0, false);
        PostureModule::add_pos(fighter.module_accessor, &mut Vector3f{x: 0.0, y: 0.01, z: 0.0});
    } else {
        VarModule::off_flag(fighter.battle_object, vars::samus::status::SHINESPARK_IS_SPECIAL_LW);
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("shinespark_start"), 0.0, 1.0, false, 0.0, false, false);
        } else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("shinespark_air_start"), 0.0, 1.0, false, 0.0, false, false);
        }
    }
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    sv_kinetic_energy!(clear_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_JOSTLE);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GROUND_MOVEMENT);
    VarModule::off_flag(fighter.battle_object, vars::samus::instance::SHINESPARK_ON);
    VarModule::set_int(fighter.battle_object, vars::samus::status::SHINESPARK_AIM_TIMER, 30);
    VarModule::set_int(fighter.battle_object, vars::samus::status::SHINESPARK_AIM_EFFECT_TIMER, 14);
    fighter.sub_shift_status_main(L2CValue::Ptr(shinespark_start_main_loop as *const () as _))
}

pub unsafe fn shinespark_start_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_motion(Hash40::new("shinespark_air_aim")) {
        if VarModule::get_int(fighter.battle_object, vars::samus::status::SHINESPARK_AIM_TIMER) <= 0 {
            fighter.change_status(FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_G.into(), true.into());
            return true.into()
        } else {
            VarModule::dec_int(fighter.battle_object, vars::samus::status::SHINESPARK_AIM_TIMER);
            if !VarModule::is_flag(fighter.battle_object, vars::samus::status::SHINESPARK_IS_SPECIAL_LW) {
                let mut stick_deg = ControlModule::get_stick_angle(fighter.module_accessor).to_degrees();
                if fighter.lr() < 0.0 {
                    if stick_deg < 0.0 {
                        stick_deg = -180.0 - stick_deg;

                    } else {
                        stick_deg = 180.0 - stick_deg;
                    }
                }
                if stick_deg < 0.0 {
                    stick_deg += 360.0;
                }
                let frame_prev = MotionModule::frame(fighter.module_accessor);
                let mut frame = (stick_deg + frame_prev) / 2.0;
                if (stick_deg - frame_prev).abs() >= 180.0 {
                    frame += 180.0;
                    if frame >= 360.0 {
                        frame -= 360.0;
                    }
                }
                MotionModule::set_frame(fighter.module_accessor, frame, true);
                let weight_prev = MotionModule::weight(fighter.module_accessor);
                let stick_x = fighter.stick_x();
                let stick_y = fighter.stick_y();
                let stick_tilt = sv_math::vec2_length(stick_x, stick_y);
                let weight = (stick_tilt + weight_prev) / 2.0;
                MotionModule::set_weight(fighter.module_accessor, weight, false);
                if VarModule::get_int(fighter.battle_object, vars::samus::status::SHINESPARK_AIM_EFFECT_TIMER) <= 0 {
                    VarModule::set_int(fighter.battle_object, vars::samus::status::SHINESPARK_AIM_EFFECT_TIMER, 14);
                    FLASH(fighter, 1.0, 0.0, 4.0, 0.4);
                    BURN_COLOR(fighter, 1.0, 0.0, 4.0, 0.1);
                    EFFECT_FOLLOW(fighter, Hash40::new("samus_jump_jet"), Hash40::new("bust"), 0, 0, 0, -90.046, -90, 0, 1, true);
                    EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 2, -5, -90, 0, 0, 1.4, true);
                    LAST_PARTICLE_SET_COLOR(fighter, 0.5, 0.2, 0.5);
                    EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0, 3.5, 3, 0, 0, 0, 1.5, true);
                } else {
                    if VarModule::get_int(fighter.battle_object, vars::samus::status::SHINESPARK_AIM_EFFECT_TIMER) == 7 {
                        FLASH(fighter, 1.0, 0.0, 4.0, 0.3);
                        BURN_COLOR(fighter, 1.0, 0.0, 4.0, 0.5);
                        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 2, 5, -90, 0, 0, 1.4, true);
                        LAST_PARTICLE_SET_COLOR(fighter, 0.5, 0.2, 0.5);
                        EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0, 3.5, -3, 0, 0, 0, 1.5, true);
                    }
                    VarModule::dec_int(fighter.battle_object, vars::samus::status::SHINESPARK_AIM_EFFECT_TIMER);
                }
            } else {
                if VarModule::get_int(fighter.battle_object, vars::samus::status::SHINESPARK_AIM_EFFECT_TIMER) <= 0 {
                    VarModule::set_int(fighter.battle_object, vars::samus::status::SHINESPARK_AIM_EFFECT_TIMER, 14);
                    FLASH(fighter, 1.0, 0.0, 4.0, 0.4);
                    BURN_COLOR(fighter, 1.0, 0.0, 4.0, 0.1);
                    EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 0, -3, -90, 0, 0, 0.9, true);
                    LAST_PARTICLE_SET_COLOR(fighter, 0.5, 0.2, 0.5);
                    EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0, 3, 0, 0, 0, 0, 1, true);
                } else {
                    if VarModule::get_int(fighter.battle_object, vars::samus::status::SHINESPARK_AIM_EFFECT_TIMER) == 7 {
                        FLASH(fighter, 1.0, 0.0, 4.0, 0.3);
                        BURN_COLOR(fighter, 1.0, 0.0, 4.0, 0.5);
                        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 0, 3, -90, 0, 0, 0.9, true);
                        LAST_PARTICLE_SET_COLOR(fighter, 0.5, 0.2, 0.5);
                        EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0, 3, 0, 0, 0, 0, 1, true);
                    }
                    VarModule::dec_int(fighter.battle_object, vars::samus::status::SHINESPARK_AIM_EFFECT_TIMER);
                }
            }
        }
    } else if MotionModule::is_end(fighter.module_accessor) {
        if !VarModule::is_flag(fighter.battle_object, vars::samus::status::SHINESPARK_IS_SPECIAL_LW) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("shinespark_air_aim"), 0.0, 0.0, false, 0.0, false, false);
            let end_frame = MotionModule::end_frame(fighter.module_accessor);
            MotionModule::add_motion_2nd(fighter.module_accessor, Hash40::new("shinespark_air_aim"), end_frame, 0.0, false, 0.0);
            MotionModule::set_weight(fighter.module_accessor, 0.0, false);
        } else {
            let frame_2nd = MotionModule::frame_2nd(fighter.module_accessor);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("shinespark_air_aim"), 0.0, 0.0, false, 0.0, false, false);
            MotionModule::add_motion_2nd(fighter.module_accessor, Hash40::new("special_lw_loop"), frame_2nd, 40.0, false, 0.0);
            MotionModule::set_weight(fighter.module_accessor, 0.0, false);
        }
    } else if VarModule::is_flag(fighter.battle_object, vars::samus::status::SHINESPARK_IS_SPECIAL_LW) {
        let rate = MotionModule::frame(fighter.module_accessor).powf(1.25);
        if rate > 40.0 {
            MotionModule::set_rate_2nd(fighter.module_accessor, 40.0);
        } else {
            MotionModule::set_rate_2nd(fighter.module_accessor, rate);
        }
    }
    return false.into()
}

unsafe extern "C" fn shinespark_start_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn shinespark_start_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn shinespark_end_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        0
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_ADDITIONS_ATTACK_01 | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        *FIGHTER_STATUS_ATTR_DISABLE_SWIM as u32,
        0,
        0
    );

    0.into()
}

unsafe extern "C" fn shinespark_end_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn shinespark_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut lr = PostureModule::lr(fighter.module_accessor);
    let speed_x: f32;
    let speed_y: f32;
    if fighter.stick_x().abs() > 0.4 {
        if fighter.stick_x() * fighter.lr() < 0.0 {
            PostureModule::reverse_lr(fighter.module_accessor);
            PostureModule::update_rot_y_lr(fighter.module_accessor);
        }
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("shinespark_air_loop_s"), 0.0, 1.0, false, 0.0, false, false);
        if fighter.stick_y() > 0.4 {
            let diagonal = (((6.0 as f32).powi(2)) / 2.0).sqrt();
            speed_x = diagonal;
            speed_y = diagonal;
        } else if fighter.stick_y() < -0.4 {
            let diagonal = (((6.0 as f32).powi(2)) / 2.0).sqrt();
            speed_x = diagonal;
            speed_y = diagonal * -1.0;
        } else {
            speed_x = 6.0;
            speed_y = 0.0;
        }
    } else if fighter.stick_y() < -0.4 {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("shinespark_air_loop_lw"), 0.0, 1.0, false, 0.0, false, false);
        speed_x = 0.0;
        speed_y = 6.0 * -1.0;
    } else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("shinespark_air_loop_hi"), 0.0, 1.0, false, 0.0, false, false);
        speed_x = 0.0;
        speed_y = 6.0;
    }
    if VarModule::is_flag(fighter.battle_object, vars::samus::status::SHINESPARK_IS_SPECIAL_LW) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("shinespark_air_loop_s"), 0.0, 1.0, false, 0.0, false, false);
        MotionModule::add_motion_2nd(fighter.module_accessor, Hash40::new("special_lw_loop"), 0.0, 40.0, false, 0.0);
        MotionModule::set_weight(fighter.module_accessor, 0.0, false);
    }
    fighter.set_situation(SITUATION_KIND_AIR.into());
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    sv_kinetic_energy!(clear_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_JOSTLE);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GROUND_MOVEMENT);
    sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 6.0, 6.0);
    sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 6.0, 6.0);
    let lr = fighter.lr();
    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, speed_x * lr, speed_y);
    VarModule::set_int(fighter.battle_object, vars::samus::status::SHINESPARK_LOOP_TIMER, 10);
    VarModule::off_flag(fighter.battle_object, vars::samus::status::SHINESPARK_ENABLE_GRAVITY);
    VarModule::off_flag(fighter.battle_object, vars::samus::status::SHINESPARK_ENABLE_CONTROL);
    fighter.sub_shift_status_main(L2CValue::Ptr(shinespark_end_main_loop as *const () as _))
}

pub unsafe fn shinespark_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_motion(Hash40::new("shinespark_air_loop_hi")) {
        if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_UP as u32) {
            shinespark_air_ceil(fighter);
        }
        if VarModule::get_int(fighter.battle_object, vars::samus::status::SHINESPARK_LOOP_TIMER) <= 0 {
            shinespark_air_end(fighter);
        } else if SlowModule::frame(fighter.module_accessor, *FIGHTER_SLOW_KIND_HIT) < 2 && !StopModule::is_stop(fighter.module_accessor) {
            VarModule::dec_int(fighter.battle_object, vars::samus::status::SHINESPARK_LOOP_TIMER);
        }
    } else if fighter.is_motion(Hash40::new("shinespark_air_loop_s")) {
        if !VarModule::is_flag(fighter.battle_object, vars::samus::status::SHINESPARK_IS_SPECIAL_LW) {
            let mut angle = 0.0;
            if fighter.is_situation(*SITUATION_KIND_GROUND) {
                let vec_rot = &mut Vector2f{x:0.0, y:0.0};
                FighterUtil::get_air_ground_touch_info(fighter.module_accessor, &mut Vector2f{x:0.0, y:0.0}, vec_rot);
                angle = (-vec_rot.x).atan2(vec_rot.y).to_degrees();
            }
            let stop_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
            let stop_speed_y = lua_bind::KineticEnergy::get_speed_y(std::mem::transmute::<u64, &mut smash::app::KineticEnergy>(stop_energy));
            if stop_speed_y > 0.0 && GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_UP as u32) {
                shinespark_air_ceil(fighter);
            } else if stop_speed_y < 0.0 && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
                shinespark_landing(fighter);
            } else if stop_speed_y == 0.0 && fighter.is_situation(*SITUATION_KIND_GROUND) && (
                (angle > 5.0 && angle < 50.0) || (angle < -5.0 && angle > -50.0)
            ) {
                speedbooster_on(fighter);
                fighter.change_status(FIGHTER_STATUS_KIND_DASH.into(), true.into());
                return true.into()
            } else if (GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_RIGHT as u32) && fighter.lr() > 0.0) 
            || (GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_LEFT as u32) && fighter.lr() < 0.0) {
                let pos =  PostureModule::pos(fighter.module_accessor);
                let ground_dist = GroundModule::get_distance_to_floor(fighter.module_accessor, pos, 90.0, true);
                if ground_dist < 2.0 && ground_dist > 0.0 {
                    shinespark_wall(fighter);
                } else {
                    shinespark_air_wall(fighter);
                }
            } else if VarModule::get_int(fighter.battle_object, vars::samus::status::SHINESPARK_LOOP_TIMER) <= 0 {
                let pos =  PostureModule::pos(fighter.module_accessor);
                let ground_dist = GroundModule::get_distance_to_floor(fighter.module_accessor, pos, 90.0, true);
                if ground_dist < 2.0 && ground_dist > 0.0 {
                    PostureModule::add_pos(fighter.module_accessor, &mut Vector3f{x: 0.0, y: -ground_dist, z: 0.0});
                    shinespark_end(fighter);
                } else {
                    shinespark_air_end(fighter);
                }
            } else if SlowModule::frame(fighter.module_accessor, *FIGHTER_SLOW_KIND_HIT) < 2 && !StopModule::is_stop(fighter.module_accessor) {
                VarModule::dec_int(fighter.battle_object, vars::samus::status::SHINESPARK_LOOP_TIMER);
            }
        } else {
            if VarModule::get_int(fighter.battle_object, vars::samus::status::SHINESPARK_LOOP_TIMER) > 0 { 
                let angle;
                let stop_speed_y;
                if fighter.is_situation(*SITUATION_KIND_GROUND) {
                    let vec_rot = &mut Vector2f{x:0.0, y:0.0};
                    FighterUtil::get_air_ground_touch_info(fighter.module_accessor, &mut Vector2f{x:0.0, y:0.0}, vec_rot);
                    angle = (-vec_rot.x).atan2(vec_rot.y).to_degrees();
                    let stop_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
                    stop_speed_y = lua_bind::KineticEnergy::get_speed_y(std::mem::transmute::<u64, &mut smash::app::KineticEnergy>(stop_energy));
                } else {
                    angle = 0.0;
                    stop_speed_y = 0.0;
                }
                if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_UP as u32)
                || (GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_RIGHT as u32) && fighter.lr() > 0.0) 
                || (GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_LEFT as u32) && fighter.lr() < 0.0) {
                    request_wall_effect(fighter.module_accessor);
                    shinespark_special_lw_wall(fighter);
                } else if fighter.is_situation(*SITUATION_KIND_GROUND) && stop_speed_y < 0.0 {
                    LANDING_EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 3, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
                    shinespark_special_lw_wall(fighter);
                } else if fighter.is_situation(*SITUATION_KIND_GROUND) && ((angle > 5.0  && angle < 50.0) || (angle < -5.0 && angle > -50.0)) {
                    speedbooster_on(fighter);
                    fighter.change_status(FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW.into(), true.into()); //loop
                    return true.into()
                } else if SlowModule::frame(fighter.module_accessor, *FIGHTER_SLOW_KIND_HIT) < 2 && !StopModule::is_stop(fighter.module_accessor) {
                    VarModule::dec_int(fighter.battle_object, vars::samus::status::SHINESPARK_LOOP_TIMER);
                }
            } else {
                shinespark_special_lw_end(fighter);
            }
        }
    } else if fighter.is_motion(Hash40::new("shinespark_air_loop_lw")) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            shinespark_landing(fighter);
        } else if VarModule::get_int(fighter.battle_object, vars::samus::status::SHINESPARK_LOOP_TIMER) <= 0 {
            shinespark_air_end(fighter);
        } else if SlowModule::frame(fighter.module_accessor, *FIGHTER_SLOW_KIND_HIT) < 2 && !StopModule::is_stop(fighter.module_accessor) {
            VarModule::dec_int(fighter.battle_object, vars::samus::status::SHINESPARK_LOOP_TIMER);
        } else {}
    } else if fighter.is_motion_one_of(&[
        Hash40::new("shinespark_end"),
        Hash40::new("shinespark_wall"),
        Hash40::new("shinespark_landing")
    ]) {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), true.into());
            return true.into()
        } else if !fighter.is_situation(*SITUATION_KIND_GROUND) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), true.into());
            return true.into()
        } else if CancelModule::is_enable_cancel(fighter.module_accessor) {
            if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
                return true.into()
            }
        } else {}
    } else if fighter.is_motion_one_of(&[
        Hash40::new("shinespark_air_end"),
        Hash40::new("shinespark_air_wall"),
        Hash40::new("shinespark_air_ceil")
    ]) {
        if !VarModule::is_flag(fighter.battle_object, vars::samus::status::SHINESPARK_IS_SPECIAL_LW) {
            if VarModule::is_flag(fighter.battle_object, vars::samus::status::SHINESPARK_ENABLE_GRAVITY) {
                VarModule::off_flag(fighter.battle_object, vars::samus::status::SHINESPARK_ENABLE_GRAVITY);
                let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, -1);
                let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, -1);
                sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, speed_x, 0.0);
                sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, speed_y);
                KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            }
            if VarModule::is_flag(fighter.battle_object, vars::samus::status::SHINESPARK_ENABLE_CONTROL) {
                VarModule::off_flag(fighter.battle_object, vars::samus::status::SHINESPARK_ENABLE_CONTROL);
                let stable_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_y_stable"), 0);
                let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, -1);
                sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, stable_y);
                sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, speed_x, 0.0);
                sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST_NO_CAP, 0.0, 0.0, 0.0, 0.0, 0.0);
                KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            }
            if MotionModule::is_end(fighter.module_accessor) {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL_AERIAL.into(), true.into());
                return true.into()
            } else if fighter.is_situation(*SITUATION_KIND_GROUND) {
                fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), true.into());
                return true.into()
            } else if CancelModule::is_enable_cancel(fighter.module_accessor) {
                if fighter.sub_air_check_fall_common().get_bool() {
                    return true.into()
                }
            } else {}
        } else {
            let mut rate = MotionModule::rate_2nd(fighter.module_accessor);
            if rate > 15.0 {
                rate -= 0.25;
                MotionModule::set_rate_2nd(fighter.module_accessor, rate);
            }
            if VarModule::is_flag(fighter.battle_object, vars::samus::status::SHINESPARK_ENABLE_GRAVITY) {
                VarModule::off_flag(fighter.battle_object, vars::samus::status::SHINESPARK_ENABLE_GRAVITY);
                let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, -1);
                let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, -1);
                sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, speed_x, 0.0);
                sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, speed_y);
                KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            } else if MotionModule::is_end(fighter.module_accessor) || fighter.is_situation(*SITUATION_KIND_GROUND) || VarModule::is_flag(fighter.battle_object, vars::samus::status::SHINESPARK_ENABLE_CONTROL) {
                fighter.change_status(FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW.into(), true.into());
                return true.into()
            }
        }
    }
    return false.into()
}

unsafe extern "C" fn shinespark_end_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn shinespark_end_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_WALK, walk_pre);
    agent.status(Pre, *FIGHTER_STATUS_KIND_JUMP_SQUAT, jump_squat_pre);
    agent.status(Pre, *FIGHTER_STATUS_KIND_JUMP_AERIAL, jump_aerial_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_WALL_JUMP, wall_jump_main);
    agent.status(Pre, *FIGHTER_STATUS_KIND_LANDING, landing_pre);
    agent.status(Pre, *FIGHTER_STATUS_KIND_LANDING_LIGHT, landing_light_pre);
    agent.status(Pre, *FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP, shinespark_ready_pre);
    agent.status(Init, *FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP, shinespark_ready_init);
    agent.status(Main, *FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP, shinespark_ready_main);
    agent.status(Exec, *FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP, shinespark_ready_exec);
    agent.status(End, *FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP, shinespark_ready_end);
    agent.status(Pre, *FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_A, shinespark_start_pre);
    agent.status(Init, *FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_A, shinespark_start_init);
    agent.status(Main, *FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_A, shinespark_start_main);
    agent.status(Exec, *FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_A, shinespark_start_exec);
    agent.status(End, *FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_A, shinespark_start_end);
    agent.status(Pre, *FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_G, shinespark_end_pre);
    agent.status(Init, *FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_G, shinespark_end_init);
    agent.status(Main, *FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_G, shinespark_end_main);
    agent.status(Exec, *FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_G, shinespark_end_exec);
    agent.status(End, *FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_G, shinespark_end_end);
}