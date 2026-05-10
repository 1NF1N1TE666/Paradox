utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

// symbol-based call for villager/isabelle's common pocket opff
extern "Rust" {
    fn ac_common(fighter: &mut smash::lua2cpp::L2CFighterCommon);
}

pub extern "C" fn murabito_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
        ac_common(fighter);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, murabito_frame_wrapper);
}