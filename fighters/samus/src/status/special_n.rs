use super::*;

unsafe extern "C" fn special_n_f_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !VarModule::is_flag(fighter.battle_object, vars::samus::instance::ICE_MODE) {
        return smashline::original_status(Main, fighter, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_F)(fighter);
    }

    let charge = fighter.get_int(*FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_COUNT) as f32 / fighter.get_param_float("param_special_n", "cshot_charge_frame");
    let max = fighter.get_int(*FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_COUNT) as f32 >= fighter.get_param_float("param_special_n", "cshot_charge_frame");
    let motion_g = if !max {Hash40::new("special_n_i")} else {Hash40::new("special_n_i_max")};
    let motion_a = if !max {Hash40::new("special_air_n_i")} else {Hash40::new("special_air_n_i_max")};
    let motion = if fighter.is_situation(*SITUATION_KIND_GROUND) {motion_g} else {motion_a};

    if fighter.is_flag(*FIGHTER_SAMUS_STATUS_SPECIAL_N_FLAG_MOT_RESTART) {
        MotionModule::change_motion(fighter.module_accessor, motion, 0.0, 1.0, false, 0.0, false, false);
        fighter.off_flag(*FIGHTER_SAMUS_STATUS_SPECIAL_N_FLAG_MOT_RESTART);
    } else {
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, motion, -1.0, 1.0, 0.0,false, false);
    }

    let groundcorrect = if fighter.is_situation(*SITUATION_KIND_GROUND) {*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK} else {*GROUND_CORRECT_KIND_AIR};
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(groundcorrect));

    let kinetic = if fighter.is_situation(*SITUATION_KIND_GROUND) {*FIGHTER_KINETIC_TYPE_GROUND_STOP} else {*FIGHTER_KINETIC_TYPE_AIR_STOP};
    KineticModule::change_kinetic(fighter.module_accessor, kinetic);
    
    fighter.sub_shift_status_main(L2CValue::Ptr(special_n_f_main_loop as *const () as _))
}

unsafe extern "C" fn special_n_f_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        let max = fighter.get_int(*FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_COUNT) as f32 >= fighter.get_param_float("param_special_n", "cshot_charge_frame");
        let motion_g = if !max {Hash40::new("special_n_i")} else {Hash40::new("special_n_i_max")};
        let motion_a = if !max {Hash40::new("special_air_n_i")} else {Hash40::new("special_air_n_i_max")};
        fighter.sub_change_motion_by_situation(motion_g.into(), motion_a.into(),true.into());

        let groundcorrect = if fighter.is_situation(*SITUATION_KIND_GROUND) {*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK} else {*GROUND_CORRECT_KIND_AIR};
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(groundcorrect));
    
        let kinetic = if fighter.is_situation(*SITUATION_KIND_GROUND) {*FIGHTER_KINETIC_TYPE_GROUND_STOP} else {*FIGHTER_KINETIC_TYPE_AIR_STOP};
        KineticModule::change_kinetic(fighter.module_accessor, kinetic);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(FIGHTER_STATUS_KIND_WAIT.into(), FIGHTER_STATUS_KIND_FALL.into(),false.into());
        return 0.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    0.into()
}

unsafe extern "C" fn special_n_f_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    EFFECT_OFF_KIND(fighter,Hash40::new("sys_ice"),false,false);
    return smashline::original_status(End, fighter, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_F)(fighter);
}

unsafe extern "C" fn shoot_init(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = weapon.get_param_int("param_cshot", "life");
    weapon.set_int(life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    weapon.set_int(life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if weapon.is_flag(*WEAPON_INSTANCE_WORK_ID_FLAG_SWALLOWED)
    && !GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
        effect!(weapon, MA_MSC_EFFECT_REQUEST_FOLLOW, Hash40::new("samus_cshot_bullet"), Hash40::new("top"), 7.98004, -0.50584, -0.25092, -91.2728, -1.7974, 176.373, 1.0, false, 0, 0, 0);
        weapon.clear_lua_stack();
        lua_args!(weapon, MA_MSC_EFFECT_GET_LAST_HANDLE);
        sv_module_access::effect(weapon.lua_state_agent);
        let handle = weapon.pop_lua_stack(1).get_i32();
        weapon.set_int(handle, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_INT_EFH_BULLET);
    }
    let lr = WorkModule::get_float(weapon.module_accessor, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_FLOAT_SHOOT_LR);
    let charge = WorkModule::get_float(weapon.module_accessor, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_FLOAT_CHARGE);
    let angle = VarModule::get_float(weapon.get_owner_boma().object(), vars::samus::instance::AIM_ANGLE);
    let min_speed = weapon.get_param_float("param_cshot", "min_speed");
    let max_speed = weapon.get_param_float("param_cshot", "max_speed");
    let speed = (max_speed - min_speed) * charge + min_speed;
    let speed_x = angle.to_radians().cos() * speed * lr;
    let speed_y = angle.to_radians().sin() * speed;
    sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x, speed_y);
    sv_kinetic_energy!(set_stable_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, -1.0, -1.0);
    sv_kinetic_energy!(set_accel, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    if !GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
        let min_scale = weapon.get_param_float("param_cshot", "min_scale");
        let max_scale = weapon.get_param_float("param_cshot", "max_scale");
        let scale = (max_scale - min_scale) * charge + min_scale;
        if (0.3..1.0).contains(&scale) {
            effect!(weapon, MA_MSC_EFFECT_REQUEST_FOLLOW, Hash40::new("samus_cshot_bullet_sub_a"), Hash40::new("top"), 7.98004, -0.50584, -0.25092, -91.2728, -1.7974, 176.373, scale, false, 0, 0, 0);
        } else {
            effect!(weapon, MA_MSC_EFFECT_REQUEST_FOLLOW, Hash40::new("samus_cshot_bullet_sub_b"), Hash40::new("top"), 7.98004, -0.50584, -0.25092, -91.2728, -1.7974, 176.373, scale, false, 0, 0, 0);
        }
        weapon.clear_lua_stack();
        lua_args!(weapon, MA_MSC_EFFECT_GET_LAST_HANDLE);
        sv_module_access::effect(weapon.lua_state_agent);
        let handle = weapon.pop_lua_stack(1).get_i32();
        weapon.set_int(handle, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_INT_EFH_BULLET_FOLLOW);
        effect!(weapon, MA_MSC_EFFECT_REQUEST_FOLLOW, Hash40::new("samus_cshot_bullet_sub"), Hash40::new("top"), 7.98004, -0.50584, -0.25092, -91.2728, -1.7974, 176.373, scale, false, 0, 0, 0);
        weapon.clear_lua_stack();
        lua_args!(weapon, MA_MSC_EFFECT_GET_LAST_HANDLE);
        sv_module_access::effect(weapon.lua_state_agent);
        let handle = weapon.pop_lua_stack(1).get_i32();
        weapon.set_int(handle, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_INT_EFH_BULLET_FOLLOW_SUB);
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_F, special_n_f_main);
    agent.status(End, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_F, special_n_f_end);

    Agent::new("samus_cshot")
        .status(Init, *WEAPON_SAMUS_CSHOT_STATUS_KIND_SHOOT, shoot_init)
        .install();
}