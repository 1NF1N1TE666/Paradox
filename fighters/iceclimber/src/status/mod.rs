use super::*;
use globals::*;

mod special_n;
mod special_s;
mod special_hi;
mod special_lw;

pub fn install() {
    let agent = &mut Agent::new("popo");
    special_n::install(agent);
    agent.install();

    let agent = &mut Agent::new("nana");
    special_n::install(agent);
    agent.install();
}