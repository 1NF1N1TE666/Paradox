use super::*;
use globals::*;

unsafe extern "C" fn burst_attack_init(weapon: &mut L2CFighterCommon) -> L2CValue {
    smashline::original_status(Main, weapon, *WEAPON_SAMUS_BOMB_STATUS_KIND_BURST_ATTACK)(weapon);
    weapon.set_int(0, *WEAPON_SAMUS_BOMB_INSTANCE_WORK_ID_INT_BOMBJUMP);
    let fighter_pos_x = PostureModule::pos_x(weapon.get_owner_boma());
    let fighter_pos_y = PostureModule::pos_y(weapon.get_owner_boma());
    let weapon_pos_x = PostureModule::pos_x(weapon.module_accessor);
    let weapon_pos_y = PostureModule::pos_y(weapon.module_accessor);
    let distance = sv_math::vec2_distance(fighter_pos_x, fighter_pos_y, weapon_pos_x, weapon_pos_y);
    let param_bomb_jump_hit_size = weapon.get_owner_boma().get_param_float("param_special_lw", "sp_lw_bj_hit_size");
    if distance <= param_bomb_jump_hit_size
    && weapon.get_owner_boma().is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW,
        *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW
    ]) && VarModule::is_flag(weapon.get_owner_boma().object(), vars::samus::status::SPECIAL_LW_BOMB_JUMP_ON) {
        VarModule::on_flag(weapon.get_owner_boma().object(), vars::samus::status::SPECIAL_LW_BOMB_JUMP_HOP);
    }
    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(Init, *WEAPON_SAMUS_BOMB_STATUS_KIND_BURST_ATTACK, burst_attack_init);
}