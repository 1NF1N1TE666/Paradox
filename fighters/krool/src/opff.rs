utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

pub extern "C" fn krool_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, krool_frame_wrapper);
}