use super::*;

mod acmd;
mod status;

pub fn install() {
    let agent = &mut Agent::new("samus_cshot");
    acmd::install(agent);
    status::install(agent);
    agent.install();
}