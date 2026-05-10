use super::*;
use globals::*;

//=================================================================
//== init_settings for edge_slipoffs module
//== Note: This is called from init_settings::init_settings_hook
//== Note: Forces GroundModule::correct to be called for
//         certain statuses
//== Note: JostleModule::set_team(boma, 0) is for walking through
//         other fighters
//=================================================================
pub unsafe fn init_settings_edges(boma: &mut BattleObjectModuleAccessor, situation: smash::app::SituationKind, arg3: i32, arg4: u32, ground_cliff_check_kind: smash::app::GroundCliffCheckKind, arg6: bool, arg7: i32, arg8: i32, arg9: i32, arg10: i32) -> u32 {
    let mut fix = arg4;
    let fighter_kind = boma.kind();
    let status_kind = StatusModule::status_kind(boma);

    if boma.is_fighter()
    && boma.is_situation(*SITUATION_KIND_GROUND) {

        if status_kind == *FIGHTER_STATUS_KIND_APPEAL {
            fix = *GROUND_CORRECT_KIND_GROUND as u32;
        }

        if [*FIGHTER_STATUS_KIND_WAIT,
            *FIGHTER_STATUS_KIND_DASH,
            *FIGHTER_STATUS_KIND_TURN,
            *FIGHTER_STATUS_KIND_TURN_DASH,
            *FIGHTER_STATUS_KIND_SQUAT,
            *FIGHTER_STATUS_KIND_SQUAT_WAIT,
            *FIGHTER_STATUS_KIND_SQUAT_F,
            *FIGHTER_STATUS_KIND_SQUAT_B,
            *FIGHTER_STATUS_KIND_SQUAT_RV,
            *FIGHTER_STATUS_KIND_LANDING,
            *FIGHTER_STATUS_KIND_LANDING_LIGHT,
            *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR,
            *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL,
            *FIGHTER_STATUS_KIND_LANDING_DAMAGE_LIGHT,
            *FIGHTER_STATUS_KIND_GUARD_DAMAGE,
            *FIGHTER_STATUS_KIND_ITEM_HEAVY_PICKUP,
            *FIGHTER_STATUS_KIND_DAMAGE,
            *FIGHTER_STATUS_KIND_AIR_LASSO_LANDING,
            *FIGHTER_STATUS_KIND_TREAD_DAMAGE,
            *FIGHTER_STATUS_KIND_TREAD_DAMAGE_RV,
            *FIGHTER_STATUS_KIND_LANDING_DAMAGE_LIGHT,
            *FIGHTER_STATUS_KIND_DAMAGE_SONG,
            *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_START,
            *FIGHTER_STATUS_KIND_DAMAGE_SLEEP,
            *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_END,
            *FIGHTER_STATUS_KIND_DOWN_DAMAGE,
            *FIGHTER_STATUS_KIND_SAVING_DAMAGE].contains(&status_kind) {
            fix = *GROUND_CORRECT_KIND_GROUND as u32;
        }

        if (boma.kind() == *FIGHTER_KIND_FALCO && boma.is_status(*FIGHTER_FALCO_STATUS_KIND_SPECIAL_S_FALL_LANDING)) {
            fix = *GROUND_CORRECT_KIND_GROUND as u32;
        }
    }

    return fix
}

//=================================================================
//== GroundModule::correct
//== Note: This is the "can edge slippoff" function in Smash
//=================================================================
#[skyline::hook(replace=GroundModule::correct)]
unsafe fn correct_hook(boma: &mut BattleObjectModuleAccessor, kind: GroundCorrectKind) -> u64 {
    if !boma.is_fighter() || !boma.is_situation(*SITUATION_KIND_GROUND) {
        return original!()(boma, kind);
    }

    let status_kind = StatusModule::status_kind(boma);
    if [
        *FIGHTER_STATUS_KIND_LANDING,
        *FIGHTER_STATUS_KIND_TURN_DASH,
        *FIGHTER_STATUS_KIND_DASH,
        *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL,
        *FIGHTER_STATUS_KIND_DAMAGE,
        *FIGHTER_STATUS_KIND_TREAD_DAMAGE,
        *FIGHTER_STATUS_KIND_TREAD_DAMAGE_RV,
        *FIGHTER_STATUS_KIND_LANDING_DAMAGE_LIGHT,
        *FIGHTER_STATUS_KIND_DAMAGE_SONG,
        *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_START,
        *FIGHTER_STATUS_KIND_DAMAGE_SLEEP,
        *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_END,
        *FIGHTER_STATUS_KIND_DOWN_DAMAGE,
        *FIGHTER_STATUS_KIND_SAVING_DAMAGE
    ].contains(&status_kind)
    || check_fighter_edge_slipoffs(boma).get_bool() {
        return original!()(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }

    original!()(boma, kind)
}

unsafe fn check_fighter_edge_slipoffs(boma: &mut BattleObjectModuleAccessor) -> L2CValue {
    let status_kind = StatusModule::status_kind(boma);
    let fighter_kind = boma.kind();

    return false.into();
}

pub fn install() {
    skyline::install_hooks!(
        correct_hook,
    );
}