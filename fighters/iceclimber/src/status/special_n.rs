use super::*;

unsafe extern "C" fn special_n_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON | *FIGHTER_LOG_MASK_FLAG_SHOOT) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32,
        0
    );

    0.into()
}

unsafe extern "C" fn special_n_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let mut sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let air_speed_x_stable = fighter.get_param_float("air_speed_x_stable", "");

    if !fighter.is_situation(*SITUATION_KIND_GROUND) {
        if VarModule::is_flag(fighter.battle_object, vars::iceclimbers::instance::SPECIAL_AIR_N) {
            sum_speed_x *= 0.4;
            sum_speed_y *= 0.65;
        }

        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, sum_speed_y, 0.0, 0.0);
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST, 0.0, 0.0, sum_speed_x, 0.0, 0.0);
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, air_speed_x_stable, 0.0);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_STOP, fighter.module_accessor);
        KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);

        VarModule::on_flag(fighter.battle_object, vars::iceclimbers::instance::SPECIAL_AIR_N);
    } else {
        sum_speed_x *= 0.2;

        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_GROUND, sum_speed_x, 0.0, 0.0, 0.0, 0.0);
        KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);
        KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, fighter.module_accessor);
    }

    0.into()
}

unsafe extern "C" fn special_n_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let log = if VarModule::get_float(fighter.battle_object, vars::iceclimbers::instance::LIMIT_GAUGE) == 100.0 { *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_02 } else { *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_01 };

    special_n_motion_helper(fighter, true.into());

    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, *FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, log - 1 );
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x3a40337e2c), log - 1);
    fighter.pop_lua_stack(1);

    fighter.sub_shift_status_main(L2CValue::Ptr(special_n_main_loop as *const () as _))
}

unsafe extern "C" fn special_n_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !CancelModule::is_enable_cancel(fighter.module_accessor) {
        if !MotionModule::is_end(fighter.module_accessor) {
            if StatusModule::is_changing(fighter.module_accessor) {
                return 0.into();
            }
            
            special_n_motion_helper(fighter, false.into());
        } else {
            if !fighter.is_situation(*SITUATION_KIND_GROUND) {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            } else {
                fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            }
        }
    } else {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }

    0.into()
}

unsafe extern "C" fn special_n_motion_helper(fighter: &mut L2CFighterCommon, param: L2CValue) {
    let air_speed_x_stable = fighter.get_param_float("air_speed_x_stable", "");

    if !fighter.is_situation(*SITUATION_KIND_GROUND) {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));

        if param.get_bool() {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, air_speed_x_stable * 0.5, 0.0);

            if VarModule::get_float(fighter.battle_object, vars::iceclimbers::instance::LIMIT_GAUGE) == 100.0 {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_lb"), 0.0, 1.0, false, 0.0, false, false);
            }
        } else {
            if VarModule::get_float(fighter.battle_object, vars::iceclimbers::instance::LIMIT_GAUGE) == 100.0
            || fighter.is_motion_one_of(&[
                Hash40::new("special_air_n_lb"),
                Hash40::new("special_n_lb")
            ]) {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n_lb"), -1.0, 1.0, 0.0, false, false);
            }
        }
    } else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));

        if param.get_bool() {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);

            if VarModule::get_float(fighter.battle_object, vars::iceclimbers::instance::LIMIT_GAUGE) == 100.0 {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_lb"), 0.0, 1.0, false, 0.0, false, false);
            }
        } else {
            if VarModule::get_float(fighter.battle_object, vars::iceclimbers::instance::LIMIT_GAUGE) == 100.0
            || fighter.is_motion_one_of(&[
                Hash40::new("special_air_n_lb"),
                Hash40::new("special_n_lb")
            ]) {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n_lb"), -1.0, 1.0, 0.0, false, false);
            }
        }
    }

    return;
}

unsafe extern "C" fn special_n_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let air_accel_y = fighter.get_param_float("air_accel_y", "");
    let air_speed_y_stable = fighter.get_param_float("air_speed_y_stable", "");

    if fighter.is_situation(*SITUATION_KIND_AIR) {
        if !VarModule::is_flag(fighter.battle_object, vars::iceclimbers::instance::SPECIAL_AIR_N_SPECIAL_FALL) {
            sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -air_accel_y);
            sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -air_speed_y_stable, 0.0);
        } else {
            if VarModule::is_flag(fighter.battle_object, vars::iceclimbers::instance::SPECIAL_AIR_N_HOP) {
                return 0.into();
            }
            
            sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.8);
            sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -0.06);
            sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -1.5, 0.0);

            VarModule::on_flag(fighter.battle_object, vars::iceclimbers::instance::SPECIAL_AIR_N_HOP);
        }
    }

    0.into()
}

unsafe extern "C" fn special_n_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_pre);
    agent.status(Init, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_init);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_main);
    agent.status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_exec);
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_end);
}