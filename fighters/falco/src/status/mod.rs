use super::*;
use globals::*;

mod special_s;
mod special_lw;

pub fn install(agent: &mut Agent) {    
    special_s::install(agent);
    special_lw::install(agent);
}