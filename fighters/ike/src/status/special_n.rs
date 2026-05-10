use super::*;

pub unsafe fn stored_aether_effect_on(fighter: &mut L2CAgentBase) {
    VarModule::set_int(fighter.battle_object, vars::ike::instance::STORED_AETHER_EFFECT_COUNT, 5);
    EFFECT_FOLLOW(fighter, Hash40::new("ike_sword2"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, false);
    EFFECT_FOLLOW(fighter, Hash40::new("ike_final_sword"), Hash40::new("sword"), 0, 1, 0, 0, 0, 0, 0.6, false);
    EFFECT_FOLLOW(fighter, Hash40::new("ike_final_sword"), Hash40::new("sword"), 0, 6, 0, 0, 0, 0, 0.6, false);
}

pub unsafe fn stored_aether_effect_off(fighter: &mut L2CAgentBase) {
    EFFECT_OFF_KIND(fighter, Hash40::new("ike_sword2"), true, true);
    EFFECT_OFF_KIND(fighter, Hash40::new("ike_final_sword"), true, true);
}

unsafe extern "C" fn special_n_start_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(End, fighter, *FIGHTER_STATUS_KIND_SPECIAL_N)(fighter);

    if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_IKE_STATUS_KIND_SPECIAL_N_LOOP
    && fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_IKE_STATUS_KIND_SPECIAL_N_END {
        if VarModule::is_flag(fighter.battle_object, vars::ike::instance::STORED_AETHER) {
            VarModule::off_flag(fighter.battle_object, vars::ike::instance::STORED_AETHER);
            stored_aether_effect_off(fighter);
        }
        VarModule::set_float(fighter.battle_object, vars::ike::instance::SPECIAL_N_CHARGE_COUNT, 0.0);
    }

    ret
}

unsafe extern "C" fn special_n_loop_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Init, fighter, *FIGHTER_IKE_STATUS_KIND_SPECIAL_N_LOOP)(fighter);

    ret;

    if VarModule::is_flag(fighter.battle_object, vars::ike::instance::STORED_AETHER) {
        let param_charge_max = WorkModule::get_param_int(fighter.module_accessor, smash::hash40("param_special_n"), smash::hash40("special_n_charge_count_max"));        
        let charge_level = ((param_charge_max - 1) * 30) as f32;
        WorkModule::set_float(fighter.module_accessor, charge_level, *FIGHTER_IKE_STATUS_SPECIAL_N_WORK_FLOAT_CHARGE_COUNT);
        WorkModule::set_int(fighter.module_accessor, 2,*FIGHTER_IKE_STATUS_SPECIAL_N_WORK_INT_READY_LEVEL);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_N_FLAG_MAX_CHARGE_SOUND_DONE);
        stored_aether_effect_off(fighter);
    } else {
        let param_charge_mdl = WorkModule::get_param_int(fighter.module_accessor, smash::hash40("param_special_n"), smash::hash40("special_n_charge_count_mdl"));        
        let charge_count_mdl = (param_charge_mdl * 30) as f32;
        if VarModule::get_float(fighter.battle_object, vars::ike::instance::SPECIAL_N_CHARGE_COUNT) > charge_count_mdl {
            WorkModule::set_int(fighter.module_accessor, 1,*FIGHTER_IKE_STATUS_SPECIAL_N_WORK_INT_READY_LEVEL);
        }
        let charge_count = VarModule::get_float(fighter.battle_object, vars::ike::instance::SPECIAL_N_CHARGE_COUNT);
        WorkModule::set_float(fighter.module_accessor, charge_count, *FIGHTER_IKE_STATUS_SPECIAL_N_WORK_FLOAT_CHARGE_COUNT);
    }
    
    VarModule::off_flag(fighter.battle_object, vars::ike::instance::STORED_AETHER);
    VarModule::set_float(fighter.battle_object, vars::ike::instance::SPECIAL_N_CHARGE_COUNT, 0.0);

    0.into()
}

unsafe extern "C" fn special_n_loop_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let param_charge_mdl = WorkModule::get_param_int(fighter.module_accessor, smash::hash40("param_special_n"), smash::hash40("special_n_charge_count_mdl"));        
    let charge_count_mdl = (param_charge_mdl * 30) as f32;
    let param_charge_max = WorkModule::get_param_int(fighter.module_accessor, smash::hash40("param_special_n"), smash::hash40("special_n_charge_count_max"));        
    let charge_count_max = ((param_charge_max - 1) * 30) as f32;
    let jump_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
    let jump_count_max = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);
    let ret = smashline::original_status(Exec, fighter, *FIGHTER_IKE_STATUS_KIND_SPECIAL_N_LOOP)(fighter);

    if WorkModule::get_float(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_N_WORK_FLOAT_CHARGE_COUNT) > charge_count_mdl 
    && WorkModule::get_int(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_N_WORK_INT_READY_LEVEL) < 1 {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("sword"), 0, 16, 0, 0, 0, 0, 1.6, false);
    }

    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP)
    && jump_count < jump_count_max {
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_N_WORK_INT_READY_LEVEL) == 2 {
            VarModule::set_float(fighter.battle_object, vars::ike::instance::SPECIAL_N_CHARGE_COUNT, charge_count_max);
            VarModule::on_flag(fighter.battle_object, vars::ike::instance::STORED_AETHER);
            stored_aether_effect_on(fighter);
        } else {
            let charge_count = WorkModule::get_float(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_N_WORK_FLOAT_CHARGE_COUNT);
            VarModule::set_float(fighter.battle_object, vars::ike::instance::SPECIAL_N_CHARGE_COUNT, charge_count);
        }

        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_JUMP_SQUAT.into(), true.into());
        } else {
            fighter.change_status(FIGHTER_STATUS_KIND_JUMP_AERIAL.into(), true.into());
        }
    } else if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
    && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR) {
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_N_WORK_INT_READY_LEVEL) == 2 {
            VarModule::set_float(fighter.battle_object, vars::ike::instance::SPECIAL_N_CHARGE_COUNT, charge_count_max);
            VarModule::on_flag(fighter.battle_object, vars::ike::instance::STORED_AETHER);
            stored_aether_effect_on(fighter);
        } else {
            let charge_count = WorkModule::get_float(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_N_WORK_FLOAT_CHARGE_COUNT);
            VarModule::set_float(fighter.battle_object, vars::ike::instance::SPECIAL_N_CHARGE_COUNT, charge_count);
        }

        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            if ControlModule::get_stick_y(fighter.module_accessor) < -0.6 {
                fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE.into(), true.into());
            } else if ControlModule::get_stick_x(fighter.module_accessor) * PostureModule::lr(fighter.module_accessor) > 0.6 {
                fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_F.into(), true.into());
            } else if ControlModule::get_stick_x(fighter.module_accessor) * PostureModule::lr(fighter.module_accessor) < -0.6 {
                fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_B.into(), true.into());
            } else {
                fighter.change_status(FIGHTER_STATUS_KIND_GUARD_ON.into(), true.into());
            }
        } else {
            fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_AIR.into(), true.into());
        }
    } else if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        if ControlModule::get_flick_y(fighter.module_accessor) <= 5 
        && ControlModule::get_stick_y(fighter.module_accessor) < 0.0 
        && GroundModule::is_passable_ground(fighter.module_accessor) {
            GroundModule::pass_floor(fighter.module_accessor);
        }
    }

    ret;

    if WorkModule::get_float(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_N_WORK_FLOAT_CHARGE_COUNT) <= charge_count_mdl {
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_IKE_STATUS_SPECIAL_N_WORK_INT_READY_LEVEL);
    } else if WorkModule::get_float(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_N_WORK_FLOAT_CHARGE_COUNT) <= charge_count_max {
        WorkModule::set_int(fighter.module_accessor, 1, *FIGHTER_IKE_STATUS_SPECIAL_N_WORK_INT_READY_LEVEL);
    } else {
        WorkModule::set_int(fighter.module_accessor, 2, *FIGHTER_IKE_STATUS_SPECIAL_N_WORK_INT_READY_LEVEL);
    }
    
    0.into()
}

unsafe extern "C" fn special_n_end_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let param_charge_mdl = WorkModule::get_param_int(fighter.module_accessor, smash::hash40("param_special_n"), smash::hash40("special_n_charge_count_mdl"));        
    let charge_count_mdl = (param_charge_mdl*30) as f32;
    let original = smashline::original_status(Pre, fighter, *FIGHTER_IKE_STATUS_KIND_SPECIAL_N_END);

    if VarModule::is_flag(fighter.battle_object, vars::ike::instance::STORED_AETHER) {
        VarModule::off_flag(fighter.battle_object, vars::ike::instance::STORED_AETHER);
        stored_aether_effect_off(fighter);
        fighter.change_status(FIGHTER_IKE_STATUS_KIND_SPECIAL_N_END_MAX.into(), true.into());
    } else if VarModule::get_float(fighter.battle_object, vars::ike::instance::SPECIAL_N_CHARGE_COUNT) > charge_count_mdl {
        fighter.change_status(FIGHTER_IKE_STATUS_KIND_SPECIAL_N_END_MDL.into(), true.into());
    } else {
        original(fighter);
    }

    if VarModule::get_float(fighter.battle_object, vars::ike::instance::SPECIAL_N_CHARGE_COUNT) > 0.0 {
        VarModule::set_float(fighter.battle_object, vars::ike::instance::SPECIAL_N_CHARGE_COUNT, 0.0);
    }

    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_start_end);
    agent.status(Init, *FIGHTER_IKE_STATUS_KIND_SPECIAL_N_LOOP, special_n_loop_init);
    agent.status(Exec, *FIGHTER_IKE_STATUS_KIND_SPECIAL_N_LOOP, special_n_loop_exec);
    agent.status(Pre, *FIGHTER_IKE_STATUS_KIND_SPECIAL_N_END, special_n_end_pre);
}