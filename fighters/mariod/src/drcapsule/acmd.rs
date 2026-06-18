use super::*;
use consts::*;

unsafe extern "C" fn game_regular(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 361, 200, 0, 0, 2.0, 0.0, 2.0, 0.0, Some(0.0), Some(-2.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MARIOD_CAPSULE, *ATTACK_REGION_NONE);
        AttackModule::enable_safe_pos(boma);
        VarModule::set_int(boma.object(), vars::mariod_drcapsule::instance::DAMAGE, 2);
    } 
    for _ in 0..i32::MAX {
        wait(lua_state, 60.0);
        if is_excute(agent) {
            let damage = VarModule::get_int(boma.object(), vars::mariod_drcapsule::instance::DAMAGE);
            ATK_POWER(agent, 0, damage + damage);
            VarModule::add_int(boma.object(), vars::mariod_drcapsule::instance::DAMAGE, damage)
        } 
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_regular", game_regular, Priority::Low);
}