use super::*;

unsafe extern "C" fn game_attacks4(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 4.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    FT_MOTION_RATE(agent, 0.66666);
    frame(lua_state, 10.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("armr"), if VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE) {12.0} else {16.0}, 361, 100, 0, 0, 6.0, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(if VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE) {"collision_attr_ice"} else {"collision_attr_fire"}), *ATTACK_SOUND_LEVEL_L, if VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE) {*COLLISION_SOUND_ATTR_FREEZE} else {*COLLISION_SOUND_ATTR_FIRE}, *ATTACK_REGION_BOMB);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_attacks4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("armr"), 4.289, -0.272, -0.135, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 11.5, 3, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
        LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        if VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE) {
            EFFECT(agent, Hash40::new("sys_hit_ice"), Hash40::new("armr"), 14.387, -0.341, -0.169, 0, 0, 0, 1.04, 0, 0, 0, 0, 0, 0, true);
        } else {
            EFFECT(agent, Hash40::new("samus_atk_bomb"), Hash40::new("armr"), 14.387, -0.341, -0.169, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
        }
    }
}

unsafe extern "C" fn game_attacks4hi(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 4.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    FT_MOTION_RATE(agent, 0.66666);
    frame(lua_state, 10.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("armr"), if VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE) {12.0} else {16.0}, 361, 100, 0, 0, 6.0, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(if VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE) {"collision_attr_ice"} else {"collision_attr_fire"}), *ATTACK_SOUND_LEVEL_L, if VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE) {*COLLISION_SOUND_ATTR_FREEZE} else {*COLLISION_SOUND_ATTR_FIRE}, *ATTACK_REGION_BOMB);    
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_attacks4hi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("armr"), 4.289, -0.272, -0.135, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 10, 3, -32, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
        LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        if VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE) {
            EFFECT(agent, Hash40::new("sys_hit_ice"), Hash40::new("armr"), 14.387, -0.341, -0.169, 0, 0, 0, 1.04, 0, 0, 0, 0, 0, 0, true);
        } else {
            EFFECT(agent, Hash40::new("samus_atk_bomb"), Hash40::new("armr"), 14.387, -0.341, -0.169, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
        }
    }
}

unsafe extern "C" fn game_attacks4lw(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 4.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    FT_MOTION_RATE(agent, 0.66666);
    frame(lua_state, 10.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("armr"), if VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE) {12.0} else {16.0}, 361, 100, 0, 0, 6.0, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(if VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE) {"collision_attr_ice"} else {"collision_attr_fire"}), *ATTACK_SOUND_LEVEL_L, if VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE) {*COLLISION_SOUND_ATTR_FREEZE} else {*COLLISION_SOUND_ATTR_FIRE}, *ATTACK_REGION_BOMB);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_attacks4lw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("armr"), 4.289, -0.272, -0.135, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 11.5, 3, 28, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
        LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        if VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE) {
            EFFECT(agent, Hash40::new("sys_hit_ice"), Hash40::new("armr"), 14.387, -0.341, -0.169, 0, 0, 0, 1.04, 0, 0, 0, 0, 0, 0, true);
        } else {
            EFFECT(agent, Hash40::new("samus_atk_bomb"), Hash40::new("armr"), 14.387, -0.341, -0.169, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
        }
    }
}

unsafe extern "C" fn sound_attacks4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 5.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_common_smash_start"));
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        if !VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE) {
            PLAY_SE(agent, Hash40::new("se_samus_smash_s01"));
        }
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE) {
            PLAY_SE(agent, Hash40::new("se_item_ice_crash"));
        }
    }
    wait(lua_state, 7.0);
    if is_excute(agent) {
        if !VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE) {
            PLAY_SE(agent, Hash40::new("se_samus_smash_s02"));
        }
    }
}

unsafe extern "C" fn game_attackhi4(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 7.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    FT_MOTION_RATE(agent, 0.25);
    frame(lua_state, 11.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("armr"), if VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE) {12.0} else {16.0}, 80, 100, 0, 0, 6.0, -4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(if VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE) {"collision_attr_ice"} else {"collision_attr_fire"}), *ATTACK_SOUND_LEVEL_L, if VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE) {*COLLISION_SOUND_ATTR_FREEZE} else {*COLLISION_SOUND_ATTR_FIRE}, *ATTACK_REGION_BOMB);
        ATTACK(agent, 1, 0, Hash40::new("armr"), if VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE) {12.0} else {16.0}, 80, 100, 0, 0, 6.0, 6.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(if VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE) {"collision_attr_ice"} else {"collision_attr_fire"}), *ATTACK_SOUND_LEVEL_L, if VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE) {*COLLISION_SOUND_ATTR_FREEZE} else {*COLLISION_SOUND_ATTR_FIRE}, *ATTACK_REGION_BOMB);
        ATTACK(agent, 2, 0, Hash40::new("armr"), if VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE) {12.0} else {16.0}, 80, 100, 0, 0, 6.0, 6.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(if VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE) {"collision_attr_ice"} else {"collision_attr_fire"}), *ATTACK_SOUND_LEVEL_L, if VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE) {*COLLISION_SOUND_ATTR_FREEZE} else {*COLLISION_SOUND_ATTR_FIRE}, *ATTACK_REGION_BOMB);
    }
    FT_MOTION_RATE(agent, 0.44444);
    wait(lua_state, 18.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_attackhi4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 6.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("armr"), 4.289, -0.272, -0.135, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE) {
            EFFECT(agent, Hash40::new("sys_hit_ice"), Hash40::new("armr"), 14.387, -0.341, -0.169, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        } else {
            EFFECT(agent, Hash40::new("samus_atk_bomb"), Hash40::new("armr"), 14.387, -0.341, -0.169, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_v"), Hash40::new("top"), 0, 0, 3, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE) {
            EFFECT(agent, Hash40::new("sys_hit_ice"), Hash40::new("armr"), 14.387, -0.341, -0.169, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        } else {
            EFFECT(agent, Hash40::new("samus_atk_bomb"), Hash40::new("armr"), 14.387, -0.341, -0.169, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE) {
            EFFECT(agent, Hash40::new("sys_hit_ice"), Hash40::new("armr"), 14.387, -0.341, -0.169, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        } else {
            EFFECT(agent, Hash40::new("samus_atk_bomb"), Hash40::new("armr"), 14.387, -0.341, -0.169, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE) {
            EFFECT(agent, Hash40::new("sys_hit_ice"), Hash40::new("armr"), 14.387, -0.341, -0.169, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        } else {
            EFFECT(agent, Hash40::new("samus_atk_bomb"), Hash40::new("armr"), 14.387, -0.341, -0.169, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE) {
            EFFECT(agent, Hash40::new("sys_hit_ice"), Hash40::new("armr"), 14.387, -0.341, -0.169, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        } else {
            EFFECT(agent, Hash40::new("samus_atk_bomb"), Hash40::new("armr"), 14.387, -0.341, -0.169, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    }
}

unsafe extern "C" fn sound_attackhi4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 11.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_common_smash_start"));
        if VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE) {
            PLAY_SE(agent, Hash40::new("se_item_ice_crash"));
        } else {
            PLAY_SE(agent, Hash40::new("se_samus_smash_h02"));
        }
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE) {
            PLAY_SE(agent, Hash40::new("se_item_ice_crash"));
        } else {
            PLAY_SE(agent, Hash40::new("se_samus_smash_h01"));
        }
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE) {
            PLAY_SE(agent, Hash40::new("se_item_ice_crash"));
        } else {
            PLAY_SE(agent, Hash40::new("se_samus_smash_h01"));
        }
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE) {
            PLAY_SE(agent, Hash40::new("se_item_ice_crash"));
        } else {
            PLAY_SE(agent, Hash40::new("se_samus_smash_h01"));
        }
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE) {
            PLAY_SE(agent, Hash40::new("se_item_ice_crash"));
        } else {
            PLAY_SE(agent, Hash40::new("se_samus_smash_h02"));
        }
    }
}

unsafe extern "C" fn game_attacklw4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    FT_MOTION_RATE(agent, 0.4);
    frame(lua_state, 9.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("kneer"), 12.0, 20, 100, 0, 0, 4.0, 8.0, 0.0, 0.0, Some(-2.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(boma, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("kneer"), 12.0, 20, 100, 0, 0, 4.0, 8.0, 0.0, 0.0, Some(-2.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(boma, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attacks4", game_attacks4, Priority::Low);
    agent.acmd("effect_attacks4", effect_attacks4, Priority::Low);
    agent.acmd("sound_attacks4", sound_attacks4, Priority::Low);
    agent.acmd("game_attacks4hi", game_attacks4hi, Priority::Low);
    agent.acmd("effect_attacks4hi", effect_attacks4hi, Priority::Low);
    agent.acmd("sound_attacks4hi", sound_attacks4, Priority::Low);
    agent.acmd("game_attacks4lw", game_attacks4lw, Priority::Low);
    agent.acmd("effect_attacks4lw", effect_attacks4lw, Priority::Low);
    agent.acmd("sound_attacks4lw", sound_attacks4, Priority::Low);
    agent.acmd("game_attackhi4", game_attackhi4, Priority::Low);
    agent.acmd("effect_attackhi4", effect_attackhi4, Priority::Low);
    agent.acmd("sound_attackhi4", sound_attackhi4, Priority::Low);
    agent.acmd("game_attacklw4", game_attacklw4, Priority::Low);
}