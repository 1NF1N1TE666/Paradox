use super::*;

unsafe extern "C" fn game_specialnstart(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
    }
}

unsafe extern "C" fn game_specialnloop(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
    }
}

unsafe extern "C" fn effect_specialnloop(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    let param_charge_mdl = WorkModule::get_param_int(boma, smash::hash40("param_special_n"), smash::hash40("special_n_charge_count_mdl"));        
    let charge_level_mdl = (param_charge_mdl * 30) as f32;
    let param_charge_max = WorkModule::get_param_int(boma, smash::hash40("param_special_n"), smash::hash40("special_n_charge_count_max"));        
    let charge_level_max = ((param_charge_max - 1) * 30) as f32;
    loop {
        if is_excute(agent) {
            if WorkModule::get_int(boma, *FIGHTER_IKE_STATUS_SPECIAL_N_WORK_INT_READY_LEVEL) == 0 {
                if WorkModule::get_float(boma, *FIGHTER_IKE_STATUS_SPECIAL_N_WORK_FLOAT_CHARGE_COUNT) < charge_level_mdl / 2.0 {
                    FLASH(agent, 0.125, 0.4, 1, 0.1);
                } else {
                    FLASH(agent, 0.125, 0.4, 1, 0.2);
                }
            } else if WorkModule::get_int(boma, *FIGHTER_IKE_STATUS_SPECIAL_N_WORK_INT_READY_LEVEL) == 1 {
                if WorkModule::get_float(boma, *FIGHTER_IKE_STATUS_SPECIAL_N_WORK_FLOAT_CHARGE_COUNT) < ((charge_level_max-charge_level_mdl) / 2.0) + charge_level_mdl {
                    FLASH(agent, 0.125, 0.4, 1, 0.3);
                } else {
                    FLASH(agent, 0.125, 0.4, 1, 0.35);
                }
            } else {
                FLASH(agent, 0.125, 0.4, 1, 0.4);
            }
        }
        wait(lua_state, 4.0);
        if is_excute(agent) {
            if WorkModule::get_int(boma, *FIGHTER_IKE_STATUS_SPECIAL_N_WORK_INT_READY_LEVEL) == 0 {
                if WorkModule::get_float(boma, *FIGHTER_IKE_STATUS_SPECIAL_N_WORK_FLOAT_CHARGE_COUNT) < charge_level_mdl / 2.0 {
                    FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -8, 0, 0, 0, 0, 0, 0.5, 15, 0, 4, 0, 0, 0, false);
                } else {
                    FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -8, 0, 0, 0, 0, 0, 0.65, 15, 0, 4, 0, 0, 0, false);
                }
            } else if WorkModule::get_int(boma, *FIGHTER_IKE_STATUS_SPECIAL_N_WORK_INT_READY_LEVEL) == 1 {
                if WorkModule::get_float(boma, *FIGHTER_IKE_STATUS_SPECIAL_N_WORK_FLOAT_CHARGE_COUNT) < ((charge_level_max-charge_level_mdl) / 2.0) + charge_level_mdl {
                    FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -8, 0, 0, 0, 0, 0, 0.75, 15, 0, 4, 0, 0, 0, false);
                } else {
                    FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -8, 0, 0, 0, 0, 0, 0.85, 15, 0, 4, 0, 0, 0, false);
                }
            } else {
                FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -8, 0, 0, 0, 0, 0, 1, 15, 0, 4, 0, 0, 0, false);
            }

            COL_NORMAL(agent);            
        }
        if WorkModule::get_int(boma, *FIGHTER_IKE_STATUS_SPECIAL_N_WORK_INT_READY_LEVEL) == 0 | 1 {
            wait(lua_state, 2.0);
        } else {
            wait(lua_state, 1.0);
        }
    }
}

unsafe extern "C" fn effect_specialairnloop(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    let param_charge_mdl = WorkModule::get_param_int(boma, smash::hash40("param_special_n"), smash::hash40("special_n_charge_count_mdl"));        
    let charge_level_mdl = (param_charge_mdl * 30) as f32;
    let param_charge_max = WorkModule::get_param_int(boma, smash::hash40("param_special_n"), smash::hash40("special_n_charge_count_max"));        
    let charge_level_max = ((param_charge_max - 1) * 30) as f32;
    loop {
        if is_excute(agent) {
            if WorkModule::get_int(boma, *FIGHTER_IKE_STATUS_SPECIAL_N_WORK_INT_READY_LEVEL) == 0 {
                if WorkModule::get_float(boma, *FIGHTER_IKE_STATUS_SPECIAL_N_WORK_FLOAT_CHARGE_COUNT) < charge_level_mdl / 2.0 {
                    FLASH(agent, 0.125, 0.4, 1, 0.1);
                } else {
                    FLASH(agent, 0.125, 0.4, 1, 0.2);
                }
            } else if WorkModule::get_int(boma, *FIGHTER_IKE_STATUS_SPECIAL_N_WORK_INT_READY_LEVEL) == 1 {
                if WorkModule::get_float(boma, *FIGHTER_IKE_STATUS_SPECIAL_N_WORK_FLOAT_CHARGE_COUNT) < ((charge_level_max-charge_level_mdl) / 2.0) + charge_level_mdl {
                    FLASH(agent, 0.125, 0.4, 1, 0.3);
                } else {
                    FLASH(agent, 0.125, 0.4, 1, 0.35);
                }
            } else {
                FLASH(agent, 0.125, 0.4, 1, 0.4);
            }
        }
        wait(lua_state, 4.0);
        if is_excute(agent) {
            COL_NORMAL(agent);
        }
        if WorkModule::get_int(boma, *FIGHTER_IKE_STATUS_SPECIAL_N_WORK_INT_READY_LEVEL) == 0 | 1 {
            wait(lua_state, 2.0);
        } else {
            wait(lua_state, 1.0);
        }
    }
}

unsafe extern "C" fn game_specialnend(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        KineticModule::set_consider_ground_friction(boma, false, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    FT_MOTION_RATE(agent, 0.9);
    frame(lua_state, 11.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        if boma.kind() == *FIGHTER_KIND_KIRBY {
            ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 90, 100, 0, 50, 15.0, 0.0, 10.0, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
            ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 90, 100, 0, 50, 10.0, 0.0, 25.0, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
            ATTACK(agent, 2, 0, Hash40::new("top"), 10.0, 90, 100, 0, 50, 5.0, 0.0, -5.0, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        } else {
            ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 90, 100, 0, 50, 15.0, 0.0, 10.0, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
            ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 90, 100, 0, 50, 10.0, 0.0, 25.0, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
            ATTACK(agent, 2, 0, Hash40::new("sword"), 10.0, 90, 100, 0, 50, 5.0, 0.0, 10.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        }
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_specialnendmdl(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        KineticModule::set_consider_ground_friction(boma, false, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    FT_MOTION_RATE(agent, 0.9);
    frame(lua_state, 11.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 25.0, 90, 100, 0, 0, 15.0, 0.0, 10.0, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 30, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
        ATTACK(agent, 1, 0, Hash40::new("top"), 25.0, 90, 100, 0, 0, 10.0, 0.0, 30.0, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 30, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
        ATTACK(agent, 2, 0, Hash40::new("top"), 25.0, 90, 100, 0, 0, 5.0, 0.0, 40.0, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 30, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
    }
    FT_MOTION_RATE(agent, 1.5);
    frame(lua_state, 15.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 3, 0, Hash40::new("top"), 25.0, 90, 100, 0, 0, 10.0, 0.0, 5.0, 30.0, Some(0.0), Some(15.0), Some(30.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
        ATTACK(agent, 4, 0, Hash40::new("top"), 25.0, 90, 100, 0, 0, 5.0, 0.0, 25.0, 30.0, Some(0.0), Some(25.0), Some(30.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_specialnendmax(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        KineticModule::set_consider_ground_friction(boma, false, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    FT_MOTION_RATE(agent, 0.9);
    frame(lua_state, 11.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 25.0, 90, 100, 0, 0, 15.0, 0.0, 10.0, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 30, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
        ATTACK(agent, 1, 0, Hash40::new("top"), 25.0, 90, 100, 0, 0, 10.0, 0.0, 30.0, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 30, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
        ATTACK(agent, 2, 0, Hash40::new("top"), 25.0, 90, 100, 0, 0, 5.0, 0.0, 40.0, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 30, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
    }
    FT_MOTION_RATE(agent, 1.5);
    frame(lua_state, 15.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 3, 0, Hash40::new("top"), 25.0, 90, 100, 0, 0, 10.0, 0.0, 5.0, 30.0, Some(0.0), Some(15.0), Some(30.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
        ATTACK(agent, 4, 0, Hash40::new("top"), 25.0, 90, 100, 0, 0, 5.0, 0.0, 25.0, 30.0, Some(0.0), Some(25.0), Some(30.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
    }
    FT_MOTION_RATE(agent, 1.5);
    frame(lua_state, 19.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 5, 0, Hash40::new("top"), 25.0, 90, 100, 0, 0, 10.0, 0.0, 5.0, 45.0, Some(0.0), Some(10.0), Some(45.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
        ATTACK(agent, 6, 0, Hash40::new("top"), 25.0, 90, 100, 0, 0, 5.0, 0.0, 20.0, 45.0, Some(0.0), Some(20.0), Some(45.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
    }
    FT_MOTION_RATE(agent, 1.5);
    frame(lua_state, 23.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_specialsstart(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    FT_MOTION_RATE(agent, 1.11111);
    frame(lua_state, 18.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        WorkModule::on_flag(boma, *FIGHTER_IKE_STATUS_SPECIAL_S_FLAG_CHARGE);
    }
}

unsafe extern "C" fn game_specialshold(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
    }
}

unsafe extern "C" fn game_specialsdash(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
    }
}

unsafe extern "C" fn effect_specialsdash(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::ike::instance::STORED_AETHER) {
            EFFECT_FOLLOW(agent, Hash40::new("ike_final_sword_fire"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, false);
        }
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 13, 4, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, true);
        LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW(agent, Hash40::new("ike_sword"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ike_iaigiri"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(boma);
    }
    frame(lua_state, 5.0);
    loop {
        if is_excute(agent) {
            FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
        }
        wait(lua_state, 4.0);
        if is_excute(agent) {
            if VarModule::is_flag(boma.object(), vars::ike::instance::STORED_AETHER) {
                EFFECT_FOLLOW(agent, Hash40::new("ike_final_sword_fire"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, false);
            }
            FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
        }
        wait(lua_state, 4.0);
    }
}

unsafe extern "C" fn effect_specialairsdash(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::ike::instance::STORED_AETHER) {
            EFFECT_FOLLOW(agent, Hash40::new("ike_final_sword_fire"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, false);
        } else {
            EFFECT_FOLLOW(agent, Hash40::new("ike_sword"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 13, 4, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ike_iaigiri"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(boma);
    }
    frame(lua_state, 9.0);
    loop {
        if is_excute(agent) {
            if VarModule::is_flag(boma.object(), vars::ike::instance::STORED_AETHER) {
                EFFECT_FOLLOW(agent, Hash40::new("ike_final_sword_fire"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, false);
            }
        }
        wait(lua_state, 9.0);
    }
}

unsafe extern "C" fn game_specialsattack(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    let coll_attr = if VarModule::is_flag(boma.object(), vars::ike::instance::STORED_AETHER) { Hash40::new("collision_attr_aura") } else { Hash40::new("collision_attr_cutup") };
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
    }
    FT_MOTION_RATE(agent, 5.0);
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 361, 100, 0, 0, 7.5, 0.0, 7.5, 15.0, Some(0.0), Some(7.5), Some(10.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, coll_attr, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        WorkModule::on_flag(boma, *FIGHTER_IKE_STATUS_SPECIAL_S_FLAG_ATTACK_END);
    }
    FT_MOTION_RATE(agent, 2.0);
    frame(lua_state, 4.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_specialsattack(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 1.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 6, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 0.5);
        if VarModule::is_flag(boma.object(), vars::ike::instance::STORED_AETHER) { 
            EFFECT_FOLLOW(agent, Hash40::new("ike_final_sword_fire"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, false);
            AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_ike_sword5"), Hash40::new("tex_ike_sword2"), 5, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("null"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        } else {
            EFFECT_FOLLOW(agent, Hash40::new("ike_sword"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
        }
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        if !VarModule::is_flag(boma.object(), vars::ike::instance::STORED_AETHER) { 
            if get_value_float(lua_state, *SO_VAR_FLOAT_LR) >= 0.0 { 
                EFFECT_FOLLOW(agent, Hash40::new("ike_iaigiri_attack"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
            } else {
                EFFECT_FOLLOW(agent, Hash40::new("ike_iaigiri_attack_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
            }
            EffectModule::set_disable_render_offset_last(boma);
        }
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        if !VarModule::is_flag(boma.object(), vars::ike::instance::STORED_AETHER) { 
            EFFECT_DETACH_KIND(agent, Hash40::new("ike_iaigiri_attack_r"), -1);
        }
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::ike::instance::STORED_AETHER) {
            AFTER_IMAGE_OFF(agent, 3);
        } else {
            EFFECT_OFF_KIND(agent, Hash40::new("ike_sword"), true, true);
        }
    }
}

unsafe extern "C" fn game_specialsend(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
}

unsafe extern "C" fn game_specialhi1(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
    }
}

unsafe extern "C" fn effect_specialhi1(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 5.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("sword"), 0, 8, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::ike::instance::STORED_AETHER) {
            EFFECT_FOLLOW(agent, Hash40::new("ike_final_sword_fire"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, false);
        }
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 3, 0, 5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        if !VarModule::is_flag(boma.object(), vars::ike::instance::STORED_AETHER) {
            EFFECT_FOLLOW(agent, Hash40::new("ike_sword"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
        }
    }
}

unsafe extern "C" fn game_specialhi2(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    let damage1 = if VarModule::is_flag(boma.object(), vars::ike::instance::STORED_AETHER) { 15.0 } else { 10.0 };
    let damage2 = if VarModule::is_flag(boma.object(), vars::ike::instance::STORED_AETHER) { 7.5 } else { 5.0 };
    let coll_attr = if VarModule::is_flag(boma.object(), vars::ike::instance::STORED_AETHER) { Hash40::new("collision_attr_aura") } else { Hash40::new("collision_attr_cutup") };
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_IKE_INSTANCE_WORK_ID_FLAG_SWORD_FINAL);
        ArticleModule::generate_article(boma, *FIGHTER_IKE_GENERATE_ARTICLE_SWORD, false, -1);
        ATTACK(agent, 0, 0, Hash40::new("top"), damage1, 90, 100, 150, 0, 5.0, 0.0, 5.0, 5.0, Some(0.0), Some(5.0), Some(7.5), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, coll_attr, *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), damage1, 90, 100, 150, 0, 2.5, 0.0, 2.5, 15.0, Some(0.0), Some(5.0), Some(15.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, coll_attr, *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("top"), damage1, 90, 100, 150, 0, 5.0, 0.0, 5.0, 20.0, Some(0.0), Some(5.0), Some(7.5), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, coll_attr, *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("top"), damage1, 90, 100, 150, 0, 5.0, 0.0, 15.0, 5.0, Some(0.0), Some(15.0), Some(7.5), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, coll_attr, *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 4, 0, Hash40::new("top"), damage1, 90, 100, 150, 0, 2.5, 0.0, 10.0, 15.0, Some(0.0), Some(15.0), Some(15.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, coll_attr, *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 5, 0, Hash40::new("top"), damage1, 90, 100, 150, 0, 5.0, 0.0, 15.0, 20.0, Some(0.0), Some(15.0), Some(7.5), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, coll_attr, *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_IKE_STATUS_SPECIAL_HI_FLAG_TRANS_JUMP);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_IKE_STATUS_SPECIAL_HI_FLAG_CONTROL);
        WorkModule::set_float(boma, 4.0, *FIGHTER_IKE_STATUS_SPECIAL_HI_WORK_FLOAT_SLIDEGAP_RECOVER_FRAME_INIT);
        WorkModule::set_float(boma, 4.0, *FIGHTER_IKE_STATUS_SPECIAL_HI_WORK_FLOAT_SLIDEGAP_RECOVER_FRAME);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), damage2, 367, 100, 50, 0, 10.0, 0.0, 15.0, 15.0, Some(0.0), Some(7.5), Some(15.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, coll_attr, *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 41.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 44.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 47.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_IKE_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn effect_specialhi2(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ike_tenku_start"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
        if VarModule::is_flag(boma.object(), vars::ike::instance::STORED_AETHER) {
            LAST_EFFECT_SET_COLOR(agent, 0, 50.0, 100.0);
        }else {
            EFFECT_OFF_KIND(agent, Hash40::new("ike_sword"), true, true);
        }
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ike_tenku_start"), true, true);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::ike::instance::STORED_AETHER) {
            EFFECT_FOLLOW(agent, Hash40::new("ike_final_sword_fire"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, false);
            AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_ike_sword5"), Hash40::new("tex_ike_sword2"), 4, Hash40::new("sword"), 0.0, 6.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("null"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
    }
    frame(lua_state, 38.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::ike::instance::STORED_AETHER) { 
            EFFECT_FOLLOW(agent, Hash40::new("ike_final_sword_fire"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, false);
        }
    }
}

unsafe extern "C" fn game_specialhi3(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    let damage = if VarModule::is_flag(boma.object(), vars::ike::instance::STORED_AETHER) { 7.5 } else { 5.0 };
    let coll_attr = if VarModule::is_flag(boma.object(), vars::ike::instance::STORED_AETHER) { Hash40::new("collision_attr_aura") } else { Hash40::new("collision_attr_cutup") };
    if is_excute(agent) {
        camera!(agent, *MA_MSC_CMD_CAMERA_CAM_OFFSET, 0, -20);
        KineticModule::clear_speed_all(boma);
        ADD_SPEED_NO_LIMIT(agent, 0, -5);
        ATTACK(agent, 0, 0, Hash40::new("sword"), damage, 270, 100, 150, 0, 5.0, 0.0, 7.5, 0.0, Some(0.0), Some(7.5), Some(-12.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, coll_attr, *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("sword"), damage, 270, 100, 150, 0, 5.0, 0.0, 7.5, 0.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, coll_attr, *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
}

unsafe extern "C" fn effect_specialhi3(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::ike::instance::STORED_AETHER) {
            EFFECT_FOLLOW(agent, Hash40::new("ike_final_sword_fire"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, false);
            AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_ike_sword5"), Hash40::new("tex_ike_sword2"), 10, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("null"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        } else {
            EFFECT_FOLLOW(agent, Hash40::new("ike_tenku_sword3"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(agent, Hash40::new("ike_sword"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
        }
    }
    frame(lua_state, 10.0);
    loop {
        if is_excute(agent) {
            if VarModule::is_flag(boma.object(), vars::ike::instance::STORED_AETHER) { 
                EFFECT_FOLLOW(agent, Hash40::new("ike_final_sword_fire"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, false);
            }
        }
        wait(lua_state, 10.0);
    }
}

unsafe extern "C" fn game_specialhi4(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    let damage = if VarModule::is_flag(boma.object(), vars::ike::instance::STORED_AETHER) { 7.5 } else { 5.0 };
    let coll_attr = if VarModule::is_flag(boma.object(), vars::ike::instance::STORED_AETHER) { Hash40::new("collision_attr_fire") } else { Hash40::new("collision_attr_cutup") };
    let coll_snd_attr = if VarModule::is_flag(boma.object(), vars::ike::instance::STORED_AETHER) { *COLLISION_SOUND_ATTR_FIRE } else { *COLLISION_SOUND_ATTR_IKE };
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), damage, 361, 250, 0, 0, 10.0, 0.0, 5.0, 12.5, Some(0.0), Some(10.0), Some(12.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, coll_attr, *ATTACK_SOUND_LEVEL_L, coll_snd_attr, *ATTACK_REGION_SWORD);
        if VarModule::is_flag(agent.object(), vars::ike::instance::STORED_AETHER) {
            ATTACK(agent, 1, 0, Hash40::new("top"), 7.5, 361, 250, 0, 0, 15.0, 0.0, 15.0, 12.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
            ATTACK(agent, 2, 0, Hash40::new("top"), 7.5, 361, 250, 0, 0, 10.0, 0.0, 25.0, 12.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
        }
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_specialhi4(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::ike::instance::STORED_AETHER) {
            AFTER_IMAGE_OFF(agent, 3);
            LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 10, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
            EFFECT(agent, Hash40::new("ike_volcano_max"), Hash40::new("top"), 0, 0, 12, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, true);
            LAST_EFFECT_SET_SCALE_W(agent, 1.2, 1, 1.2);
            EFFECT(agent, Hash40::new("ike_volcano_add2"), Hash40::new("top"), 0, 0, 12, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, true);
            EFFECT(agent, Hash40::new("ike_volcano_add4"), Hash40::new("top"), 0, 0, 12, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, true);
            EFFECT(agent, Hash40::new("ike_volcano_flash3"), Hash40::new("top"), 0, 0, 12, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, true);
        } else {
            EFFECT_OFF_KIND(agent, Hash40::new("ike_tenku_sword3"), false, false);
            EFFECT_FOLLOW(agent, Hash40::new("ike_sword"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
            LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 10, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
            EFFECT(agent, Hash40::new("ike_tenku_ground"), Hash40::new("top"), 10, 0, 0, 0, 180, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        if !VarModule::is_flag(boma.object(), vars::ike::instance::STORED_AETHER) { 
            EFFECT_OFF_KIND(agent, Hash40::new("ike_sword"), true, true);
        }
    }
}

unsafe extern "C" fn sound_specialhi4(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_ike_special_h05"));
        PLAY_SE(agent, Hash40::new("se_ike_special_h06"));
        PLAY_SE(agent, Hash40::new("se_ike_swordgroundhit"));
        if VarModule::is_flag(boma.object(), vars::ike::instance::STORED_AETHER) {
            PLAY_SE(agent, Hash40::new("se_ike_special_n08"));
        }
    }
}

unsafe extern "C" fn game_speciallw(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    FT_MOTION_RATE(agent, 0.55555);
    frame(lua_state, 9.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_IKE_STATUS_SPECIAL_LW_FLAG_SHIELD);
    }
    FT_MOTION_RATE(agent, 1.04);
    frame(lua_state, 34.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_IKE_STATUS_SPECIAL_LW_FLAG_SHIELD);
        HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_NORMAL);
    }
}

unsafe extern "C" fn game_speciallwhit(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    let damage = if VarModule::is_flag(boma.object(), vars::ike::instance::STORED_AETHER) { 25.0 } else { 10.0 };
    let coll_attr = if VarModule::is_flag(boma.object(), vars::ike::instance::STORED_AETHER) { Hash40::new("collision_attr_aura") } else { Hash40::new("collision_attr_cutup") };
    frame(lua_state, 5.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), damage, 361, 100, 0, 0, 10.0, 0.0, 10.0, 20.0, Some(0.0), Some(10.0), Some(5.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, coll_attr, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        AttackModule::set_force_reaction(boma, 0, true, false);
        if WorkModule::is_flag(boma, *FIGHTER_IKE_STATUS_SPECIAL_LW_FLAG_SPECIAL_EFFECT) {
            AttackModule::set_optional_hit_sound(boma, 0, Hash40::new("se_ike_criticalhit"));
        }
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_speciallwhit(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        if WorkModule::is_flag(boma, *FIGHTER_IKE_STATUS_SPECIAL_LW_FLAG_SPECIAL_EFFECT) {
            EFFECT(agent, Hash40::new("sys_counter_flash"), Hash40::new("top"), 0, 16, -1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.25, 0, 0, 0, 0, 0, 0, false);
        EFFECT(agent, Hash40::new("ike_counter_success"), Hash40::new("top"), 0, 16, -1, 0, 90, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 0.5);
        FLASH(agent, 1, 1, 1, 0);
        if VarModule::is_flag(boma.object(), vars::ike::instance::STORED_AETHER) {
            AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_ike_sword5"), Hash40::new("tex_ike_sword2"), 5, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("null"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.5, 0.1);
            EFFECT_FOLLOW(agent, Hash40::new("ike_counter_attack"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
        } else {
            AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_ike_sword6"), Hash40::new("tex_ike_sword2"), 5, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.5, 0.1);
        }
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 4);
        COL_NORMAL(agent);
    }
}

unsafe extern "C" fn effect_specialairlwhit(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        if WorkModule::is_flag(boma, *FIGHTER_IKE_STATUS_SPECIAL_LW_FLAG_SPECIAL_EFFECT) {
            EFFECT(agent, Hash40::new("sys_counter_flash"), Hash40::new("top"), 0, 16, -1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        EFFECT(agent, Hash40::new("ike_counter_success"), Hash40::new("top"), 0, 16, -1, 0, 90, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        LAST_EFFECT_SET_RATE(agent, 0.5);
        FLASH(agent, 1, 1, 1, 0);
        if VarModule::is_flag(boma.object(), vars::ike::instance::STORED_AETHER) {
            AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_ike_sword5"), Hash40::new("tex_ike_sword2"), 5, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("null"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.5, 0.1);
            EFFECT_FOLLOW(agent, Hash40::new("ike_counter_attack"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
        }else {
            AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_ike_sword6"), Hash40::new("tex_ike_sword2"), 5, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.5, 0.1);
        }
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 4);
        COL_NORMAL(agent);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialnstart", game_specialnstart, Priority::Low);
    agent.acmd("game_specialairnstart", game_specialnstart, Priority::Low);
    agent.acmd("game_specialnloop", game_specialnloop, Priority::Low);
    agent.acmd("game_specialairnloop", game_specialnloop, Priority::Low);
    agent.acmd("effect_specialnloop", effect_specialnloop, Priority::Low);
    agent.acmd("effect_specialairnloop", effect_specialairnloop, Priority::Low);
    agent.acmd("game_specialnend", game_specialnend, Priority::Low);
    agent.acmd("game_specialairnend", game_specialnend, Priority::Low);
    agent.acmd("game_specialnendmdl", game_specialnendmdl, Priority::Low);
    agent.acmd("game_specialairnendmdl", game_specialnendmdl, Priority::Low);
    agent.acmd("game_specialnendmax", game_specialnendmax, Priority::Low);
    agent.acmd("game_specialairnendmax", game_specialnendmax, Priority::Low);
    agent.acmd("game_specialsstart", game_specialsstart, Priority::Low);
    agent.acmd("game_specialairsstart", game_specialsstart, Priority::Low);
    agent.acmd("game_specialshold", game_specialshold, Priority::Low);
    agent.acmd("game_specialairshold", game_specialshold, Priority::Low);
    agent.acmd("game_specialsdash", game_specialsdash, Priority::Low);
    agent.acmd("game_specialairsdash", game_specialsdash, Priority::Low);
    agent.acmd("effect_specialsdash", effect_specialsdash, Priority::Low);
    agent.acmd("effect_specialairsdash", effect_specialairsdash, Priority::Low);
    agent.acmd("game_specialsattack", game_specialsattack, Priority::Low);
    agent.acmd("game_specialairsattack", game_specialsattack, Priority::Low);
    agent.acmd("effect_specialsattack", effect_specialsattack, Priority::Low);
    agent.acmd("effect_specialairsattack", effect_specialsattack, Priority::Low);
    agent.acmd("game_specialsend", game_specialsend, Priority::Low);
    agent.acmd("game_specialairsend", game_specialsend, Priority::Low);
    agent.acmd("game_specialhi1", game_specialhi1, Priority::Low);
    agent.acmd("effect_specialhi1", effect_specialhi1, Priority::Low);
    agent.acmd("effect_specialairhi1", effect_specialhi1, Priority::Low);
    agent.acmd("game_specialhi2", game_specialhi2, Priority::Low);
    agent.acmd("game_specialairhi2", game_specialhi2, Priority::Low);
    agent.acmd("effect_specialhi2", effect_specialhi2, Priority::Low);
    agent.acmd("effect_specialairhi2", effect_specialhi2, Priority::Low);
    agent.acmd("game_specialhi3", game_specialhi3, Priority::Low);
    agent.acmd("effect_specialhi3", effect_specialhi3, Priority::Low);
    agent.acmd("effect_specialairhi3", effect_specialhi3, Priority::Low);
    agent.acmd("game_specialhi4", game_specialhi4, Priority::Low);
    agent.acmd("effect_specialhi4", effect_specialhi4, Priority::Low);
    agent.acmd("effect_specialairhi4", effect_specialhi4, Priority::Low);
    agent.acmd("game_speciallw", game_speciallw, Priority::Low);
    agent.acmd("game_specialairlw", game_speciallw, Priority::Low);
    agent.acmd("game_speciallwhit", game_speciallwhit, Priority::Low);
    agent.acmd("game_specialairlwhit", game_speciallwhit, Priority::Low);
    agent.acmd("effect_speciallwhit", effect_speciallwhit, Priority::Low);
    agent.acmd("effect_specialairlwhit", effect_specialairlwhit, Priority::Low);
}