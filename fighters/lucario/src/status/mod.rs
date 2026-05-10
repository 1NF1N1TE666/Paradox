use super::*;
use globals::*;

mod special_s;

extern "C" {
    #[link_name = "\u{1}_ZN3app14LinkEventThrow13new_l2c_tableEv"]
    fn new_event_table() -> L2CValue;
    #[link_name = "\u{1}_ZN3app8lua_bind31LinkEvent__store_l2c_table_implEPKNS_9LinkEventE"]
    fn store_event_table(event: *const app::LinkEvent) -> L2CValue;
}

unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_DEAD,
        *FIGHTER_STATUS_KIND_REBIRTH,
        *FIGHTER_STATUS_KIND_WIN,
        *FIGHTER_STATUS_KIND_LOSE,
        *FIGHTER_STATUS_KIND_ENTRY
    ]) {
        VarModule::set_float(fighter.battle_object, vars::lucario::instance::PREV_SPEED_X, 0.0);
        VarModule::set_float(fighter.battle_object, vars::lucario::instance::PREV_SPEED_Y, 0.0);
        VarModule::set_float(fighter.battle_object, vars::lucario::instance::PREV_LR, fighter.lr());
        VarModule::set_float(fighter.battle_object, vars::lucario::instance::PREV_DAMAGE_STORAGE, 0.0);
    }

    true.into()
}

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    fighter.global_table[globals::STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));
    VarModule::set_float(fighter.battle_object, vars::lucario::instance::PREV_SPEED_X, 0.0);
    VarModule::set_float(fighter.battle_object, vars::lucario::instance::PREV_SPEED_Y, 0.0);
    VarModule::set_float(fighter.battle_object, vars::lucario::instance::PREV_LR, fighter.lr());
    VarModule::set_float(fighter.battle_object, vars::lucario::instance::PREV_DAMAGE_STORAGE, 0.0);
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);
    special_s::install(agent);
}