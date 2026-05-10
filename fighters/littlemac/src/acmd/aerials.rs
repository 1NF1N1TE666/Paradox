use super::*;

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attackairn", acmd_stub, Priority::Low);
    agent.acmd("game_attackairf", acmd_stub, Priority::Low);
    agent.acmd("game_attackairb", acmd_stub, Priority::Low);
    agent.acmd("game_attackairhi", acmd_stub, Priority::Low);
    agent.acmd("game_attackairlw", acmd_stub, Priority::Low);
}