use super::*;

unsafe extern "C" fn throw_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_motion(Hash40::new("throw_lw")) {
        STOP_SE(fighter, Hash40::new("se_samus_special_n01"));
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_CSHOT, ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST));
    }

    smashline::original_status(End, fighter, *FIGHTER_STATUS_KIND_THROW)(fighter)
}

pub fn install(agent: &mut Agent) {
    agent.status(End, *FIGHTER_STATUS_KIND_THROW, throw_end);
}