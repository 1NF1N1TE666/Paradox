use super::*;

unsafe extern "C" fn game_ready(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    let angle = if boma.get_owner_boma().kind() == *FIGHTER_KIND_SAMUS && boma.get_owner_boma().is_status(*FIGHTER_STATUS_KIND_THROW) {90.0} else {VarModule::get_float(boma.get_owner_boma().object(), vars::samus::instance::AIM_ANGLE)};
    if is_excute(agent) {
        if boma.get_owner_boma().is_status(*FIGHTER_STATUS_KIND_THROW) {
            PostureModule::add_pos(boma, &Vector3f::new(0.0, 12.0, 0.0));
        }
        PostureModule::set_rot(boma, &Vector3f{x: -angle, y: 0.0, z: 0.0}, 0);
    }
}

unsafe extern "C" fn game_straight(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 16.0, 363, 100, 0, 0, 2.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_OBJECT);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_ready", game_ready, Priority::Low);
    agent.acmd("game_straight", game_straight, Priority::Low);
}