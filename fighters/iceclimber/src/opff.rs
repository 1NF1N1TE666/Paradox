utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

static mut EFFECT_ON: bool = false;
static mut NANA_POS_X: f32 = 0.0;
static mut NANA_POS_Y: f32 = 0.0;

pub static mut NANA_BOMA: [u64; 8] = [0; 8];

unsafe fn nana_boma(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize) {
    if boma.kind() == *FIGHTER_KIND_NANA {
        let mut nana_boma_func = boma;
        NANA_BOMA[id] = (&mut *nana_boma_func as *mut BattleObjectModuleAccessor) as u64;
    }
}

unsafe fn spotdodge_desync(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    if boma.kind() == *FIGHTER_KIND_NANA {
        if ![*FIGHTER_STATUS_KIND_ESCAPE, *FIGHTER_STATUS_KIND_ESCAPE_F, *FIGHTER_STATUS_KIND_ESCAPE_B, *FIGHTER_STATUS_KIND_CLIFF_ESCAPE, *FIGHTER_STATUS_KIND_CLIFF_CLIMB].contains(&status_kind){
            InputModule::disable_persist(boma.object());
        } else if [*FIGHTER_STATUS_KIND_ESCAPE, *FIGHTER_STATUS_KIND_ESCAPE_F, *FIGHTER_STATUS_KIND_ESCAPE_B, *FIGHTER_STATUS_KIND_CLIFF_ESCAPE, *FIGHTER_STATUS_KIND_CLIFF_CLIMB].contains(&StatusModule::status_kind_next(boma)) {
            InputModule::enable_persist(boma.object());
        } else {}
    }
}

unsafe fn voluntary_sopo(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, frame: f32) {
    if fighter.kind() != *FIGHTER_KIND_POPO {
        return;
    }

    let nana = NANA_BOMA[id] as *mut BattleObjectModuleAccessor;
    if (*nana).is_status(*FIGHTER_STATUS_KIND_REBIRTH) {
        if fighter.is_status(*FIGHTER_STATUS_KIND_REBIRTH) {
            VarModule::off_flag(boma.object(), vars::iceclimbers::instance::IS_VOLUNTARY_SOPO_A)
        } else if VarModule::is_flag(boma.object(), vars::iceclimbers::instance::IS_VOLUNTARY_SOPO_A | vars::iceclimbers::instance::IS_VOLUNTARY_SOPO_B) {
            StatusModule::change_status_request(nana, *FIGHTER_STATUS_KIND_STANDBY, false);
        } else {}
    }

    if fighter.is_status(*FIGHTER_STATUS_KIND_APPEAL) 
    && fighter.is_button_on(Buttons::Guard) {
        VarModule::on_flag(boma.object(), vars::iceclimbers::instance::IS_VOLUNTARY_SOPO_A);
        StatusModule::change_status_request(nana, *FIGHTER_STATUS_KIND_DEAD, false);
    } else if fighter.is_prev_status(*FIGHTER_STATUS_KIND_ENTRY) 
    && fighter.is_button_on(Buttons::Guard) {
        VarModule::on_flag(boma.object(), vars::iceclimbers::instance::IS_VOLUNTARY_SOPO_B);
        StatusModule::change_status_request(nana, *FIGHTER_STATUS_KIND_DEAD, false);
    } else {}
}

unsafe fn nana_death_effect(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, frame: f32) {
    if boma.kind() == *FIGHTER_KIND_POPO {
        if status_kind == *FIGHTER_STATUS_KIND_STANDBY {
            EFFECT_ON = true;
            NANA_POS_X = PostureModule::pos_x(NANA_BOMA[id] as *mut BattleObjectModuleAccessor);
            NANA_POS_Y = PostureModule::pos_y(NANA_BOMA[id] as *mut BattleObjectModuleAccessor);
        }

        if status_kind == *FIGHTER_STATUS_KIND_REBIRTH && fighter.status_frame() <= 1 && EFFECT_ON {
            let pos =  Vector3f {x: NANA_POS_X, y: NANA_POS_Y, z: 0.0};
            let rot =  Vector3f {x: 0.0, y: 0.0, z: 0.0};
            EffectModule::req(boma, Hash40::new("sys_recovery"), &pos, &rot, 1.0, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32, 0, true, 0);
            EFFECT_ON = false;
        }
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_HI,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_POPO_STATUS_KIND_SPECIAL_HI_FAIL,
        *FIGHTER_POPO_STATUS_KIND_SPECIAL_HI_JUMP,
        *FIGHTER_POPO_STATUS_KIND_SPECIAL_HI_JUMP_PARTNER
        ]) 
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
    }
}

pub unsafe fn ice_climbers_moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    nana_boma(fighter, boma, id);
    spotdodge_desync(boma, status_kind);
    voluntary_sopo(fighter, boma, id, status_kind, frame);
    nana_death_effect(fighter, boma, id, status_kind, frame);
    fastfall_specials(fighter);
}

pub extern "C" fn popo_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
        // ice_climbers_common(fighter);
    }
}

pub extern "C" fn nana_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
        // ice_climbers_common(fighter);
    }
}

#[no_mangle]
pub unsafe extern "Rust" fn ice_climbers_common(fighter: &mut L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        ice_climbers_moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

pub fn install_popo(agent: &mut Agent) {
    agent.on_line(Main, popo_frame_wrapper);
}

pub fn install_nana(agent: &mut Agent) {
    agent.on_line(Main, nana_frame_wrapper);
}