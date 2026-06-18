use super::*;

unsafe extern "C" fn game_specialnstart(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    FT_MOTION_RATE(agent, 0.5);
    frame(lua_state, 4.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        if IS_EXIST_ARTICLE(agent, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER) {
            ArticleModule::change_motion(boma, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER, Hash40::new("open"), false, -1.0);
        }
    }
}

unsafe extern "C" fn game_specialairnstart(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    FT_MOTION_RATE(agent, 0.5);
    frame(lua_state, 4.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        if IS_EXIST_ARTICLE(agent, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER) {
            ArticleModule::change_motion(boma, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER, Hash40::new("open"), false, -1.0);
        }
    }
}

unsafe extern "C" fn game_specialnloop(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 2.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_FOX_BLASTER_STATUS_WORK_ID_FLAG_LOOP_ACCEPT);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER_BULLET, false, -1);
        if IS_EXIST_ARTICLE(agent, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER) {
            ArticleModule::change_motion(boma, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER, Hash40::new("loop"), false, -1.0);
        }
    }
    FT_MOTION_RATE(agent, 3.0);
    frame(lua_state, 11.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_FOX_BLASTER_STATUS_WORK_ID_FLAG_LOOP_ACCEPT);
    }
}


unsafe extern "C" fn game_specialairnloop(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 2.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_FOX_BLASTER_STATUS_WORK_ID_FLAG_LOOP_ACCEPT);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER_BULLET, false, -1);
        if IS_EXIST_ARTICLE(agent, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER) {
            ArticleModule::change_motion(boma, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER, Hash40::new("loop"), false, -1.0);
        }
    }
    FT_MOTION_RATE(agent, 3.0);
    frame(lua_state, 11.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_FOX_BLASTER_STATUS_WORK_ID_FLAG_LOOP_ACCEPT);
    }
}

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        JostleModule::set_status(boma, false);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("hip"), 12.0, 362, 100, 100, 0, 12.0, 2.0, -2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
    }
}

unsafe extern "C" fn game_speciallwstart(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("reflector"), 8.0, 362, 100, 100, 0, 8.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec_whip"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_ENERGY);
    }
}

unsafe extern "C" fn effect_speciallwstart(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("fox_ref_flash"), Hash40::new("reflector"), 1.0, 0, -0.5, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("fox_ref_start"), Hash40::new("top"), 0, 6, 0, 0, 0, 0, 0.75, false);
    }
}

unsafe extern "C" fn sound_speciallwstart(agent: &mut L2CAgentBase) {
	let lua_state = agent.lua_state_agent;
	let boma = agent.boma();
	if is_excute(agent) {
		PLAY_SE(agent, Hash40::new("se_item_item_get"));
	}
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialnstart", game_specialnstart, Priority::Low);
    agent.acmd("game_specialairnstart", game_specialairnstart, Priority::Low);
    agent.acmd("game_specialnloop", game_specialnloop, Priority::Low);
    agent.acmd("game_specialairnloop", game_specialairnloop, Priority::Low);
    agent.acmd("game_specialhihold", acmd_stub, Priority::Low);
    agent.acmd("game_specialhiholdair", acmd_stub, Priority::Low);
    agent.acmd("game_specialhi", game_specialhi, Priority::Low);
    agent.acmd("game_speciallwstart", game_speciallwstart, Priority::Low);
    agent.acmd("effect_speciallwstart", effect_speciallwstart, Priority::Low);
    agent.acmd("sound_speciallwstart", sound_speciallwstart, Priority::Low);
    agent.acmd("game_specialairlwstart", game_speciallwstart, Priority::Low);
    agent.acmd("effect_specialairlwstart", effect_speciallwstart, Priority::Low);
    agent.acmd("sound_specialairlwstart", sound_speciallwstart, Priority::Low);
}