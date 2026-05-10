use super::*;

mod glide;
mod special_n;
mod special_hi;

pub fn install(agent: &mut Agent) {
    glide::install(agent);
    special_n::install(agent);
    special_hi::install(agent);
}