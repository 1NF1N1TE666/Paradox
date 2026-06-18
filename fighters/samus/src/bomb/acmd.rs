use super::*;

unsafe extern "C" fn game_fall(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 22.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), if VarModule::is_flag(boma.get_owner_boma().object(), vars::samus::instance::ICE_MODE) {8.0} else {12.0}, 363, 100, if VarModule::is_flag(boma.get_owner_boma().object(), vars::samus::instance::ICE_MODE) {120} else {80}, 0, 2.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 6, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(if VarModule::is_flag(boma.get_owner_boma().object(), vars::samus::instance::ICE_MODE) {"collision_attr_ice"} else {"collision_attr_fire"}), *ATTACK_SOUND_LEVEL_S, if VarModule::is_flag(boma.get_owner_boma().object(), vars::samus::instance::ICE_MODE) {*COLLISION_SOUND_ATTR_FREEZE} else {*COLLISION_SOUND_ATTR_BOMB}, *ATTACK_REGION_OBJECT);
    }
}

unsafe extern "C" fn effect_fall(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    for i in 1..i32::MAX {
        if is_excute(agent) {
            if VarModule::is_flag(boma.get_owner_boma().object(), vars::samus::instance::ICE_MODE) {
                EFFECT_FOLLOW(agent, Hash40::new("sys_starrod_splash"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
                LAST_EFFECT_SET_COLOR(agent,0.375, 1.0,1.0);
            }
        }
        wait(lua_state, 15.0);
    }
}

unsafe extern "C" fn game_burstattack(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), if VarModule::is_flag(boma.get_owner_boma().object(), vars::samus::instance::ICE_MODE) {8.0} else {12.0}, 363, 100, if VarModule::is_flag(boma.get_owner_boma().object(), vars::samus::instance::ICE_MODE) {120} else {80}, 0, 8.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 6, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(if VarModule::is_flag(boma.get_owner_boma().object(), vars::samus::instance::ICE_MODE) {"collision_attr_ice"} else {"collision_attr_fire"}), *ATTACK_SOUND_LEVEL_S, if VarModule::is_flag(boma.get_owner_boma().object(), vars::samus::instance::ICE_MODE) {*COLLISION_SOUND_ATTR_FREEZE} else {*COLLISION_SOUND_ATTR_BOMB}, *ATTACK_REGION_OBJECT);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_explosion"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    wait(lua_state, 7.0);
    if is_excute(agent) {
        AreaModule::erase_wind(boma, 0);
    }
    wait(lua_state, 5.0);
    if is_excute(agent) {
        AttackModule::set_size(boma, 0, 6.0);
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        AttackModule::set_size(boma, 0, 4.0);
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x199c462b5d));
    }
}

unsafe extern "C" fn effect_burstattack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if VarModule::is_flag(boma.get_owner_boma().object(), vars::samus::instance::ICE_MODE) {
            EFFECT(agent, Hash40::new("sys_hit_ice"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        } else {
            agent.clear_lua_stack();
            lua_args!(agent, Hash40::new("sys_bomb_a"), Hash40::new("sys_bomb_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
            sv_animcmd::EFFECT_BRANCH_SITUATION(lua_state);
            agent.clear_lua_stack();
        }
    }
}

unsafe extern "C" fn sound_burstattack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if VarModule::is_flag(boma.get_owner_boma().object(), vars::samus::instance::ICE_MODE) { 
            PLAY_SE(agent, Hash40::new("se_item_ice_crash"));
        } else {
            PLAY_SE_REMAIN(agent, Hash40::new("se_common_bomb_s"));
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_fall", game_fall, Priority::Low);
    agent.acmd("effect_fall", effect_fall, Priority::Low);
    agent.acmd("game_burstattack", game_burstattack, Priority::Low);
    agent.acmd("effect_burstattack", effect_burstattack, Priority::Low);
    agent.acmd("sound_burstattack", sound_burstattack, Priority::Low);
}