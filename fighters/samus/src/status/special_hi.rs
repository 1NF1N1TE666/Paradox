use super::*;

unsafe extern "C" fn special_hi_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn special_hi_init(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn special_hi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi"), 0.0, 1.0, false, 0.0, false, false);
        MotionModule::add_motion_2nd(fighter.module_accessor, Hash40::new("special_hi_2nd"), 0.0, 1.0, false, 0.0);
    } else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi"), 0.0, 1.0, false, 0.0, false, false);
        MotionModule::add_motion_2nd(fighter.module_accessor, Hash40::new("special_air_hi_2nd"), 0.0, 1.0, false, 0.0);
    }

    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        if !VarModule::is_flag(fighter.battle_object, vars::samus::instance::SPECIAL_HI_HOP_DISABLE) {
            VarModule::on_flag(fighter.battle_object, vars::samus::instance::SPECIAL_HI_HOP_DISABLE);
            let speed = if VarModule::is_flag(fighter.battle_object, vars::samus::instance::SPEEDBOOSTER_ON) {3.0} else {1.5} as f32;
            sv_kinetic_energy!(
                set_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                speed
            );
        }
        fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES.into());
        WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_CLIFF);
    }

    WorkModule::set_int(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GBEAM, *FIGHTER_STATUS_AIR_LASSO_WORK_INT_ARTICLE_ID);
    WorkModule::set_int(fighter.module_accessor, *ARTICLE_ID_NONE, *FIGHTER_STATUS_AIR_LASSO_WORK_INT_ARTICLE2_ID);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_AIR_LASSO_FLAG_CHECK);
    MotionModule::set_weight(fighter.module_accessor, 0.5, false);
    VarModule::off_flag(fighter.battle_object, vars::samus::status::SPECIAL_HI_LOCK_ANGLE);
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        if !VarModule::is_flag(fighter.battle_object, vars::samus::instance::SPECIAL_HI_HOP_DISABLE) {
            VarModule::on_flag(fighter.battle_object, vars::samus::instance::SPECIAL_HI_HOP_DISABLE);
            let speed = if VarModule::is_flag(fighter.battle_object, vars::samus::instance::SPEEDBOOSTER_ON) {3.0} else {1.5} as f32;
            sv_kinetic_energy!(
                set_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                speed
            );
        }
    }

    VarModule::off_flag(fighter.battle_object, vars::samus::status::SPECIAL_HI_FIX_GBEAM_POS);

    fighter.sub_shift_status_main(L2CValue::Ptr(special_hi_main_loop as *const () as _))
}

pub unsafe fn special_hi_main_loop(fighter: &mut L2CFighterCommon) -> bool {
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), true.into());
        } else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), true.into());
        }
        return true.into()
    } else if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return true.into()
        }
    } else {}

    if fighter.global_table[SITUATION_KIND].get_i32() != fighter.global_table[PREV_SITUATION_KIND].get_i32() {
        let frame = MotionModule::frame(fighter.module_accessor);
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            PLAY_LANDING_SE(fighter, Hash40::new("se_samus_landing01"));
            LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("special_hi"), -1.0, 1.0, 0.0);
            MotionModule::add_motion_2nd(fighter.module_accessor, Hash40::new("special_hi_2nd"), frame, 1.0, false, 0.0);
        } else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("special_air_hi"), -1.0, 1.0, 0.0);
            MotionModule::add_motion_2nd(fighter.module_accessor, Hash40::new("special_air_hi_2nd"), frame, 1.0, false, 0.0);
        }

        let angle = VarModule::get_float(fighter.battle_object, vars::samus::status::SPECIAL_HI_ANGLE);
        MotionModule::set_weight(fighter.module_accessor, angle / 90.0, false);
    }

    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND
    && fighter.is_flag(*FIGHTER_STATUS_AIR_LASSO_FLAG_CHECK) 
    && fighter.stick_y() > -0.4 
    && GroundModule::is_status_cliff(fighter.module_accessor)
    && GroundModule::can_entry_cliff(fighter.module_accessor) == 1
    && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_NO_CATCH_FRAME) <= 0 {
        fighter.change_status(FIGHTER_STATUS_KIND_AIR_LASSO_REACH.into(), true.into());
        return true.into()
    }

    if !VarModule::is_flag(fighter.battle_object, vars::samus::status::SPECIAL_HI_LOCK_ANGLE) {
        let angle;
        if fighter.stick_x().abs() + fighter.stick_y().abs() == 0.0 {
            angle = 90.0;
        } else {
            let mut stick_deg = ControlModule::get_stick_angle(fighter.module_accessor).to_degrees();
            if fighter.lr() < 0.0 {
                if stick_deg < 0.0 {
                    stick_deg = -180.0 - stick_deg;
                } else {
                    stick_deg = 180.0 - stick_deg;
                }
            }
            angle = stick_deg.clamp(0.0, 90.0);
        }
        VarModule::set_float(fighter.battle_object, vars::samus::status::SPECIAL_HI_ANGLE, angle);
        MotionModule::set_weight(fighter.module_accessor, angle / 90.0, false);
    }

    return false.into()
}

unsafe extern "C" fn special_hi_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.battle_object, vars::samus::status::SPECIAL_HI_FIX_GBEAM_POS) {
        VarModule::off_flag(fighter.battle_object, vars::samus::status::SPECIAL_HI_FIX_GBEAM_POS);
        let article = ArticleModule::get_article(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GBEAM);
        let object_id = lua_bind::Article::get_battle_object_id(article) as u32;
        let article_boma = sv_battle_object::module_accessor(object_id);

        LinkModule::set_model_constraint_target_joint(article_boma, Hash40::new("haver"));

        let lr = fighter.lr();
        let angle = VarModule::get_float(fighter.battle_object, vars::samus::status::SPECIAL_HI_ANGLE);

        let pos_x = angle.to_radians().cos() * 3.0 * lr;
        let pos_y = angle.to_radians().sin() * 3.0;
        let end_joint = PhysicsModule::get_2nd_node_num(article_boma) as i32;
        let vec3 = &mut Vector3f{x:0.0, y:3.0, z:0.0};
        ModelModule::joint_global_position(fighter.module_accessor, Hash40::new("haver"), vec3, false);
        PhysicsModule::set_2nd_pos(article_boma, end_joint, &Vector3f{x:vec3.x+pos_x, y:vec3.y+pos_y, z:0.0});
    }

    0.into()
}

unsafe extern "C" fn special_hi_exit(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn special_hi_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_STATUS_KIND_AIR_LASSO_REACH {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GBEAM, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    0.into()
}

unsafe extern "C" fn air_lasso_hang_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let article = ArticleModule::get_article(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GBEAM);
    let object_id = lua_bind::Article::get_battle_object_id(article) as u32;
    let article_boma = sv_battle_object::module_accessor(object_id);
    if fighter.lr() != PostureModule::lr(article_boma) {
        PostureModule::reverse_lr(fighter.module_accessor);
        PostureModule::update_rot_y_lr(fighter.module_accessor);
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_AIR_LASSO_BODY_FLIP_X, *FIGHTER_STATUS_AIR_LASSO_HANG_WORK_INT_BODY_FLIP);
    }

    smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_AIR_LASSO_HANG)(fighter)
}

unsafe extern "C" fn air_lasso_rewind_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.get_int(*FIGHTER_STATUS_AIR_LASSO_HANG_WORK_INT_BODY_FLIP) == *FIGHTER_STATUS_AIR_LASSO_BODY_FLIP_X 
    && fighter.lr() == -1.0 {
        PostureModule::reverse_lr(fighter.module_accessor);
        PostureModule::update_rot_y_lr(fighter.module_accessor);
    }

    smashline::original_status(Pre, fighter, *FIGHTER_STATUS_KIND_AIR_LASSO_REWIND)(fighter)
}

unsafe extern "C" fn air_lasso_rewind_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let article = ArticleModule::get_article(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GBEAM);
    let object_id = lua_bind::Article::get_battle_object_id(article) as u32;
    let article_boma = sv_battle_object::module_accessor(object_id);
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_AIR_LASSO_HANG_WORK_INT_BODY_FLIP) == *FIGHTER_STATUS_AIR_LASSO_BODY_FLIP_X 
    && PostureModule::lr(fighter.module_accessor) == PostureModule::lr(article_boma) {
        PostureModule::reverse_lr(fighter.module_accessor);
    }

    smashline::original_status(End, fighter, *FIGHTER_STATUS_KIND_AIR_LASSO_REWIND)(fighter)
}

unsafe extern "C" fn shoot_exec(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let fighter_boma = weapon.get_owner_boma();
    let lr = weapon.get_owner_boma().lr();
    let angle = VarModule::get_float(weapon.get_owner_boma().object(), vars::samus::status::SPECIAL_HI_ANGLE);
    let speed = weapon.get_param_float("param_gbeam", "shoot_air_speed");
    let chain_max = PhysicsModule::get_2nd_node_num(weapon.module_accessor) as i32;
    let x_speed_max = angle.to_radians().cos() * speed * lr;
    let y_speed_max = angle.to_radians().sin() * speed;
    let constraint_pos = &mut Vector3f{x:0.0, y:0.0, z:0.0};
    ModelModule::joint_global_position(weapon.get_owner_boma(), Hash40::new("haver"), constraint_pos, false);
    for chain_curr in 0..chain_max {
        let joint_hash = WorkModule::get_param_int64(weapon.module_accessor, hash40("joint_id_gbeam"), chain_curr as u64);
        let joint_pos = &mut Vector3f{x:0.0, y:0.0, z:0.0};
        ModelModule::joint_global_position(weapon.module_accessor, Hash40::new_raw(joint_hash), joint_pos, false);
        let x_speed = constraint_pos.x -joint_pos.x;
        let y_speed = constraint_pos.y -joint_pos.y;
        PhysicsModule::set_2nd_speed(weapon.module_accessor, chain_curr, &Vector3f{x:x_speed, y:y_speed, z:0.0});
    }
    PhysicsModule::set_2nd_speed(weapon.module_accessor, chain_max, &Vector3f{x:x_speed_max, y:y_speed_max, z:0.0});
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_pre);
    agent.status(Init, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_init);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_main);
    agent.status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_exec);
    agent.status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_exit);
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_end);
    agent.status(Main, *FIGHTER_STATUS_KIND_AIR_LASSO_HANG, air_lasso_hang_main);
    agent.status(Pre, *FIGHTER_STATUS_KIND_AIR_LASSO_REWIND, air_lasso_rewind_pre);
    agent.status(End, *FIGHTER_STATUS_KIND_AIR_LASSO_REWIND, air_lasso_rewind_end);

    Agent::new("samus_gbeam")
        .status(Exec, *WEAPON_SAMUS_GBEAM_STATUS_KIND_SHOOT, shoot_exec)
        .install();
}