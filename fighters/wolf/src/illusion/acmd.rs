use super::*;

pub fn install(agent: &mut Agent) {
    agent.acmd("game_moveground", acmd_stub, Priority::Low);
    agent.acmd("game_moveair", acmd_stub, Priority::Low);
}
