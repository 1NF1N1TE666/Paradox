use super::*;

unsafe extern "C" fn game_appealhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FT_MOTION_RATE(agent, 1.41);
    frame(lua_state, 17.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        if DamageModule::damage(boma, 0) < 150.0 {
            VarModule::set_float(boma.object(), vars::lucario::instance::PREV_DAMAGE_STORAGE, DamageModule::damage(boma, 0));
            DamageModule::add_damage(boma, 150.0 - DamageModule::damage(boma, 0), 0);
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_appealhil", game_appealhi, Priority::Low);
    agent.acmd("game_appealhir", game_appealhi, Priority::Low);
}