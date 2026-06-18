use super::*;
use globals::*;

mod attacks;
mod throw;
mod special_n;
mod special_s;
mod special_hi;
mod special_lw;

pub mod speedbooster_shinespark;

unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_DEAD,
        *FIGHTER_STATUS_KIND_REBIRTH,
        *FIGHTER_STATUS_KIND_WIN,
        *FIGHTER_STATUS_KIND_LOSE,
        *FIGHTER_STATUS_KIND_ENTRY
    ]) {
        VarModule::off_flag(fighter.battle_object, vars::samus::instance::ICE_MODE);
        VarModule::off_flag(fighter.battle_object, vars::samus::instance::SPEEDBOOSTER_ON);
        VarModule::off_flag(fighter.battle_object, vars::samus::instance::SHINESPARK_ON);
    }

    true.into()
}

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    fighter.global_table[globals::STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));
    VarModule::off_flag(fighter.battle_object, vars::samus::instance::ICE_MODE);
    VarModule::off_flag(fighter.battle_object, vars::samus::instance::SPEEDBOOSTER_ON);
    VarModule::off_flag(fighter.battle_object, vars::samus::instance::SHINESPARK_ON);
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);
    attacks::install(agent);
    throw::install(agent);
    special_n::install(agent);
    special_s::install(agent);
    special_hi::install(agent);
    special_lw::install(agent);
    speedbooster_shinespark::install(agent);
}