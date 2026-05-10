use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("lucario_auraball");
    acmd::install(agent);
    agent.install();
}