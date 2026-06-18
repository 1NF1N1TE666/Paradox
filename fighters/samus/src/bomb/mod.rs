use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("samus_bomb");
    acmd::install(agent);
    agent.install();
}