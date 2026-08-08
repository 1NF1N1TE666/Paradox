use super::*;
use interpolation::Lerp;

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    let angle = VarModule::get_float(boma.object(), vars::samus::status::SPECIAL_HI_ANGLE);
    FT_MOTION_RATE(agent, 0.8);
    frame(lua_state, 5.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_SAMUS_GENERATE_ARTICLE_GBEAM, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::generate_article(boma, *FIGHTER_SAMUS_GENERATE_ARTICLE_GBEAM, false, -1);
        ArticleModule::set_visibility_whole(boma, *FIGHTER_SAMUS_GENERATE_ARTICLE_GBEAM, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        GroundModule::select_cliff_hangdata(boma, *FIGHTER_SAMUS_CLIFF_HANG_DATA_AIR_LASSO as u32);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_AIR_LASSO_FLAG_CHECK);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_AIR_LASSO_FLAG_CHECK);
        GroundModule::select_cliff_hangdata(boma, *FIGHTER_CLIFF_HANG_DATA_DEFAULT as u32);
        ArticleModule::change_status(boma, *FIGHTER_SAMUS_GENERATE_ARTICLE_GBEAM, *WEAPON_SAMUS_GBEAM_STATUS_KIND_SHOOT, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        VarModule::on_flag(boma.object(), vars::samus::status::SPECIAL_HI_LOCK_ANGLE);
        VarModule::on_flag(boma.object(), vars::samus::status::SPECIAL_HI_FIX_GBEAM_POS);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        ArticleModule::set_visibility_whole(boma, *FIGHTER_SAMUS_GENERATE_ARTICLE_GBEAM, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        let article = ArticleModule::get_article(boma, *FIGHTER_SAMUS_GENERATE_ARTICLE_GBEAM);
        let object_id = lua_bind::Article::get_battle_object_id(article) as u32;
        let article_boma = sv_battle_object::module_accessor(object_id);
        let pos = &mut Vector3f{x:0.0, y:0.0, z:0.0};
        ModelModule::joint_global_position(boma, Hash40::new("armr"), pos, false);
        if !(&mut *article_boma).is_status(*WEAPON_SAMUS_GBEAM_STATUS_KIND_SHOOT) && GroundModule::get_distance_to_floor(boma, pos, 0.0, false) < 8.0 && angle > 60.0 {
            ArticleModule::change_status(boma, *FIGHTER_SAMUS_GENERATE_ARTICLE_GBEAM, *WEAPON_SAMUS_GBEAM_STATUS_KIND_SHOOT, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("throw"), 28.0, 361, 120, 0, 80, 2.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BAT, *ATTACK_REGION_ENERGY);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 32.0);
    if is_excute(agent) {
        ArticleModule::change_status(boma, *FIGHTER_SAMUS_GENERATE_ARTICLE_GBEAM, *WEAPON_SAMUS_GBEAM_STATUS_KIND_REWIND, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(lua_state, 45.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_SAMUS_GENERATE_ARTICLE_GBEAM, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn effect_specialhi(agent : &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 8.0);
    if is_excute(agent) {
        if boma.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        }
    }
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("samus_gbeam_flash"), Hash40::new("armr"), 7.2, 0, 0, 0, 0, 0, 1.3, true);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("samus_gbeam_shot"), Hash40::new("armr"), 7, 0, -0.5, 0, 0, 0, 0.75, true);
    }
    frame(lua_state, 42.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("samus_gbeam_vanish"), Hash40::new("armr"), 7, 0, 0, 0, 0, 0, 0.7, true);
    }
}

unsafe extern "C" fn sound_specialhi(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 9.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_samus_catch"));
    }
}

unsafe extern "C" fn expression_specialhi(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        VisibilityModule::set_int64(boma, hash40("body") as i64, hash40("body_hide_gun") as i64);
        ArticleModule::remove_exist(boma, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::generate_article(boma, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, true, -1);
        ArticleModule::change_motion(boma, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, Hash40::new("air_catch"), false, -1.0);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_elecattacks"), 9, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_elecattacks"), 14, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn game_speciallw(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 8.0);
    if is_excute(agent) {
        VarModule::on_flag(boma.object(), vars::samus::status::SPECIAL_LW_BOMB_JUMP_ON)
    }
}

unsafe extern "C" fn sound_speciallw(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 7.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_samus_escape_ex"));
    }
}

unsafe extern "C" fn expression_speciallw(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        slope!(agent, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        slope!(agent, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_TOP);
        VisibilityModule::set_int64(boma, hash40("body") as i64, hash40("body_sphere") as i64);
        ItemModule::set_have_item_visibility(boma, false, 0);
        ItemModule::set_attach_item_visibility(boma, false, 0);
    }
}

unsafe extern "C" fn game_speciallwend(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 4.0);
    if is_excute(agent) {
        VarModule::off_flag(boma.object(), vars::samus::status::SPECIAL_LW_BOMB_JUMP_ON)
    }
}

unsafe extern "C" fn sound_speciallwend(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 3.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_samus_escape_ex"));
    }
}

unsafe extern "C" fn expression_speciallwend(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 4.0);
    if is_excute(agent) {
        slope!(agent, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
        VisibilityModule::set_int64(boma, hash40("body") as i64, hash40("body_normal") as i64);
        ItemModule::set_have_item_visibility(boma, true, 0);
        ItemModule::set_attach_item_visibility(boma, true, 0);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialhi", game_specialhi, Priority::Low);
    agent.acmd("effect_specialhi", effect_specialhi, Priority::Low);
    agent.acmd("sound_specialhi", sound_specialhi, Priority::Low);
    agent.acmd("expression_specialhi", expression_specialhi, Priority::Low);
    agent.acmd("game_specialairhi", game_specialhi, Priority::Low);
    agent.acmd("effect_specialairhi", effect_specialhi, Priority::Low);
    agent.acmd("sound_specialairhi", sound_specialhi, Priority::Low);
    agent.acmd("expression_specialairhi", expression_specialhi, Priority::Low);
    agent.acmd("game_speciallw", game_speciallw, Priority::Low);
    agent.acmd("effect_speciallw", acmd_stub, Priority::Low);
    agent.acmd("sound_speciallw", sound_speciallw, Priority::Low);
    agent.acmd("expression_speciallw", expression_speciallw, Priority::Low);
    agent.acmd("game_specialairlw", game_speciallw, Priority::Low);
    agent.acmd("effect_specialairlw", acmd_stub, Priority::Low);
    agent.acmd("sound_specialairlw", sound_speciallw, Priority::Low);
    agent.acmd("expression_specialairlw", expression_speciallw, Priority::Low);
    agent.acmd("game_speciallwend", game_speciallwend, Priority::Low);
    agent.acmd("effect_speciallwend", acmd_stub, Priority::Low);
    agent.acmd("sound_speciallwend", sound_speciallwend, Priority::Low);
    agent.acmd("expression_speciallwend", expression_speciallwend, Priority::Low);
    agent.acmd("game_specialairlwend", game_speciallwend, Priority::Low);
    agent.acmd("effect_specialairlwend", acmd_stub, Priority::Low);
    agent.acmd("sound_specialairlwend", sound_speciallwend, Priority::Low);
    agent.acmd("expression_specialairlwend", expression_speciallwend, Priority::Low);
}