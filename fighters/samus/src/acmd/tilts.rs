use super::*;

unsafe extern "C" fn game_attacks3(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    FT_MOTION_RATE(agent, 0.85);
    frame(lua_state, 7.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("armr"), 12.0, 361, 100, 0, 0, 6.0, 6.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("armr"), 12.0, 361, 100, 0, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_height_all(boma, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
        HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_INVINCIBLE);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 2, 0, Hash40::new("top"), 12.0, 361, 100, 0, 0, 6.0, 0.0, 12.0, 6.0, Some(0.0), Some(12.0),Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        AttackModule::clear(boma, 2, false);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        HitModule::set_status_all(boma, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

unsafe extern "C" fn effect_attacks3(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), -6, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 14, 4, 0, 25, 90, 1, true);
    }
}

unsafe extern "C" fn sound_attacks3(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 7.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_samus_swing_m"));
    }
}

unsafe extern "C" fn expression_attacks3(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

unsafe extern "C" fn game_attackhi3(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    FT_MOTION_RATE(agent, 0.85);
    frame(lua_state, 7.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("kneer"), 12.0, 80, 100, 0, 0, 4.0, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneer"), 12.0, 80, 100, 0, 0, 6.0, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(boma, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
        HIT_NODE(agent, Hash40::new("kneer"), *HIT_STATUS_XLU);
    }
    wait(lua_state, 6.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_attackhi3(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 4.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 15, -2, 0, 60, 90, 1.4, true);
        EFFECT_FOLLOW(agent, Hash40::new("samus_atk_air_lw"), Hash40::new("top"), 0, 15, 0, 0, 0, 90, 1.1, true);
        LAST_EFFECT_SET_RATE(agent, 0.8);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), -9, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_attackhi3(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 4.0);
    if is_excute(agent) {
        PLAY_STATUS(agent, Hash40::new("se_samus_swing_l"));
        PLAY_SE(agent, Hash40::new("se_samus_attackhard_h01"));
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_samus_step_right_m"));
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_samus_landing01"));
    }
}

unsafe extern "C" fn expression_attackhi3(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 8);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 3, true);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_lands"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 6);
    }
}

unsafe extern "C" fn game_attacklw3(agent : &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    FT_MOTION_RATE(agent, 0.66666);
    frame(lua_state, 9.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        if boma.is_situation(*SITUATION_KIND_GROUND) {
            ATTACK(agent, 0, 0, Hash40::new("top"), if VarModule::is_flag(boma.object(), vars::samus::instance::SPEEDBOOSTER_ON) {12.0} else {8.0}, 80, 100, if VarModule::is_flag(boma.object(), vars::samus::instance::SPEEDBOOSTER_ON) {120} else {80}, 0, if VarModule::is_flag(boma.object(), vars::samus::instance::SPEEDBOOSTER_ON) {6.0} else {4.0}, 0.0, 4.0, 12.0, Some(0.0), Some(4.0), Some(-4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(if VarModule::is_flag(boma.object(), vars::samus::instance::SPEEDBOOSTER_ON) {"collision_attr_elec_whip"} else {"collision_attr_normal"}), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            AttackModule::set_attack_height_all(boma, AttackHeight(*ATTACK_HEIGHT_LOW), false);
            HIT_NODE(agent, Hash40::new("kneer"), *HIT_STATUS_XLU);
        }
    }
    wait(lua_state, 20.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        HitModule::set_status_all(boma, HitStatus(*HIT_STATUS_NORMAL), 0);
        VarModule::on_flag(boma.object(), vars::samus::status::ATTACK_LW3_CHECK_CEIL);
    }
}

unsafe extern "C" fn effect_attacklw3(agent : &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 6.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::samus::instance::SPEEDBOOSTER_ON) {
            EFFECT_FOLLOW(agent, Hash40::new("sys_damage_spark"), Hash40::new("top"), -1, 0, 7, 135, 25, 0, 0.5, true);
            if boma.is_situation(*SITUATION_KIND_GROUND) {
                EFFECT_FOLLOW(agent, Hash40::new("sys_damage_spark"), Hash40::new("top"), 3, 0, 8, 135, 25, 0, 0.5, true);
                EFFECT_FOLLOW(agent, Hash40::new("sys_damage_spark"), Hash40::new("top"), 4, 0, -8, 135, 25, 0, 0.5, true);
            }
        }
    }
    frame(lua_state, 8.0);
    if VarModule::is_flag(boma.object(), vars::samus::instance::SPEEDBOOSTER_ON) {
        for _ in 0..4 {
            if is_excute(agent) {
                if boma.is_situation(*SITUATION_KIND_GROUND) {
                    EFFECT_FOLLOW(agent, Hash40::new("sys_damage_spark"), Hash40::new("top"), -1, 0, 11, 135, 25, 0, 0.5, true);
                    EFFECT_FOLLOW(agent, Hash40::new("sys_damage_spark"), Hash40::new("top"), 3, 0, 6, 135, 25, 0, 0.5, true);
                    EFFECT_FOLLOW(agent, Hash40::new("sys_damage_spark"), Hash40::new("top"), 0, 0, 0, 135, 25, 0, 0.5, true);
                    EFFECT_FOLLOW(agent, Hash40::new("sys_damage_spark"), Hash40::new("top"), 4, 0, -8, 135, 25, 0, 0.5, true);
                }
            }
            wait(lua_state, 4.0);
        }
    } else {
        if is_excute(agent) {
            LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 0.8, 0, 0, 4, 0, 0, 0, false);
            LAST_EFFECT_SET_RATE(agent, 1.2);
        }
        wait(lua_state, 2.0);
        if is_excute(agent) {
            if boma.is_situation(*SITUATION_KIND_GROUND) {
                LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 0.5, 0, 0, 5, 0, 0, 0, false);
            }
        }
        wait(lua_state, 2.0);
        if is_excute(agent) {
            if boma.is_situation(*SITUATION_KIND_GROUND) {
                FOOT_EFFECT(agent, Hash40::new("sys_sliding_smoke"), Hash40::new("top"), 6, 0, -3, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false);
                LAST_EFFECT_SET_RATE(agent, 1.4);
            }
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            if boma.is_situation(*SITUATION_KIND_GROUND) {
                LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 8, 0, -3, 0, 0, 0, 0.45, 0, 0, 5, 0, 0, 0, false);
            }
        }
        wait(lua_state, 3.0);
        if is_excute(agent) {
            if boma.is_situation(*SITUATION_KIND_GROUND) {
                LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 7, 0, -3, 0, 0, 0, 0.35, 0, 0, 5, 0, 0, 0, false);
            }
        }
        wait(lua_state, 3.0);
        if is_excute(agent) {
            if boma.is_situation(*SITUATION_KIND_GROUND) {
                LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 8, 0, -3, 0, 0, 0, 0.35, 0, 0, 5, 0, 0, 0, false);
            }
        }
        wait(lua_state, 4.0);
        if is_excute(agent) {
            if boma.is_situation(*SITUATION_KIND_GROUND) {
                LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 7, 0, -3, 0, 0, 0, 0.35, 0, 0, 5, 0, 0, 0, false);
            }
        }
    }
}

unsafe extern "C" fn sound_attacklw3(agent : &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 8.0);
    if is_excute(agent) {
        PLAY_STATUS(agent, Hash40::new("se_samus_swing_m"));
    }
}

unsafe extern "C" fn expression_attacklw3(agent : &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        if boma.is_situation(*SITUATION_KIND_GROUND) {
            slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 5);
        }
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        if boma.is_situation(*SITUATION_KIND_GROUND) {
            slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 2, true);
        }
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        if boma.is_situation(*SITUATION_KIND_GROUND) {
            RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
            ControlModule::set_rumble(boma, Hash40::new("rbkind_dash"), 0, false, 0);
        }
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        if boma.is_situation(*SITUATION_KIND_GROUND) {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_dash"), 0, false, 0);
        }
    }
    frame(lua_state, 36.0);
    if is_excute(agent) {
        if boma.is_situation(*SITUATION_KIND_GROUND) {
            slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 6);
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attacks3", game_attacks3, Priority::Low);
    agent.acmd("effect_attacks3", effect_attacks3, Priority::Low);
    agent.acmd("sound_attacks3", sound_attacks3, Priority::Low);
    agent.acmd("expression_attacks3", expression_attacks3, Priority::Low);
    agent.acmd("game_attackhi3", game_attackhi3, Priority::Low);
    agent.acmd("effect_attackhi3", effect_attackhi3, Priority::Low);
    agent.acmd("sound_attackhi3", sound_attackhi3, Priority::Low);
    agent.acmd("expression_attackhi3", expression_attackhi3, Priority::Low);
    agent.acmd("game_attacklw3", game_attacklw3, Priority::Low);
    agent.acmd("effect_attacklw3", effect_attacklw3, Priority::Low);
    agent.acmd("sound_attacklw3", sound_attacklw3, Priority::Low);
    agent.acmd("expression_attacklw3", expression_attacklw3, Priority::Low);
}