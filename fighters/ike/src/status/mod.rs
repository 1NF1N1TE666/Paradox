use super::*;
use globals::*;
use special_n::*;

pub mod special_n;
pub mod special_s;

unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_DEAD,
        *FIGHTER_STATUS_KIND_REBIRTH,
        *FIGHTER_STATUS_KIND_WIN,
        *FIGHTER_STATUS_KIND_LOSE,
        *FIGHTER_STATUS_KIND_ENTRY
    ]) {
        VarModule::off_flag(fighter.battle_object, vars::ike::instance::STORED_AETHER);
        VarModule::set_float(fighter.battle_object, vars::ike::instance::SPECIAL_N_CHARGE_COUNT, 0.0);
        stored_aether_effect_off(fighter);
    }

    true.into()
}

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    fighter.global_table[globals::STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));
    VarModule::off_flag(fighter.battle_object, vars::ike::instance::STORED_AETHER);
    VarModule::set_float(fighter.battle_object, vars::ike::instance::SPECIAL_N_CHARGE_COUNT, 0.0);
    stored_aether_effect_off(fighter);
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);
    special_n::install(agent);
    special_s::install(agent);
}