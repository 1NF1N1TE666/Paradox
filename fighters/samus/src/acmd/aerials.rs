use super::*;

unsafe extern "C" fn game_attackairn(agent : &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 4.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 367, 100, 40, 0, 3.2, 0.0, 2.0, 4.0, Some(0.0), Some(2.0), Some(4.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 1.0, 367, 100, 40, 0, 3.2, 0.0, 2.0, -4.0, Some(0.0), Some(2.0), Some(-4.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
        ATTACK(agent, 2, 0, Hash40::new("top"), 1.0, 367, 100, 40, 0, 3.2, 0.0, 10.0, 4.0, Some(0.0), Some(10.0), Some(4.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
        ATTACK(agent, 3, 0, Hash40::new("top"), 1.0, 367, 100, 40, 0, 3.2, 0.0, 10.0, -4.0, Some(0.0), Some(10.0), Some(-4.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 60, 200, 0, 0, 8.0, 0.0, 6.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SAMUS_SCREW_FINISH, *ATTACK_REGION_BODY);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_attackairn(agent : &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 4.0);
    if is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, Hash40::new("samus_screwattack"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 0.6, true, 0.471, 0.471, 0.471, 16, 1, 1, 0);
        sv_animcmd::EFFECT_FOLLOW_LIGHT(lua_state);
        agent.clear_lua_stack();
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("samus_screwattack"), false, true);
        agent.clear_lua_stack();
        lua_args!(agent, 3, *EFFECT_SUB_ATTRIBUTE_FOLLOW);
        sv_animcmd::EFFECT_LIGHT_END(lua_state);
        agent.clear_lua_stack();
    }
}

unsafe extern "C" fn sound_attackairn(agent : &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 5.0);
    if is_excute(agent) {
        PLAY_STATUS(agent, Hash40::new("se_samus_special_h01"));
    }
    wait(lua_state, 25.0);
    if is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, *MA_MSC_CMD_SOUND_STOP_SE_STATUS);
        sv_module_access::sound(lua_state);
        agent.clear_lua_stack();
    }
}

unsafe extern "C" fn expression_attackairn(agent : &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_spinattack"), 6);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_spinattacks"), 0, true, 0);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_erase"), 0, false, 0);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
}

unsafe extern "C" fn game_attackairf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FT_MOTION_RATE(agent, 0.66666);
    frame(lua_state, 6.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("armr"), if VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE) {12.0} else {16.0}, 361, 100, 0, 0, 8.0, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(if VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE) {"collision_attr_ice"} else {"collision_attr_fire"}), *ATTACK_SOUND_LEVEL_L, if VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE) {*COLLISION_SOUND_ATTR_FREEZE} else {*COLLISION_SOUND_ATTR_FIRE}, *ATTACK_REGION_BOMB);
        ATTACK(agent, 1, 0, Hash40::new("armr"), if VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE) {12.0} else {16.0}, 361, 100, 0, 0, 6.0, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(if VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE) {"collision_attr_ice"} else {"collision_attr_fire"}), *ATTACK_SOUND_LEVEL_L, if VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE) {*COLLISION_SOUND_ATTR_FREEZE} else {*COLLISION_SOUND_ATTR_FIRE}, *ATTACK_REGION_BOMB);
    }
    FT_MOTION_RATE(agent, 0.46);
    wait(lua_state, 26.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_attackairf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE) {
            EFFECT(agent, Hash40::new("sys_hit_ice"), Hash40::new("armr"), 16.5, -0.341, -0.169, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        } else {
            EFFECT(agent, Hash40::new("samus_atk_bomb"), Hash40::new("armr"), 16.5, -0.341, -0.169, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE) {
            EFFECT(agent, Hash40::new("sys_hit_ice"), Hash40::new("armr"), 16.5, -0.341, -0.169, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        } else {
            EFFECT(agent, Hash40::new("samus_atk_bomb"), Hash40::new("armr"), 16.5, -0.341, -0.169, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE) {
            EFFECT(agent, Hash40::new("sys_hit_ice"), Hash40::new("armr"), 16.5, -0.341, -0.169, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        } else {
            EFFECT(agent, Hash40::new("samus_atk_bomb"), Hash40::new("armr"), 16.5, -0.341, -0.169, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE) {
            EFFECT(agent, Hash40::new("sys_hit_ice"), Hash40::new("armr"), 16.5, -0.341, -0.169, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        } else {
            EFFECT(agent, Hash40::new("samus_atk_bomb"), Hash40::new("armr"), 16.5, -0.341, -0.169, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE) {
            EFFECT(agent, Hash40::new("sys_hit_ice"), Hash40::new("armr"), 16.5, -0.341, -0.169, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        } else {
            EFFECT(agent, Hash40::new("samus_atk_bomb"), Hash40::new("armr"), 16.5, -0.341, -0.169, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    }
}

unsafe extern "C" fn sound_attackairf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 6.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE) {
            PLAY_SE(agent, Hash40::new("se_item_ice_crash"));
        } else {
            PLAY_SE(agent, Hash40::new("se_samus_smash_h01"));
        }
    }
    wait(lua_state, 6.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE) {
            PLAY_SE(agent, Hash40::new("se_item_ice_crash"));
        } else {
            PLAY_SE(agent, Hash40::new("se_samus_smash_h01"));
        }
    }
    wait(lua_state, 6.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE) {
            PLAY_SE(agent, Hash40::new("se_item_ice_crash"));
        } else {
            PLAY_SE(agent, Hash40::new("se_samus_smash_h01"));
        }
    }
    wait(lua_state, 6.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE) {
            PLAY_SE(agent, Hash40::new("se_item_ice_crash"));
        } else {
            PLAY_SE(agent, Hash40::new("se_samus_smash_h01"));
        }
    }
    wait(lua_state, 6.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE) {
            PLAY_SE(agent, Hash40::new("se_item_ice_crash"));
        } else {
            PLAY_SE(agent, Hash40::new("se_samus_smash_h02"));
        }
    }
}

unsafe extern "C" fn game_attackairb(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    FT_MOTION_RATE(agent, 0.88888);
    frame(lua_state, 9.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("legr"), 16.0, 361, 100, 0, 0, 4.0, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneer"), 16.0, 361, 100, 0, 0, 4.0, 6.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        ATK_POWER(agent, 0, 10);
        ATK_POWER(agent, 1, 10);
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_attackairhi(agent : &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    FT_MOTION_RATE(agent, 0.88888);
    frame(lua_state, 9.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        boma.on_flag(*FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
        ATTACK(agent, 2, 0, Hash40::new("armr"), if VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE) {12.0} else {16.0}, 80, 100, 0, 0, 6.0, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(if VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE) {"collision_attr_ice"} else {"collision_attr_fire"}), *ATTACK_SOUND_LEVEL_L, if VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE) {*COLLISION_SOUND_ATTR_FREEZE} else {*COLLISION_SOUND_ATTR_FIRE}, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_attackairhi(agent : &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 9.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE) {
            EFFECT(agent, Hash40::new("sys_hit_ice"), Hash40::new("armr"), 14.387, -0.341, -0.169, 0, 0, 0, 1.04, 0, 0, 0, 0, 0, 0, true);
        } else {
            EFFECT(agent, Hash40::new("samus_atk_bomb"), Hash40::new("armr"), 14.387, -0.341, -0.169, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
        }
    }
}

unsafe extern "C" fn sound_attackairhi(agent : &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 9.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE) {
            PLAY_SE(agent, Hash40::new("se_item_ice_crash"));
        } else {
            PLAY_SE(agent, Hash40::new("se_samus_attackhard_l01"));
        }
    }
}

unsafe extern "C" fn expression_attackairhi(agent : &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 9.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_explosion"), 0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohit_explosion"), 0, false, 0);
    }
}

unsafe extern "C" fn game_attackairlw(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    FT_MOTION_RATE(agent, 0.25);
    frame(lua_state, 17.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("armr"), 12.0, 270, 100, 0, 0, 4.0, -2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("armr"), 12.0, 270, 100, 0, 0, 6.0, 6.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    FT_MOTION_RATE(agent, 1.25);
    frame(lua_state, 24.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attackairn", game_attackairn, Priority::Low);
    agent.acmd("effect_attackairn", effect_attackairn, Priority::Low);
    agent.acmd("sound_attackairn", sound_attackairn, Priority::Low);
    agent.acmd("expression_attackairn", expression_attackairn, Priority::Low);
    agent.acmd("game_attackairf", game_attackairf, Priority::Low);
    agent.acmd("effect_attackairf", effect_attackairf, Priority::Low);
    agent.acmd("sound_attackairf", sound_attackairf, Priority::Low);
    agent.acmd("game_attackairb", game_attackairb, Priority::Low);
    agent.acmd("game_attackairhi", game_attackairhi, Priority::Low);
    agent.acmd("effect_attackairhi", effect_attackairhi, Priority::Low);
    agent.acmd("sound_attackairhi", sound_attackairhi, Priority::Low);
    agent.acmd("expression_attackairhi", expression_attackairhi, Priority::Low);
    agent.acmd("game_attackairlw", game_attackairlw, Priority::Low);
}