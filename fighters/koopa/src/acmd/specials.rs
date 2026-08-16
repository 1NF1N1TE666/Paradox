use super::*;

unsafe extern "C" fn game_specialscatch(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        smash_script::macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 4.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        GrabModule::set_rebound(boma, true);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("top"), 8.0, 0.0, 8.0, 16.0, Some(0.0), Some(8.0), Some(12.0), *FIGHTER_STATUS_KIND_KOOPA_DIVED, *COLLISION_SITUATION_MASK_GA);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        GrabModule::set_rebound(boma, false);
    }
}

unsafe extern "C" fn game_specialssquat(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        smash_script::macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 4.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_BODY);
    }
}

unsafe extern "C" fn game_specialsjump(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        smash_script::macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 4.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_BODY);
    }
}

unsafe extern "C" fn game_specialsfall(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        smash_script::macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 4.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_BODY);
        ATTACK_IGNORE_THROW(agent, 0, 0, Hash40::new("top"), 24.0, 280, 120, 0, 0, 8.0, 0.0, 2.0, 6.0, Some(0.0), Some(2.0), Some(-4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
}

unsafe extern "C" fn game_specialslanding(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        smash_script::macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 24.0, 361, 60, 0, 80, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_BODY);
        smash_script::macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 4.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        CHECK_FINISH_CAMERA(agent, 3, 9);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.0);
        lua_bind::FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 0.0, y: -9.0, z: 0.0});
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_KOOPA_STATUS_SPECIAL_S_FLAG_HIT);
    }
}

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_XLU), 0);
    }
    FT_MOTION_RATE(agent, 0.66666);
    frame(lua_state, 6.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_KOOPA_STATUS_SPECIAL_HI_FLAG4);
        ATTACK(agent, 0, 0, Hash40::new("top"), 16.0, 80, 120, 0, 0, 8.0, 0.0, 8.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 16.0, 80, 120, 0, 0, 8.0, 0.0, 8.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        KineticModule::set_consider_ground_friction(boma, false, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        KineticModule::set_consider_ground_friction(boma, false, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_KOOPA_STATUS_SPECIAL_HI_FLAG4);
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_KOOPA_STATUS_SPECIAL_HI_FLAG4);
    }
    wait(lua_state, 12.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_KOOPA_STATUS_SPECIAL_HI_FLAG4);
    }
    wait(lua_state, 15.0);
    if is_excute(agent) {
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(boma);
        KineticModule::set_consider_ground_friction(boma, true, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        WorkModule::on_flag(boma, *FIGHTER_KOOPA_STATUS_SPECIAL_HI_FLAG1);
    }
    wait(lua_state, 12.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_KOOPA_STATUS_SPECIAL_HI_FLAG4);
    }
}

unsafe extern "C" fn game_specialairhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FT_MOTION_RATE(agent, 0.66666);
    frame(lua_state, 6.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_KOOPA_STATUS_SPECIAL_HI_FLAG4);
        ATTACK(agent, 0, 0, Hash40::new("top"), 16.0, 80, 120, 0, 0, 8.0, 0.0, 8.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 16.0, 80, 120, 0, 0, 8.0, 0.0, 8.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_KOOPA_STATUS_SPECIAL_HI_FLAG4);
    }
    wait(lua_state, 45.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_KOOPA_STATUS_SPECIAL_HI_FLAG4);
        WorkModule::on_flag(boma, *FIGHTER_KOOPA_STATUS_SPECIAL_HI_FLAG1);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FT_MOTION_RATE(agent, 0.36);
    frame(lua_state, 11.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 80, 100, 120, 0, 8.0, 0.0, 0.0, 24.0, Some(0.0), Some(8.0), Some(12.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        AttackModule::set_add_reaction_frame(boma, 0, 8.0, false);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_KOOPA_STATUS_SPECIAL_LW_FLAG1);
    }
    FT_MOTION_RATE(agent, 0.83);
    wait(lua_state, 23.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 24.0, 80, 120, 0, 0, 8.0, 0.0, 8.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
}

unsafe extern "C" fn game_specialairlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FT_MOTION_RATE(agent, 0.77);
    frame(lua_state, 31.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_KOOPA_STATUS_SPECIAL_LW_FLAG1);
        ATTACK(agent, 0, 0, Hash40::new("top"), 24.0, 80, 120, 0, 0, 8.0, 0.0, 8.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
}

unsafe extern "C" fn game_speciallwlanding(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 24.0, 80, 120, 0, 0, 8.0, 0.0, 2.0, -6.0, Some(0.0), Some(2.0), Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialscatch", game_specialscatch, Priority::Low);
    agent.acmd("game_specialsaircatch", game_specialscatch, Priority::Low);
    agent.acmd("game_specialssquat", game_specialssquat, Priority::Low);
    agent.acmd("game_specialairssquat", game_specialssquat, Priority::Low);
    agent.acmd("game_specialsjump", game_specialsjump, Priority::Low);
    agent.acmd("game_specialsfall", game_specialsfall, Priority::Low);
    agent.acmd("game_specialslanding", game_specialslanding, Priority::Low);
    agent.acmd("game_specialhi", game_specialhi, Priority::Low);
    agent.acmd("game_specialairhi", game_specialairhi, Priority::Low);
    agent.acmd("game_speciallw", game_speciallw, Priority::Low);
    agent.acmd("game_specialairlw", game_specialairlw, Priority::Low);
    agent.acmd("game_speciallwlanding", game_speciallwlanding, Priority::Low);
}