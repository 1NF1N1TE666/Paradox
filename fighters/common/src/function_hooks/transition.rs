use super::*;
use globals::*;

#[skyline::hook(replace=WorkModule::enable_transition_term)]
unsafe fn enable_transition_term_hook(boma: &mut BattleObjectModuleAccessor, flag: i32) -> bool {
    if flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN {
        VarModule::on_flag(boma.object(), vars::common::status::IS_DASH_TO_RUN_FRAME);
    }
    original!()(boma, flag)
}

pub fn install() {
    skyline::install_hooks!(
        enable_transition_term_hook
    );
}