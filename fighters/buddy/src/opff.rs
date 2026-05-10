use super::*;
use globals::*;
 
utils::import_noreturn!(common::opff::fighter_common_opff);

pub unsafe extern "C" fn buddy_frame_wrapper(fighter: &mut L2CFighterCommon) {
    common::opff::fighter_common_opff(fighter);
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, buddy_frame_wrapper);
}