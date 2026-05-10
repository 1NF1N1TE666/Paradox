use super::*;
use globals::*;

unsafe extern "C" fn special_s_drag_jump_main(fighter: &mut L2CFighterCommon) -> L2CValue  {
    if fighter.is_button_on(Buttons::Attack)
    || fighter.is_button_on(Buttons::Special) {
        fighter.change_status(FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_DRAG_CLIFF.into(), false.into());
        return true.into()
    } else {
        smashline::original_status(Main, fighter, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_DRAG_JUMP)(fighter)
    }
}

unsafe extern "C" fn special_s_drag_jump_end(fighter: &mut L2CFighterCommon) -> L2CValue  {
    if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_DRAG_CLIFF {
        smashline::original_status(End, fighter, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_DRAG_JUMP)(fighter)
    } else {
        0.into()
    }
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_DRAG_JUMP, special_s_drag_jump_main);
    agent.status(End, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_DRAG_JUMP, special_s_drag_jump_end);
}