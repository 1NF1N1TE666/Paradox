use super::*;

unsafe extern "C" fn game_attackairn(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    FT_MOTION_RATE(agent, 1.33333);
    frame(lua_state, 3.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("kneer"), 4.0, 361, 200, 0, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneel"), 4.0, 361, 200, 0, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        ATK_POWER(agent, 0, 6);
        ATK_POWER(agent, 1, 6);
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        ATK_POWER(agent, 0, 8);
        ATK_POWER(agent, 1, 8);
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        ATK_POWER(agent, 0, 10);
        ATK_POWER(agent, 1, 10);
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        ATK_POWER(agent, 0, 12);
        ATK_POWER(agent, 1, 12);
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        ATK_POWER(agent, 0, 14);
        ATK_POWER(agent, 1, 14);
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        ATK_POWER(agent, 0, 16);
        ATK_POWER(agent, 1, 16);
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_attackairf(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 16.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("arml"), 24.0, 60, 100, 0, 0, 4.0, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("shoulderl"), 24.0, 60, 100, 0, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    FT_MOTION_RATE(agent, 0.33333);
    wait(lua_state, 6.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_attackairb(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    FT_MOTION_RATE(agent, 0.66666);
    frame(lua_state, 6.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("kneer"), 4.0, 361, 200, 0, 0, 4.0, 6.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("legr"), 4.0, 361, 200, 0, 0, 6.0, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    FT_MOTION_RATE(agent, 2.0);
    wait(lua_state, 1.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATK_POWER(agent, 0, 6);
        ATK_POWER(agent, 1, 6);
    }
    FT_MOTION_RATE(agent, 2.0);
    wait(lua_state, 1.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATK_POWER(agent, 0, 8);
        ATK_POWER(agent, 1, 8);
    }
    FT_MOTION_RATE(agent, 2.0);
    wait(lua_state, 1.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATK_POWER(agent, 0, 10);
        ATK_POWER(agent, 1, 10);
    }
    FT_MOTION_RATE(agent, 2.0);
    wait(lua_state, 1.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATK_POWER(agent, 0, 12);
        ATK_POWER(agent, 1, 12);
    }
    FT_MOTION_RATE(agent, 2.0);
    wait(lua_state, 1.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATK_POWER(agent, 0, 14);
        ATK_POWER(agent, 1, 14);
    }
    FT_MOTION_RATE(agent, 2.0);
    wait(lua_state, 1.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATK_POWER(agent, 0, 16);
        ATK_POWER(agent, 1, 16);
    }
    FT_MOTION_RATE(agent, 2.0);
    wait(lua_state, 1.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_attackairhi(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    FT_MOTION_RATE(agent, 0.8);
    frame(lua_state, 5.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("legr"), 4.0, 361, 200, 0, 0, 4.0, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneer"), 4.0, 361, 200, 0, 0, 6.0, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
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

unsafe extern "C" fn game_attackairlw(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    FT_MOTION_RATE(agent, 0.25);
    frame(lua_state, 16.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 270, 200, 0, 0, 6.0, 0.0, -2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("top"), 4.0, 270, 200, 0, 0, 4.0, 0.0, 2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    FT_MOTION_RATE(agent, 7.0);
    wait(lua_state, 0.6);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATK_POWER(agent, 0, 6);
        ATK_POWER(agent, 1, 6);
    }
    FT_MOTION_RATE(agent, 7.0);
    wait(lua_state, 0.6);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATK_POWER(agent, 0, 8);
        ATK_POWER(agent, 1, 8);
    }
    FT_MOTION_RATE(agent, 7.0);
    wait(lua_state, 0.6);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATK_POWER(agent, 0, 10);
        ATK_POWER(agent, 1, 10);
    }
    FT_MOTION_RATE(agent, 7.0);
    wait(lua_state, 0.6);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATK_POWER(agent, 0, 12);
        ATK_POWER(agent, 1, 12);
    }
    FT_MOTION_RATE(agent, 7.0);
    wait(lua_state, 0.6);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATK_POWER(agent, 0, 14);
        ATK_POWER(agent, 1, 14);
    }
    FT_MOTION_RATE(agent, 7.0);
    wait(lua_state, 0.6);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATK_POWER(agent, 0, 16);
        ATK_POWER(agent, 1, 16);
    }
    FT_MOTION_RATE(agent, 7.0);
    wait(lua_state, 0.6);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attackairn", game_attackairn, Priority::Low);
    agent.acmd("game_attackairf", game_attackairf, Priority::Low);
    agent.acmd("game_attackairb", game_attackairb, Priority::Low);
    agent.acmd("game_attackairhi", game_attackairhi, Priority::Low);
    agent.acmd("game_attackairlw", game_attackairlw, Priority::Low);
}