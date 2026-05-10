use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("fox_illusion");
    acmd::install(agent);
    agent.install();
}