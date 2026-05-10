use super::*;
use globals::*;

#[skyline::hook(replace = StatusModule::situation_kind)]
unsafe fn situation_kind_replace(boma: &mut smash::app::BattleObjectModuleAccessor) -> i32 {
    if boma.kind() == *FIGHTER_KIND_LITTLEMAC
    && !boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_FALL_SPECIAL,
        *FIGHTER_STATUS_KIND_DAMAGE, 
        *FIGHTER_STATUS_KIND_DAMAGE_AIR, 
        *FIGHTER_STATUS_KIND_DAMAGE_FLY, 
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, 
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR, 
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D, 
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U, 
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
        *FIGHTER_STATUS_KIND_DAMAGE_SONG, 
        *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_START,
        *FIGHTER_STATUS_KIND_DAMAGE_SLEEP,
        *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_END,
        *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N2,
        *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_S_JUMP,
        *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_S_BLOW,
    ]) {
        return *SITUATION_KIND_GROUND;
    }

    if boma.kind() == *FIGHTER_KIND_PURIN {
        return *SITUATION_KIND_AIR;
    }

    original!()(boma)
}

pub fn install() {
    skyline::install_hooks!(
        situation_kind_replace,
    );
}
