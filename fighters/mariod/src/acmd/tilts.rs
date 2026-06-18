use super::*;

unsafe extern "C" fn game_attacks3(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    FT_MOTION_RATE(agent, 0.8);
    frame(lua_state, 5.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("kneer"), 4.0, 361, 200, 0, 0, 4.0, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("legr"), 4.0, 361, 200, 0, 0, 4.0, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("hip"), 4.0, 361, 200, 0, 0, 2.0, 2.0, -2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    FT_MOTION_RATE(agent, 2.0);
    wait(lua_state, 1.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATK_POWER(agent, 0, 6);
        ATK_POWER(agent, 1, 6);
        ATK_POWER(agent, 2, 6);
    }
    FT_MOTION_RATE(agent, 2.0);
    wait(lua_state, 1.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATK_POWER(agent, 0, 8);
        ATK_POWER(agent, 1, 8);
        ATK_POWER(agent, 2, 8);
    }
    FT_MOTION_RATE(agent, 2.0);
    wait(lua_state, 1.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATK_POWER(agent, 0, 10);
        ATK_POWER(agent, 1, 10);
        ATK_POWER(agent, 2, 10);
    }
    FT_MOTION_RATE(agent, 2.0);
    wait(lua_state, 1.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATK_POWER(agent, 0, 12);
        ATK_POWER(agent, 1, 12);
        ATK_POWER(agent, 2, 12);
    }
    FT_MOTION_RATE(agent, 2.0);
    wait(lua_state, 1.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATK_POWER(agent, 0, 14);
        ATK_POWER(agent, 1, 14);
        ATK_POWER(agent, 2, 14);
    }
    FT_MOTION_RATE(agent, 2.0);
    wait(lua_state, 1.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATK_POWER(agent, 0, 16);
        ATK_POWER(agent, 1, 16);
        ATK_POWER(agent, 2, 16);
    }
    FT_MOTION_RATE(agent, 2.0);
    wait(lua_state, 1.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_attacks3hi(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    FT_MOTION_RATE(agent, 0.8);
    frame(lua_state, 5.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("kneer"), 4.0, 361, 200, 0, 0, 4.0, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("legr"), 4.0, 361, 200, 0, 0, 4.0, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("hip"), 4.0, 361, 200, 0, 0, 2.0, 2.0, -2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    FT_MOTION_RATE(agent, 2.0);
    wait(lua_state, 1.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATK_POWER(agent, 0, 6);
        ATK_POWER(agent, 1, 6);
        ATK_POWER(agent, 2, 6);
    }
    FT_MOTION_RATE(agent, 2.0);
    wait(lua_state, 1.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATK_POWER(agent, 0, 8);
        ATK_POWER(agent, 1, 8);
        ATK_POWER(agent, 2, 8);
    }
    FT_MOTION_RATE(agent, 2.0);
    wait(lua_state, 1.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATK_POWER(agent, 0, 10);
        ATK_POWER(agent, 1, 10);
        ATK_POWER(agent, 2, 10);
    }
    FT_MOTION_RATE(agent, 2.0);
    wait(lua_state, 1.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATK_POWER(agent, 0, 12);
        ATK_POWER(agent, 1, 12);
        ATK_POWER(agent, 2, 12);
    }
    FT_MOTION_RATE(agent, 2.0);
    wait(lua_state, 1.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATK_POWER(agent, 0, 14);
        ATK_POWER(agent, 1, 14);
        ATK_POWER(agent, 2, 14);
    }
    FT_MOTION_RATE(agent, 2.0);
    wait(lua_state, 1.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATK_POWER(agent, 0, 16);
        ATK_POWER(agent, 1, 16);
        ATK_POWER(agent, 2, 16);
    }
    FT_MOTION_RATE(agent, 2.0);
    wait(lua_state, 1.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_attacks3lw(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    FT_MOTION_RATE(agent, 0.8);
    frame(lua_state, 5.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("kneer"), 4.0, 361, 200, 0, 0, 4.0, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("legr"), 4.0, 361, 200, 0, 0, 4.0, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("hip"), 4.0, 361, 200, 0, 0, 2.0, 2.0, -2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(boma, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    FT_MOTION_RATE(agent, 2.0);
    wait(lua_state, 1.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATK_POWER(agent, 0, 6);
        ATK_POWER(agent, 1, 6);
        ATK_POWER(agent, 2, 6);
    }
    FT_MOTION_RATE(agent, 2.0);
    wait(lua_state, 1.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATK_POWER(agent, 0, 8);
        ATK_POWER(agent, 1, 8);
        ATK_POWER(agent, 2, 8);
    }
    FT_MOTION_RATE(agent, 2.0);
    wait(lua_state, 1.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATK_POWER(agent, 0, 10);
        ATK_POWER(agent, 1, 10);
        ATK_POWER(agent, 2, 10);
    }
    FT_MOTION_RATE(agent, 2.0);
    wait(lua_state, 1.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATK_POWER(agent, 0, 12);
        ATK_POWER(agent, 1, 12);
        ATK_POWER(agent, 2, 12);
    }
    FT_MOTION_RATE(agent, 2.0);
    wait(lua_state, 1.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATK_POWER(agent, 0, 14);
        ATK_POWER(agent, 1, 14);
        ATK_POWER(agent, 2, 14);
    }
    FT_MOTION_RATE(agent, 2.0);
    wait(lua_state, 1.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATK_POWER(agent, 0, 16);
        ATK_POWER(agent, 1, 16);
        ATK_POWER(agent, 2, 16);
    }
    FT_MOTION_RATE(agent, 2.0);
    wait(lua_state, 1.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_attackhi3(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    FT_MOTION_RATE(agent, 0.8);
    frame(lua_state, 5.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("head"), 4.0, 361, 200, 0, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("arml"), 4.0, 361, 200, 0, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("arml"), 4.0, 361, 200, 0, 0, 6.0, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 3, 0, Hash40::new("top"), 4.0, 361, 200, 0, 0, 4.0, 0.0, 0.0, 8.0, Some(0.0), Some(12.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    FT_MOTION_RATE(agent, 2.0);
    wait(lua_state, 1.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATK_POWER(agent, 0, 6);
        ATK_POWER(agent, 1, 6);
        ATK_POWER(agent, 2, 6);
        ATK_POWER(agent, 3, 6);
    }
    FT_MOTION_RATE(agent, 2.0);
    wait(lua_state, 1.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATK_POWER(agent, 0, 8);
        ATK_POWER(agent, 1, 8);
        ATK_POWER(agent, 2, 8);
        ATK_POWER(agent, 3, 8);
    }
    FT_MOTION_RATE(agent, 2.0);
    wait(lua_state, 1.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATK_POWER(agent, 0, 10);
        ATK_POWER(agent, 1, 10);
        ATK_POWER(agent, 2, 10);
        ATK_POWER(agent, 3, 10);
    }
    FT_MOTION_RATE(agent, 2.0);
    wait(lua_state, 1.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATK_POWER(agent, 0, 12);
        ATK_POWER(agent, 1, 12);
        ATK_POWER(agent, 2, 12);
        AttackModule::clear(boma, 3, false);
    }
    FT_MOTION_RATE(agent, 2.0);
    wait(lua_state, 1.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATK_POWER(agent, 0, 14);
        ATK_POWER(agent, 1, 14);
        ATK_POWER(agent, 2, 14);
    }
    FT_MOTION_RATE(agent, 2.0);
    wait(lua_state, 1.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATK_POWER(agent, 0, 16);
        ATK_POWER(agent, 1, 16);
        ATK_POWER(agent, 2, 16);
    }
    FT_MOTION_RATE(agent, 2.0);
    wait(lua_state, 1.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_attacklw3(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    FT_MOTION_RATE(agent, 0.8);
    frame(lua_state, 5.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("kneel"), 4.0, 80, 200, 0, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("toel"), 4.0, 80, 200, 0, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(boma, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    FT_MOTION_RATE(agent, 3.5);
    wait(lua_state, 0.6);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATK_POWER(agent, 0, 6);
        ATK_POWER(agent, 1, 6);
    }
    FT_MOTION_RATE(agent, 3.5);
    wait(lua_state, 0.6);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATK_POWER(agent, 0, 8);
        ATK_POWER(agent, 1, 8);
    }
    FT_MOTION_RATE(agent, 3.5);
    wait(lua_state, 0.6);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATK_POWER(agent, 0, 10);
        ATK_POWER(agent, 1, 10);
    }
    FT_MOTION_RATE(agent, 3.5);
    wait(lua_state, 0.6);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATK_POWER(agent, 0, 12);
        ATK_POWER(agent, 1, 12);
    }
    FT_MOTION_RATE(agent, 3.5);
    wait(lua_state, 0.6);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATK_POWER(agent, 0, 14);
        ATK_POWER(agent, 1, 14);
    }
    FT_MOTION_RATE(agent, 3.5);
    wait(lua_state, 0.6);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATK_POWER(agent, 0, 16);
        ATK_POWER(agent, 1, 16);
    }
    FT_MOTION_RATE(agent, 3.5);
    wait(lua_state, 0.6);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attacks3", game_attacks3, Priority::Low);
    agent.acmd("game_attacks3hi", game_attacks3hi, Priority::Low);
    agent.acmd("game_attacks3lw", game_attacks3lw, Priority::Low);
    agent.acmd("game_attackhi3", game_attackhi3, Priority::Low);
    agent.acmd("game_attacklw3", game_attacklw3, Priority::Low);
}