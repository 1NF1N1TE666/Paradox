use super::*;
use interpolation::Lerp;

unsafe extern "C" fn game_specialnstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 14.0);
    if is_excute(agent) {
        if !VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE) {
            ArticleModule::generate_article_enable(boma, *FIGHTER_SAMUS_GENERATE_ARTICLE_CSHOT, false, -1);
            WorkModule::on_flag(boma, *FIGHTER_SAMUS_STATUS_SPECIAL_N_FLAG_BULLET_DISP);
        }
    }
}

unsafe extern "C" fn effect_specialnhold(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let ice = VarModule::is_flag(boma.object(), vars::samus::instance::ICE_MODE);
    frame(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("samus_cshot_hold"), Hash40::new("armr"), 7.98, -0.506, -0.251, -91.273, -1.797, 176.373, 1, true);
        EffectModule::enable_sync_init_pos_last(boma);
        if ice {
            LAST_EFFECT_SET_COLOR(agent,0.25, 1.5,1.0);
            EFFECT_FOLLOW(agent, Hash40::new("sys_ice"), Hash40::new("armr"), 7.0, 0.0, 0.0, 0, 0, -90, 0.075, true);
        }
    }
}

unsafe extern "C" fn game_specialnice(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let angle = VarModule::get_float(boma.object(), vars::samus::instance::AIM_ANGLE);
    let mut damage = 1.0;
    let mut size = 1.0;
    frame(lua_state, 3.0);
    if is_excute(agent) {
        let charge = boma.get_int(*FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_COUNT) as f32 / WorkModule::get_param_float(boma, hash40("param_special_n"), hash40("cshot_charge_frame"));
        let length = Lerp::lerp(&0.0, &10.0, &charge);
        damage = Lerp::lerp(&5.0, &25.0, &charge);
        size = Lerp::lerp(&1.0, &5.0, &charge);
        ATTACK(agent, 0, 0, Hash40::new("top"), damage, if angle > 60.0 {90} else if angle > 30.0 {60} else if angle > -30.0 {361} else if angle > -60.0 {300} else {270}, 100, 0, 0, size, 0.0, 10.0 + (if charge < 1.0 {15.0 - ((1.0 - charge) * 7.5)} else {15.0}) * angle.to_radians().sin(), (if charge < 1.0 {15.0 - ((1.0 - charge) * 7.5)} else {15.0}) * angle.to_radians().cos(), Some(0.0), Some(10.0 + (15.0 + length) * angle.to_radians().sin()), Some((15.0 + length) * angle.to_radians().cos()), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), if charge < 0.5 {*ATTACK_SOUND_LEVEL_S} else if charge < 1.0 {*ATTACK_SOUND_LEVEL_M} else {*ATTACK_SOUND_LEVEL_L}, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_ENERGY);
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        AttackModule::set_size(boma,0,size * 1.25);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_specialnice(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        let charge = boma.get_int(*FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_COUNT) as f32 / WorkModule::get_param_float(boma, hash40("param_special_n"), hash40("cshot_charge_frame"));
        let length = Lerp::lerp(&(1.0 as f32), &(3.0 as f32), &charge);
        EFFECT(agent, Hash40::new("sys_muzzleflash"), Hash40::new("armr"), 7.5, 0.0, 0.0, 0, 0, 0, length / 1.25, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(agent,0.25, 0.875, 1.0);
        LAST_EFFECT_SET_RATE(agent, 0.75);
        LAST_EFFECT_SET_SCALE_W(agent, length / 2.0, length, length / 2.0);
        if charge >= 0.5 {
            EFFECT(agent, Hash40::new("samus_cshot_shot"), Hash40::new("top"), 5, 6, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        if boma.is_motion(Hash40::new("effect_specialairnice")) {
            LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            FLASH(agent, 1, 0.755, 1, 0.705);
        }
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        FLASH_FRM(agent, 10, 0.314, 0.314, 0.314, 0);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
}

unsafe extern "C" fn sound_specialnice(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_samus_special_n01"));
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        let charge = boma.get_int(*FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_COUNT) as f32 / WorkModule::get_param_float(boma, hash40("param_special_n"), hash40("cshot_charge_frame"));
        if charge <= 0.25 {
            PLAY_SE_REMAIN(agent, Hash40::new("se_samus_special_n02"));
        } else if charge <= 0.625 {
            PLAY_SE_REMAIN(agent, Hash40::new("se_samus_special_n03"));
        } else if charge <= 0.875 {
            PLAY_SE_REMAIN(agent, Hash40::new("se_samus_special_n04"));
        } else {
            PLAY_SE_REMAIN(agent, Hash40::new("se_samus_special_n05"));
        }
    }
}

unsafe extern "C" fn expression_specialnice(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        if boma.is_motion_one_of(&[
            Hash40::new("special_n_i_max"),
            Hash40::new("special_air_n_i_max"),
        ]) {
            QUAKE(agent, *CAMERA_QUAKE_KIND_S);
            VisibilityModule::set_int64(boma, hash40("body") as i64, hash40("body_hide_gun") as i64);
            ArticleModule::remove_exist(boma, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            ArticleModule::generate_article(boma, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, true, -1);
            ArticleModule::change_motion(boma, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, Hash40::new("special_n"), false, -1.0);
        }
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        if boma.is_motion_one_of(&[
            Hash40::new("special_n_i_max"),
            Hash40::new("special_air_n_i_max"),
        ]) {
            ArticleModule::remove_exist(boma, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            VisibilityModule::set_int64(boma, hash40("body") as i64, hash40("body_normal") as i64);
        }
    }
}

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
        if !(&mut *article_boma).is_status(*WEAPON_SAMUS_GBEAM_STATUS_KIND_SHOOT) && GroundModule::get_distance_to_floor(boma, pos, 0.0, false) < 6.0 && angle > 70.0 {
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
    agent.acmd("game_specialnstart", game_specialnstart, Priority::Low);
    agent.acmd("game_specialairnstart", game_specialnstart, Priority::Low);
    agent.acmd("effect_specialnhold", effect_specialnhold, Priority::Low);
    agent.acmd("effect_specialairnhold", effect_specialnhold, Priority::Low);
    agent.acmd("game_specialnice", game_specialnice, Priority::Low);
    agent.acmd("effect_specialnice", effect_specialnice, Priority::Low);
    agent.acmd("sound_specialnice", sound_specialnice, Priority::Low);
    agent.acmd("expression_specialnice", expression_specialnice, Priority::Low);
    agent.acmd("game_specialairnice", game_specialnice, Priority::Low);
    agent.acmd("effect_specialairnice", effect_specialnice, Priority::Low);
    agent.acmd("sound_specialairnice", sound_specialnice, Priority::Low);
    agent.acmd("expression_specialairnice", expression_specialnice, Priority::Low);
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