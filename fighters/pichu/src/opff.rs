utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

pub unsafe extern "C" fn pichu_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    common::opff::fighter_common_opff(fighter);
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, pichu_frame_wrapper);
}
