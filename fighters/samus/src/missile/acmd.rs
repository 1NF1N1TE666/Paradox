use super::*;
use globals::*;

unsafe extern "C" fn game_homing(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    let angle = if boma.get_owner_boma().kind() == *FIGHTER_KIND_SAMUS && boma.get_owner_boma().is_status(*FIGHTER_STATUS_KIND_THROW) {90.0} else {VarModule::get_float(boma.get_owner_boma().object(), vars::samus::instance::AIM_ANGLE)};
    if is_excute(agent) {
        if boma.get_owner_boma().is_status(*FIGHTER_STATUS_KIND_THROW) {
            PostureModule::add_pos(boma, &Vector3f::new(0.0, 12.0, 0.0));
        }
        PostureModule::set_rot(boma, &Vector3f{x: -angle, y: 0.0, z: 0.0}, 0);
        boma.set_float(-angle, *WEAPON_SAMUS_MISSILE_INSTANCE_WORK_ID_FLOAT_ROT);
        ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 363, 100, 0, 0, 2.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_OBJECT);
        AttackModule::enable_safe_pos(boma);
    }
}

unsafe extern "C" fn effect_homing(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("samus_missile_homing"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.75, true);
        LAST_EFFECT_SET_COLOR(agent,0.375, 1.0,1.0);
    }
    for i in 1..i32::MAX {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("sys_starrod_splash"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
            LAST_EFFECT_SET_COLOR(agent,0.375, 1.0,1.0);
        }
        wait(lua_state, 15.0);
    }
}

unsafe extern "C" fn effect_hburst(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_hit_ice"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}
unsafe extern "C" fn sound_hburst(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_item_ice_crash"));
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_homing", game_homing, Priority::Low);
    agent.acmd("effect_homing", effect_homing, Priority::Low);
    agent.acmd("effect_hburst", effect_hburst, Priority::Low);
    agent.acmd("sound_hburst", sound_hburst, Priority::Low);
}