use super::*;

unsafe extern "C" fn game_fly(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 160, 100, 100, 0, 1.0, 0.0, 0.0, 1.0, Some(0.0), Some(0.0), Some(10.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_blaster_throw_down"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FOX_BLASTER, *ATTACK_REGION_ENERGY);
    }
}

unsafe extern "C" fn game_flythrowb(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 120, 60, 0, 80, 10.0, 0.0, 0.0, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_blaster_throw_down"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FOX_BLASTER, *ATTACK_REGION_ENERGY);
    }
}

unsafe extern "C" fn game_flythrowhi(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 270, 60, 0, 80, 10.0, 0.0, 0.0, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_blaster_throw_up"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FOX_BLASTER, *ATTACK_REGION_ENERGY);
    }
}

unsafe extern "C" fn game_flythrowlw(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 0, 0, 0, 0, 10.0, 0.0, 0.0, 8.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_blaster_throw_down"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FOX_BLASTER, *ATTACK_REGION_ENERGY);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_fly", game_fly, Priority::Low);
    agent.acmd("game_flythrowb", game_flythrowb, Priority::Low);
    agent.acmd("game_flythrowhi", game_flythrowhi, Priority::Low);
    agent.acmd("game_flythrowlw", game_flythrowlw, Priority::Low);
}