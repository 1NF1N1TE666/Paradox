use super::*;
use utils::consts::vars::brave::instance;
utils::import_noreturn!(common::opff::fighter_common_opff);

pub extern "C" fn brave_frame_wrapper(fighter: &mut L2CFighterCommon) {
    unsafe { 
        common::opff::fighter_common_opff(fighter);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, brave_frame_wrapper);
}