use interpolation::Lerp;
use super::*;
use globals::*;

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_pre_EscapeAir,
            status_EscapeAir,
            status_end_EscapeAir
        );
    }
}

#[skyline::hook(replace = L2CFighterCommon_status_pre_EscapeAir)]
pub unsafe fn status_pre_EscapeAir(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[PREV_STATUS_KIND] != FIGHTER_STATUS_KIND_DAMAGE_FALL 
    && (VarModule::is_flag(fighter.battle_object, vars::common::instance::PERFECT_WAVEDASH) || fighter.handle_waveland(false)) {
        VarModule::on_flag(fighter.battle_object, vars::common::status::SHOULD_WAVELAND);
        GroundModule::attach_ground(fighter.module_accessor, true);
        let status = if fighter.global_table[FIGHTER_KIND] == FIGHTER_KIND_KOOPAJR
        && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KOOPAJR_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_INTERRUPT) {
            FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_LANDING
        } else {
            FIGHTER_STATUS_KIND_LANDING
        };
        fighter.change_status(status.into(), false.into());
        return 0.into();
    }
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_AIR as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        false,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_DISABLE,
        false,
        false,
        false,
        0,
        0,
        0,
        0
    );
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_EscapeAir)]
unsafe fn status_EscapeAir(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_escape_air_common();
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) {
        let motion = if fighter.global_table[FIGHTER_KIND] == FIGHTER_KIND_KOOPAJR && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KOOPAJR_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_INTERRUPT) {"special_hi_jr_escape"} else {"escape_air_slide"};
        MotionModule::change_motion(fighter.module_accessor, Hash40::new(motion), 0.0, 1.0, false, 0.0, false, false);
        VarModule::on_flag(fighter.battle_object, vars::common::instance::ENABLE_AIR_ESCAPE_MAGNET);
    } else {
        let motion = if fighter.global_table[FIGHTER_KIND] == FIGHTER_KIND_KOOPAJR && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KOOPAJR_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_INTERRUPT) {"special_hi_jr_escape"} else {"escape_air"};
        MotionModule::change_motion(fighter.module_accessor, Hash40::new(motion), 0.0, 1.0, false, 0.0, false, false);
        VarModule::off_flag(fighter.battle_object, vars::common::instance::ENABLE_AIR_ESCAPE_MAGNET);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(status_EscapeAir_Main as *const () as _))
}

unsafe extern "C" fn status_EscapeAir_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.battle_object, vars::common::status::SHOULD_WAVELAND) {
        return 1.into();
    }
    if !fighter.sub_escape_air_common_main().get_bool() {
        fighter.sub_escape_check_rumble();
    }
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_end_EscapeAir)]
pub unsafe fn status_end_EscapeAir(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.battle_object, vars::common::status::SHOULD_WAVELAND);
    VarModule::off_flag(fighter.battle_object, vars::common::instance::PERFECT_WAVEDASH);
    VarModule::on_flag(fighter.battle_object, vars::common::instance::ENABLE_AIR_ESCAPE_MAGNET);
    
    call_original!(fighter)
}