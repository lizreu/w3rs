#![allow(clippy::should_implement_trait)]
#![allow(clippy::style)]
#![allow(clippy::too_many_arguments)]
#![allow(non_camel_case_types)]
use handles::*;
use crate::base::Handle;
use crate::base::j_handle;
use crate::base::CodeId;
#[rustfmt::skip]
pub mod natives {
    use super::*;
    crate::declare_ffi! {
        {
            link: "ConvertRace"
            name: convert_race
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_race(i)
            };
        },
        {
            link: "ConvertAllianceType"
            name: convert_alliance_type
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_alliance_type(i)
            };
        },
        {
            link: "ConvertRacePref"
            name: convert_race_pref
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_race_pref(i)
            };
        },
        {
            link: "ConvertIGameState"
            name: convert_i_game_state
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_i_game_state(i)
            };
        },
        {
            link: "ConvertFGameState"
            name: convert_f_game_state
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_f_game_state(i)
            };
        },
        {
            link: "ConvertPlayerState"
            name: convert_player_state
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_player_state(i)
            };
        },
        {
            link: "ConvertPlayerScore"
            name: convert_player_score
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_player_score(i)
            };
        },
        {
            link: "ConvertPlayerGameResult"
            name: convert_player_game_result
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_player_game_result(i)
            };
        },
        {
            link: "ConvertUnitState"
            name: convert_unit_state
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_unit_state(i)
            };
        },
        {
            link: "ConvertAIDifficulty"
            name: convert_a_i_difficulty
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_a_i_difficulty(i)
            };
        },
        {
            link: "ConvertGameEvent"
            name: convert_game_event
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_game_event(i)
            };
        },
        {
            link: "ConvertPlayerEvent"
            name: convert_player_event
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_player_event(i)
            };
        },
        {
            link: "ConvertPlayerUnitEvent"
            name: convert_player_unit_event
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_player_unit_event(i)
            };
        },
        {
            link: "ConvertWidgetEvent"
            name: convert_widget_event
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_widget_event(i)
            };
        },
        {
            link: "ConvertDialogEvent"
            name: convert_dialog_event
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_dialog_event(i)
            };
        },
        {
            link: "ConvertUnitEvent"
            name: convert_unit_event
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_unit_event(i)
            };
        },
        {
            link: "ConvertLimitOp"
            name: convert_limit_op
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_limit_op(i)
            };
        },
        {
            link: "ConvertUnitType"
            name: convert_unit_type
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_unit_type(i)
            };
        },
        {
            link: "ConvertGameSpeed"
            name: convert_game_speed
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_game_speed(i)
            };
        },
        {
            link: "ConvertPlacement"
            name: convert_placement
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_placement(i)
            };
        },
        {
            link: "ConvertStartLocPrio"
            name: convert_start_loc_prio
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_start_loc_prio(i)
            };
        },
        {
            link: "ConvertGameDifficulty"
            name: convert_game_difficulty
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_game_difficulty(i)
            };
        },
        {
            link: "ConvertGameType"
            name: convert_game_type
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_game_type(i)
            };
        },
        {
            link: "ConvertMapFlag"
            name: convert_map_flag
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_map_flag(i)
            };
        },
        {
            link: "ConvertMapVisibility"
            name: convert_map_visibility
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_map_visibility(i)
            };
        },
        {
            link: "ConvertMapSetting"
            name: convert_map_setting
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_map_setting(i)
            };
        },
        {
            link: "ConvertMapDensity"
            name: convert_map_density
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_map_density(i)
            };
        },
        {
            link: "ConvertMapControl"
            name: convert_map_control
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_map_control(i)
            };
        },
        {
            link: "ConvertPlayerColor"
            name: convert_player_color
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_player_color(i)
            };
        },
        {
            link: "ConvertPlayerSlotState"
            name: convert_player_slot_state
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_player_slot_state(i)
            };
        },
        {
            link: "ConvertVolumeGroup"
            name: convert_volume_group
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_volume_group(i)
            };
        },
        {
            link: "ConvertCameraField"
            name: convert_camera_field
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_camera_field(i)
            };
        },
        {
            link: "ConvertBlendMode"
            name: convert_blend_mode
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_blend_mode(i)
            };
        },
        {
            link: "ConvertRarityControl"
            name: convert_rarity_control
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_rarity_control(i)
            };
        },
        {
            link: "ConvertTexMapFlags"
            name: convert_tex_map_flags
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_tex_map_flags(i)
            };
        },
        {
            link: "ConvertFogState"
            name: convert_fog_state
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_fog_state(i)
            };
        },
        {
            link: "ConvertEffectType"
            name: convert_effect_type
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_effect_type(i)
            };
        },
        {
            link: "ConvertVersion"
            name: convert_version
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_version(i)
            };
        },
        {
            link: "ConvertItemType"
            name: convert_item_type
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_item_type(i)
            };
        },
        {
            link: "ConvertAttackType"
            name: convert_attack_type
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_attack_type(i)
            };
        },
        {
            link: "ConvertDamageType"
            name: convert_damage_type
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_damage_type(i)
            };
        },
        {
            link: "ConvertWeaponType"
            name: convert_weapon_type
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_weapon_type(i)
            };
        },
        {
            link: "ConvertSoundType"
            name: convert_sound_type
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_sound_type(i)
            };
        },
        {
            link: "ConvertPathingType"
            name: convert_pathing_type
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_pathing_type(i)
            };
        },
        {
            link: "ConvertMouseButtonType"
            name: convert_mouse_button_type
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_mouse_button_type(i)
            };
        },
        {
            link: "ConvertAnimType"
            name: convert_anim_type
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_anim_type(i)
            };
        },
        {
            link: "ConvertSubAnimType"
            name: convert_sub_anim_type
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_sub_anim_type(i)
            };
        },
        {
            link: "ConvertOriginFrameType"
            name: convert_origin_frame_type
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_origin_frame_type(i)
            };
        },
        {
            link: "ConvertFramePointType"
            name: convert_frame_point_type
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_frame_point_type(i)
            };
        },
        {
            link: "ConvertTextAlignType"
            name: convert_text_align_type
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_text_align_type(i)
            };
        },
        {
            link: "ConvertFrameEventType"
            name: convert_frame_event_type
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_frame_event_type(i)
            };
        },
        {
            link: "ConvertOsKeyType"
            name: convert_os_key_type
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_os_key_type(i)
            };
        },
        {
            link: "ConvertAbilityIntegerField"
            name: convert_ability_integer_field
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_ability_integer_field(i)
            };
        },
        {
            link: "ConvertAbilityRealField"
            name: convert_ability_real_field
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_ability_real_field(i)
            };
        },
        {
            link: "ConvertAbilityBooleanField"
            name: convert_ability_boolean_field
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_ability_boolean_field(i)
            };
        },
        {
            link: "ConvertAbilityStringField"
            name: convert_ability_string_field
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_ability_string_field(i)
            };
        },
        {
            link: "ConvertAbilityIntegerLevelField"
            name: convert_ability_integer_level_field
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_ability_integer_level_field(i)
            };
        },
        {
            link: "ConvertAbilityRealLevelField"
            name: convert_ability_real_level_field
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_ability_real_level_field(i)
            };
        },
        {
            link: "ConvertAbilityBooleanLevelField"
            name: convert_ability_boolean_level_field
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_ability_boolean_level_field(i)
            };
        },
        {
            link: "ConvertAbilityStringLevelField"
            name: convert_ability_string_level_field
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_ability_string_level_field(i)
            };
        },
        {
            link: "ConvertAbilityIntegerLevelArrayField"
            name: convert_ability_integer_level_array_field
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_ability_integer_level_array_field(i)
            };
        },
        {
            link: "ConvertAbilityRealLevelArrayField"
            name: convert_ability_real_level_array_field
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_ability_real_level_array_field(i)
            };
        },
        {
            link: "ConvertAbilityBooleanLevelArrayField"
            name: convert_ability_boolean_level_array_field
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_ability_boolean_level_array_field(i)
            };
        },
        {
            link: "ConvertAbilityStringLevelArrayField"
            name: convert_ability_string_level_array_field
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_ability_string_level_array_field(i)
            };
        },
        {
            link: "ConvertUnitIntegerField"
            name: convert_unit_integer_field
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_unit_integer_field(i)
            };
        },
        {
            link: "ConvertUnitRealField"
            name: convert_unit_real_field
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_unit_real_field(i)
            };
        },
        {
            link: "ConvertUnitBooleanField"
            name: convert_unit_boolean_field
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_unit_boolean_field(i)
            };
        },
        {
            link: "ConvertUnitStringField"
            name: convert_unit_string_field
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_unit_string_field(i)
            };
        },
        {
            link: "ConvertUnitWeaponIntegerField"
            name: convert_unit_weapon_integer_field
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_unit_weapon_integer_field(i)
            };
        },
        {
            link: "ConvertUnitWeaponRealField"
            name: convert_unit_weapon_real_field
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_unit_weapon_real_field(i)
            };
        },
        {
            link: "ConvertUnitWeaponBooleanField"
            name: convert_unit_weapon_boolean_field
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_unit_weapon_boolean_field(i)
            };
        },
        {
            link: "ConvertUnitWeaponStringField"
            name: convert_unit_weapon_string_field
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_unit_weapon_string_field(i)
            };
        },
        {
            link: "ConvertItemIntegerField"
            name: convert_item_integer_field
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_item_integer_field(i)
            };
        },
        {
            link: "ConvertItemRealField"
            name: convert_item_real_field
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_item_real_field(i)
            };
        },
        {
            link: "ConvertItemBooleanField"
            name: convert_item_boolean_field
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_item_boolean_field(i)
            };
        },
        {
            link: "ConvertItemStringField"
            name: convert_item_string_field
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_item_string_field(i)
            };
        },
        {
            link: "ConvertMoveType"
            name: convert_move_type
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_move_type(i)
            };
        },
        {
            link: "ConvertTargetFlag"
            name: convert_target_flag
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_target_flag(i)
            };
        },
        {
            link: "ConvertArmorType"
            name: convert_armor_type
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_armor_type(i)
            };
        },
        {
            link: "ConvertHeroAttribute"
            name: convert_hero_attribute
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_hero_attribute(i)
            };
        },
        {
            link: "ConvertDefenseType"
            name: convert_defense_type
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_defense_type(i)
            };
        },
        {
            link: "ConvertRegenType"
            name: convert_regen_type
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_regen_type(i)
            };
        },
        {
            link: "ConvertUnitCategory"
            name: convert_unit_category
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_unit_category(i)
            };
        },
        {
            link: "ConvertPathingFlag"
            name: convert_pathing_flag
            raw: (i: i32) -> i32;
            user: (i: i32) -> i32 {
                raw::convert_pathing_flag(i)
            };
        },
        {
            link: "OrderId"
            name: order_id
            raw: (order_id_string: *const i8) -> i32;
            user: (order_id_string: &CStr) -> i32 {
                raw::order_id(order_id_string.as_ptr())
            };
        },
        {
            link: "OrderId2String"
            name: order_id2_string
            raw: (order_id: i32, out: *mut i8, out_len: *mut usize);
            user: (order_id: i32) -> String {
                raw::order_id2_string(order_id, stringret::ptr(), stringret::reset_len()); stringret::read_string()
            };
        },
        {
            link: "UnitId"
            name: unit_id
            raw: (unit_id_string: *const i8) -> i32;
            user: (unit_id_string: &CStr) -> i32 {
                raw::unit_id(unit_id_string.as_ptr())
            };
        },
        {
            link: "UnitId2String"
            name: unit_id2_string
            raw: (unit_id: i32, out: *mut i8, out_len: *mut usize);
            user: (unit_id: i32) -> String {
                raw::unit_id2_string(unit_id, stringret::ptr(), stringret::reset_len()); stringret::read_string()
            };
        },
        {
            link: "AbilityId"
            name: ability_id
            raw: (ability_id_string: *const i8) -> i32;
            user: (ability_id_string: &CStr) -> i32 {
                raw::ability_id(ability_id_string.as_ptr())
            };
        },
        {
            link: "AbilityId2String"
            name: ability_id2_string
            raw: (ability_id: i32, out: *mut i8, out_len: *mut usize);
            user: (ability_id: i32) -> String {
                raw::ability_id2_string(ability_id, stringret::ptr(), stringret::reset_len()); stringret::read_string()
            };
        },
        {
            link: "GetObjectName"
            name: get_object_name
            raw: (object_id: i32, out: *mut i8, out_len: *mut usize);
            user: (object_id: i32) -> String {
                raw::get_object_name(object_id, stringret::ptr(), stringret::reset_len()); stringret::read_string()
            };
        },
        {
            link: "GetBJMaxPlayers"
            name: get_b_j_max_players
            raw: () -> i32;
            user: () -> i32 {
                raw::get_b_j_max_players()
            };
        },
        {
            link: "GetBJPlayerNeutralVictim"
            name: get_b_j_player_neutral_victim
            raw: () -> i32;
            user: () -> i32 {
                raw::get_b_j_player_neutral_victim()
            };
        },
        {
            link: "GetBJPlayerNeutralExtra"
            name: get_b_j_player_neutral_extra
            raw: () -> i32;
            user: () -> i32 {
                raw::get_b_j_player_neutral_extra()
            };
        },
        {
            link: "GetBJMaxPlayerSlots"
            name: get_b_j_max_player_slots
            raw: () -> i32;
            user: () -> i32 {
                raw::get_b_j_max_player_slots()
            };
        },
        {
            link: "GetPlayerNeutralPassive"
            name: get_player_neutral_passive
            raw: () -> i32;
            user: () -> i32 {
                raw::get_player_neutral_passive()
            };
        },
        {
            link: "GetPlayerNeutralAggressive"
            name: get_player_neutral_aggressive
            raw: () -> i32;
            user: () -> i32 {
                raw::get_player_neutral_aggressive()
            };
        },
        {
            link: "Deg2Rad"
            name: deg2_rad
            raw: (degrees: f32) -> f32;
            user: (degrees: f32) -> f32 {
                raw::deg2_rad(degrees)
            };
        },
        {
            link: "Rad2Deg"
            name: rad2_deg
            raw: (radians: f32) -> f32;
            user: (radians: f32) -> f32 {
                raw::rad2_deg(radians)
            };
        },
        {
            link: "Sin"
            name: sin
            raw: (radians: f32) -> f32;
            user: (radians: f32) -> f32 {
                raw::sin(radians)
            };
        },
        {
            link: "Cos"
            name: cos
            raw: (radians: f32) -> f32;
            user: (radians: f32) -> f32 {
                raw::cos(radians)
            };
        },
        {
            link: "Tan"
            name: tan
            raw: (radians: f32) -> f32;
            user: (radians: f32) -> f32 {
                raw::tan(radians)
            };
        },
        {
            link: "Asin"
            name: asin
            raw: (y: f32) -> f32;
            user: (y: f32) -> f32 {
                raw::asin(y)
            };
        },
        {
            link: "Acos"
            name: acos
            raw: (x: f32) -> f32;
            user: (x: f32) -> f32 {
                raw::acos(x)
            };
        },
        {
            link: "Atan"
            name: atan
            raw: (x: f32) -> f32;
            user: (x: f32) -> f32 {
                raw::atan(x)
            };
        },
        {
            link: "Atan2"
            name: atan2
            raw: (y: f32, x: f32) -> f32;
            user: (y: f32, x: f32) -> f32 {
                raw::atan2(y, x)
            };
        },
        {
            link: "SquareRoot"
            name: square_root
            raw: (x: f32) -> f32;
            user: (x: f32) -> f32 {
                raw::square_root(x)
            };
        },
        {
            link: "Pow"
            name: pow
            raw: (x: f32, power: f32) -> f32;
            user: (x: f32, power: f32) -> f32 {
                raw::pow(x, power)
            };
        },
        {
            link: "MathRound"
            name: math_round
            raw: (r: f32) -> i32;
            user: (r: f32) -> i32 {
                raw::math_round(r)
            };
        },
        {
            link: "I2R"
            name: i2_r
            raw: (i: i32) -> f32;
            user: (i: i32) -> f32 {
                raw::i2_r(i)
            };
        },
        {
            link: "R2I"
            name: r2_i
            raw: (r: f32) -> i32;
            user: (r: f32) -> i32 {
                raw::r2_i(r)
            };
        },
        {
            link: "I2S"
            name: i2_s
            raw: (i: i32, out: *mut i8, out_len: *mut usize);
            user: (i: i32) -> String {
                raw::i2_s(i, stringret::ptr(), stringret::reset_len()); stringret::read_string()
            };
        },
        {
            link: "R2S"
            name: r2_s
            raw: (r: f32, out: *mut i8, out_len: *mut usize);
            user: (r: f32) -> String {
                raw::r2_s(r, stringret::ptr(), stringret::reset_len()); stringret::read_string()
            };
        },
        {
            link: "R2SW"
            name: r2_s_w
            raw: (r: f32, width: i32, precision: i32, out: *mut i8, out_len: *mut usize);
            user: (r: f32, width: i32, precision: i32) -> String {
                raw::r2_s_w(r, width, precision, stringret::ptr(), stringret::reset_len()); stringret::read_string()
            };
        },
        {
            link: "S2I"
            name: s2_i
            raw: (s: *const i8) -> i32;
            user: (s: &CStr) -> i32 {
                raw::s2_i(s.as_ptr())
            };
        },
        {
            link: "S2R"
            name: s2_r
            raw: (s: *const i8) -> f32;
            user: (s: &CStr) -> f32 {
                raw::s2_r(s.as_ptr())
            };
        },
        {
            link: "GetHandleId"
            name: get_handle_id
            raw: (h: Handle<j_handle>) -> i32;
            user: (h: Handle<j_handle>) -> i32 {
                raw::get_handle_id(h)
            };
        },
        {
            link: "SubString"
            name: sub_string
            raw: (source: *const i8, start: i32, end: i32, out: *mut i8, out_len: *mut usize);
            user: (source: &CStr, start: i32, end: i32) -> String {
                raw::sub_string(source.as_ptr(), start, end, stringret::ptr(), stringret::reset_len()); stringret::read_string()
            };
        },
        {
            link: "StringLength"
            name: string_length
            raw: (s: *const i8) -> i32;
            user: (s: &CStr) -> i32 {
                raw::string_length(s.as_ptr())
            };
        },
        {
            link: "StringCase"
            name: string_case
            raw: (source: *const i8, upper: bool, out: *mut i8, out_len: *mut usize);
            user: (source: &CStr, upper: bool) -> String {
                raw::string_case(source.as_ptr(), upper, stringret::ptr(), stringret::reset_len()); stringret::read_string()
            };
        },
        {
            link: "StringHash"
            name: string_hash
            raw: (s: *const i8) -> i32;
            user: (s: &CStr) -> i32 {
                raw::string_hash(s.as_ptr())
            };
        },
        {
            link: "GetLocalizedString"
            name: get_localized_string
            raw: (source: *const i8, out: *mut i8, out_len: *mut usize);
            user: (source: &CStr) -> String {
                raw::get_localized_string(source.as_ptr(), stringret::ptr(), stringret::reset_len()); stringret::read_string()
            };
        },
        {
            link: "GetLocalizedHotkey"
            name: get_localized_hotkey
            raw: (source: *const i8) -> i32;
            user: (source: &CStr) -> i32 {
                raw::get_localized_hotkey(source.as_ptr())
            };
        },
        {
            link: "SetMapName"
            name: set_map_name
            raw: (name: *const i8);
            user: (name: &CStr) {
                raw::set_map_name(name.as_ptr())
            };
        },
        {
            link: "SetMapDescription"
            name: set_map_description
            raw: (description: *const i8);
            user: (description: &CStr) {
                raw::set_map_description(description.as_ptr())
            };
        },
        {
            link: "SetTeams"
            name: set_teams
            raw: (teamcount: i32);
            user: (teamcount: i32) {
                raw::set_teams(teamcount)
            };
        },
        {
            link: "SetPlayers"
            name: set_players
            raw: (playercount: i32);
            user: (playercount: i32) {
                raw::set_players(playercount)
            };
        },
        {
            link: "DefineStartLocation"
            name: define_start_location
            raw: (which_start_loc: i32, x: f32, y: f32);
            user: (which_start_loc: i32, x: f32, y: f32) {
                raw::define_start_location(which_start_loc, x, y)
            };
        },
        {
            link: "DefineStartLocationLoc"
            name: define_start_location_loc
            raw: (which_start_loc: i32, which_location: Handle<j_location>);
            user: (which_start_loc: i32, which_location: &Agent<j_location>) {
                raw::define_start_location_loc(which_start_loc, which_location.to_handle())
            };
        },
        {
            link: "SetStartLocPrioCount"
            name: set_start_loc_prio_count
            raw: (which_start_loc: i32, prio_slot_count: i32);
            user: (which_start_loc: i32, prio_slot_count: i32) {
                raw::set_start_loc_prio_count(which_start_loc, prio_slot_count)
            };
        },
        {
            link: "SetStartLocPrio"
            name: set_start_loc_prio
            raw: (which_start_loc: i32, prio_slot_index: i32, other_start_loc_index: i32, priority: i32);
            user: (which_start_loc: i32, prio_slot_index: i32, other_start_loc_index: i32, priority: i32) {
                raw::set_start_loc_prio(which_start_loc, prio_slot_index, other_start_loc_index, priority)
            };
        },
        {
            link: "GetStartLocPrioSlot"
            name: get_start_loc_prio_slot
            raw: (which_start_loc: i32, prio_slot_index: i32) -> i32;
            user: (which_start_loc: i32, prio_slot_index: i32) -> i32 {
                raw::get_start_loc_prio_slot(which_start_loc, prio_slot_index)
            };
        },
        {
            link: "GetStartLocPrio"
            name: get_start_loc_prio
            raw: (which_start_loc: i32, prio_slot_index: i32) -> i32;
            user: (which_start_loc: i32, prio_slot_index: i32) -> i32 {
                raw::get_start_loc_prio(which_start_loc, prio_slot_index)
            };
        },
        {
            link: "SetEnemyStartLocPrioCount"
            name: set_enemy_start_loc_prio_count
            raw: (which_start_loc: i32, prio_slot_count: i32);
            user: (which_start_loc: i32, prio_slot_count: i32) {
                raw::set_enemy_start_loc_prio_count(which_start_loc, prio_slot_count)
            };
        },
        {
            link: "SetEnemyStartLocPrio"
            name: set_enemy_start_loc_prio
            raw: (which_start_loc: i32, prio_slot_index: i32, other_start_loc_index: i32, priority: i32);
            user: (which_start_loc: i32, prio_slot_index: i32, other_start_loc_index: i32, priority: i32) {
                raw::set_enemy_start_loc_prio(which_start_loc, prio_slot_index, other_start_loc_index, priority)
            };
        },
        {
            link: "SetGameTypeSupported"
            name: set_game_type_supported
            raw: (which_game_type: i32, value: bool);
            user: (which_game_type: i32, value: bool) {
                raw::set_game_type_supported(which_game_type, value)
            };
        },
        {
            link: "SetMapFlag"
            name: set_map_flag
            raw: (which_map_flag: i32, value: bool);
            user: (which_map_flag: i32, value: bool) {
                raw::set_map_flag(which_map_flag, value)
            };
        },
        {
            link: "SetGamePlacement"
            name: set_game_placement
            raw: (which_placement_type: i32);
            user: (which_placement_type: i32) {
                raw::set_game_placement(which_placement_type)
            };
        },
        {
            link: "SetGameSpeed"
            name: set_game_speed
            raw: (whichspeed: i32);
            user: (whichspeed: i32) {
                raw::set_game_speed(whichspeed)
            };
        },
        {
            link: "SetGameDifficulty"
            name: set_game_difficulty
            raw: (whichdifficulty: i32);
            user: (whichdifficulty: i32) {
                raw::set_game_difficulty(whichdifficulty)
            };
        },
        {
            link: "SetResourceDensity"
            name: set_resource_density
            raw: (whichdensity: i32);
            user: (whichdensity: i32) {
                raw::set_resource_density(whichdensity)
            };
        },
        {
            link: "SetCreatureDensity"
            name: set_creature_density
            raw: (whichdensity: i32);
            user: (whichdensity: i32) {
                raw::set_creature_density(whichdensity)
            };
        },
        {
            link: "GetTeams"
            name: get_teams
            raw: () -> i32;
            user: () -> i32 {
                raw::get_teams()
            };
        },
        {
            link: "GetPlayers"
            name: get_players
            raw: () -> i32;
            user: () -> i32 {
                raw::get_players()
            };
        },
        {
            link: "IsGameTypeSupported"
            name: is_game_type_supported
            raw: (which_game_type: i32) -> bool;
            user: (which_game_type: i32) -> bool {
                raw::is_game_type_supported(which_game_type)
            };
        },
        {
            link: "GetGameTypeSelected"
            name: get_game_type_selected
            raw: () -> i32;
            user: () -> i32 {
                raw::get_game_type_selected()
            };
        },
        {
            link: "IsMapFlagSet"
            name: is_map_flag_set
            raw: (which_map_flag: i32) -> bool;
            user: (which_map_flag: i32) -> bool {
                raw::is_map_flag_set(which_map_flag)
            };
        },
        {
            link: "GetGamePlacement"
            name: get_game_placement
            raw: () -> i32;
            user: () -> i32 {
                raw::get_game_placement()
            };
        },
        {
            link: "GetGameSpeed"
            name: get_game_speed
            raw: () -> i32;
            user: () -> i32 {
                raw::get_game_speed()
            };
        },
        {
            link: "GetGameDifficulty"
            name: get_game_difficulty
            raw: () -> i32;
            user: () -> i32 {
                raw::get_game_difficulty()
            };
        },
        {
            link: "GetResourceDensity"
            name: get_resource_density
            raw: () -> i32;
            user: () -> i32 {
                raw::get_resource_density()
            };
        },
        {
            link: "GetCreatureDensity"
            name: get_creature_density
            raw: () -> i32;
            user: () -> i32 {
                raw::get_creature_density()
            };
        },
        {
            link: "GetStartLocationX"
            name: get_start_location_x
            raw: (which_start_location: i32) -> f32;
            user: (which_start_location: i32) -> f32 {
                raw::get_start_location_x(which_start_location)
            };
        },
        {
            link: "GetStartLocationY"
            name: get_start_location_y
            raw: (which_start_location: i32) -> f32;
            user: (which_start_location: i32) -> f32 {
                raw::get_start_location_y(which_start_location)
            };
        },
        {
            link: "GetStartLocationLoc"
            name: get_start_location_loc
            raw: (which_start_location: i32) -> Handle<j_location>;
            user: (which_start_location: i32) -> Option<Weak<j_location>> {
                raw::get_start_location_loc(which_start_location).to_weak()
            };
        },
        {
            link: "SetPlayerTeam"
            name: set_player_team
            raw: (which_player: Handle<j_player>, which_team: i32);
            user: (which_player: &Agent<j_player>, which_team: i32) {
                raw::set_player_team(which_player.to_handle(), which_team)
            };
        },
        {
            link: "SetPlayerStartLocation"
            name: set_player_start_location
            raw: (which_player: Handle<j_player>, start_loc_index: i32);
            user: (which_player: &Agent<j_player>, start_loc_index: i32) {
                raw::set_player_start_location(which_player.to_handle(), start_loc_index)
            };
        },
        {
            link: "ForcePlayerStartLocation"
            name: force_player_start_location
            raw: (which_player: Handle<j_player>, start_loc_index: i32);
            user: (which_player: &Agent<j_player>, start_loc_index: i32) {
                raw::force_player_start_location(which_player.to_handle(), start_loc_index)
            };
        },
        {
            link: "SetPlayerColor"
            name: set_player_color
            raw: (which_player: Handle<j_player>, color: i32);
            user: (which_player: &Agent<j_player>, color: i32) {
                raw::set_player_color(which_player.to_handle(), color)
            };
        },
        {
            link: "SetPlayerAlliance"
            name: set_player_alliance
            raw: (source_player: Handle<j_player>, other_player: Handle<j_player>, which_alliance_setting: i32, value: bool);
            user: (source_player: &Agent<j_player>, other_player: &Agent<j_player>, which_alliance_setting: i32, value: bool) {
                raw::set_player_alliance(source_player.to_handle(), other_player.to_handle(), which_alliance_setting, value)
            };
        },
        {
            link: "SetPlayerTaxRate"
            name: set_player_tax_rate
            raw: (source_player: Handle<j_player>, other_player: Handle<j_player>, which_resource: i32, rate: i32);
            user: (source_player: &Agent<j_player>, other_player: &Agent<j_player>, which_resource: i32, rate: i32) {
                raw::set_player_tax_rate(source_player.to_handle(), other_player.to_handle(), which_resource, rate)
            };
        },
        {
            link: "SetPlayerRacePreference"
            name: set_player_race_preference
            raw: (which_player: Handle<j_player>, which_race_preference: i32);
            user: (which_player: &Agent<j_player>, which_race_preference: i32) {
                raw::set_player_race_preference(which_player.to_handle(), which_race_preference)
            };
        },
        {
            link: "SetPlayerRaceSelectable"
            name: set_player_race_selectable
            raw: (which_player: Handle<j_player>, value: bool);
            user: (which_player: &Agent<j_player>, value: bool) {
                raw::set_player_race_selectable(which_player.to_handle(), value)
            };
        },
        {
            link: "SetPlayerController"
            name: set_player_controller
            raw: (which_player: Handle<j_player>, control_type: i32);
            user: (which_player: &Agent<j_player>, control_type: i32) {
                raw::set_player_controller(which_player.to_handle(), control_type)
            };
        },
        {
            link: "SetPlayerName"
            name: set_player_name
            raw: (which_player: Handle<j_player>, name: *const i8);
            user: (which_player: &Agent<j_player>, name: &CStr) {
                raw::set_player_name(which_player.to_handle(), name.as_ptr())
            };
        },
        {
            link: "SetPlayerOnScoreScreen"
            name: set_player_on_score_screen
            raw: (which_player: Handle<j_player>, flag: bool);
            user: (which_player: &Agent<j_player>, flag: bool) {
                raw::set_player_on_score_screen(which_player.to_handle(), flag)
            };
        },
        {
            link: "GetPlayerTeam"
            name: get_player_team
            raw: (which_player: Handle<j_player>) -> i32;
            user: (which_player: &Agent<j_player>) -> i32 {
                raw::get_player_team(which_player.to_handle())
            };
        },
        {
            link: "GetPlayerStartLocation"
            name: get_player_start_location
            raw: (which_player: Handle<j_player>) -> i32;
            user: (which_player: &Agent<j_player>) -> i32 {
                raw::get_player_start_location(which_player.to_handle())
            };
        },
        {
            link: "GetPlayerColor"
            name: get_player_color
            raw: (which_player: Handle<j_player>) -> i32;
            user: (which_player: &Agent<j_player>) -> i32 {
                raw::get_player_color(which_player.to_handle())
            };
        },
        {
            link: "GetPlayerSelectable"
            name: get_player_selectable
            raw: (which_player: Handle<j_player>) -> bool;
            user: (which_player: &Agent<j_player>) -> bool {
                raw::get_player_selectable(which_player.to_handle())
            };
        },
        {
            link: "GetPlayerController"
            name: get_player_controller
            raw: (which_player: Handle<j_player>) -> i32;
            user: (which_player: &Agent<j_player>) -> i32 {
                raw::get_player_controller(which_player.to_handle())
            };
        },
        {
            link: "GetPlayerSlotState"
            name: get_player_slot_state
            raw: (which_player: Handle<j_player>) -> i32;
            user: (which_player: &Agent<j_player>) -> i32 {
                raw::get_player_slot_state(which_player.to_handle())
            };
        },
        {
            link: "GetPlayerTaxRate"
            name: get_player_tax_rate
            raw: (source_player: Handle<j_player>, other_player: Handle<j_player>, which_resource: i32) -> i32;
            user: (source_player: &Agent<j_player>, other_player: &Agent<j_player>, which_resource: i32) -> i32 {
                raw::get_player_tax_rate(source_player.to_handle(), other_player.to_handle(), which_resource)
            };
        },
        {
            link: "IsPlayerRacePrefSet"
            name: is_player_race_pref_set
            raw: (which_player: Handle<j_player>, pref: i32) -> bool;
            user: (which_player: &Agent<j_player>, pref: i32) -> bool {
                raw::is_player_race_pref_set(which_player.to_handle(), pref)
            };
        },
        {
            link: "GetPlayerName"
            name: get_player_name
            raw: (which_player: Handle<j_player>, out: *mut i8, out_len: *mut usize);
            user: (which_player: &Agent<j_player>) -> String {
                raw::get_player_name(which_player.to_handle(), stringret::ptr(), stringret::reset_len()); stringret::read_string()
            };
        },
        {
            link: "CreateTimer"
            name: create_timer
            raw: () -> Handle<j_timer>;
            user: () -> Option<Weak<j_timer>> {
                raw::create_timer().to_weak()
            };
        },
        {
            link: "DestroyTimer"
            name: destroy_timer
            raw: (which_timer: Handle<j_timer>);
            user: (which_timer: &Agent<j_timer>) {
                raw::destroy_timer(which_timer.to_handle())
            };
        },
        {
            link: "TimerStart"
            name: timer_start
            raw: (which_timer: Handle<j_timer>, timeout: f32, periodic: bool, handler_func: CodeId);
            user: (which_timer: &Agent<j_timer>, timeout: f32, periodic: bool, handler_func: CodeId) {
                raw::timer_start(which_timer.to_handle(), timeout, periodic, handler_func)
            };
        },
        {
            link: "TimerGetElapsed"
            name: timer_get_elapsed
            raw: (which_timer: Handle<j_timer>) -> f32;
            user: (which_timer: &Agent<j_timer>) -> f32 {
                raw::timer_get_elapsed(which_timer.to_handle())
            };
        },
        {
            link: "TimerGetRemaining"
            name: timer_get_remaining
            raw: (which_timer: Handle<j_timer>) -> f32;
            user: (which_timer: &Agent<j_timer>) -> f32 {
                raw::timer_get_remaining(which_timer.to_handle())
            };
        },
        {
            link: "TimerGetTimeout"
            name: timer_get_timeout
            raw: (which_timer: Handle<j_timer>) -> f32;
            user: (which_timer: &Agent<j_timer>) -> f32 {
                raw::timer_get_timeout(which_timer.to_handle())
            };
        },
        {
            link: "PauseTimer"
            name: pause_timer
            raw: (which_timer: Handle<j_timer>);
            user: (which_timer: &Agent<j_timer>) {
                raw::pause_timer(which_timer.to_handle())
            };
        },
        {
            link: "ResumeTimer"
            name: resume_timer
            raw: (which_timer: Handle<j_timer>);
            user: (which_timer: &Agent<j_timer>) {
                raw::resume_timer(which_timer.to_handle())
            };
        },
        {
            link: "GetExpiredTimer"
            name: get_expired_timer
            raw: () -> Handle<j_timer>;
            user: () -> Option<Weak<j_timer>> {
                raw::get_expired_timer().to_weak()
            };
        },
        {
            link: "CreateGroup"
            name: create_group
            raw: () -> Handle<j_group>;
            user: () -> Option<Weak<j_group>> {
                raw::create_group().to_weak()
            };
        },
        {
            link: "DestroyGroup"
            name: destroy_group
            raw: (which_group: Handle<j_group>);
            user: (which_group: &Agent<j_group>) {
                raw::destroy_group(which_group.to_handle())
            };
        },
        {
            link: "GroupAddUnit"
            name: group_add_unit
            raw: (which_group: Handle<j_group>, which_unit: Handle<j_unit>) -> bool;
            user: (which_group: &Agent<j_group>, which_unit: &Agent<j_unit>) -> bool {
                raw::group_add_unit(which_group.to_handle(), which_unit.to_handle())
            };
        },
        {
            link: "GroupRemoveUnit"
            name: group_remove_unit
            raw: (which_group: Handle<j_group>, which_unit: Handle<j_unit>) -> bool;
            user: (which_group: &Agent<j_group>, which_unit: &Agent<j_unit>) -> bool {
                raw::group_remove_unit(which_group.to_handle(), which_unit.to_handle())
            };
        },
        {
            link: "BlzGroupAddGroupFast"
            name: blz_group_add_group_fast
            raw: (which_group: Handle<j_group>, add_group: Handle<j_group>) -> i32;
            user: (which_group: &Agent<j_group>, add_group: &Agent<j_group>) -> i32 {
                raw::blz_group_add_group_fast(which_group.to_handle(), add_group.to_handle())
            };
        },
        {
            link: "BlzGroupRemoveGroupFast"
            name: blz_group_remove_group_fast
            raw: (which_group: Handle<j_group>, remove_group: Handle<j_group>) -> i32;
            user: (which_group: &Agent<j_group>, remove_group: &Agent<j_group>) -> i32 {
                raw::blz_group_remove_group_fast(which_group.to_handle(), remove_group.to_handle())
            };
        },
        {
            link: "GroupClear"
            name: group_clear
            raw: (which_group: Handle<j_group>);
            user: (which_group: &Agent<j_group>) {
                raw::group_clear(which_group.to_handle())
            };
        },
        {
            link: "BlzGroupGetSize"
            name: blz_group_get_size
            raw: (which_group: Handle<j_group>) -> i32;
            user: (which_group: &Agent<j_group>) -> i32 {
                raw::blz_group_get_size(which_group.to_handle())
            };
        },
        {
            link: "BlzGroupUnitAt"
            name: blz_group_unit_at
            raw: (which_group: Handle<j_group>, index: i32) -> Handle<j_unit>;
            user: (which_group: &Agent<j_group>, index: i32) -> Option<Weak<j_unit>> {
                raw::blz_group_unit_at(which_group.to_handle(), index).to_weak()
            };
        },
        {
            link: "GroupEnumUnitsOfType"
            name: group_enum_units_of_type
            raw: (which_group: Handle<j_group>, unitname: *const i8, filter: Handle<j_boolexpr>);
            user: (which_group: &Agent<j_group>, unitname: &CStr, filter: &Agent<j_boolexpr>) {
                raw::group_enum_units_of_type(which_group.to_handle(), unitname.as_ptr(), filter.to_handle())
            };
        },
        {
            link: "GroupEnumUnitsOfPlayer"
            name: group_enum_units_of_player
            raw: (which_group: Handle<j_group>, which_player: Handle<j_player>, filter: Handle<j_boolexpr>);
            user: (which_group: &Agent<j_group>, which_player: &Agent<j_player>, filter: &Agent<j_boolexpr>) {
                raw::group_enum_units_of_player(which_group.to_handle(), which_player.to_handle(), filter.to_handle())
            };
        },
        {
            link: "GroupEnumUnitsOfTypeCounted"
            name: group_enum_units_of_type_counted
            raw: (which_group: Handle<j_group>, unitname: *const i8, filter: Handle<j_boolexpr>, count_limit: i32);
            user: (which_group: &Agent<j_group>, unitname: &CStr, filter: &Agent<j_boolexpr>, count_limit: i32) {
                raw::group_enum_units_of_type_counted(which_group.to_handle(), unitname.as_ptr(), filter.to_handle(), count_limit)
            };
        },
        {
            link: "GroupEnumUnitsInRect"
            name: group_enum_units_in_rect
            raw: (which_group: Handle<j_group>, r: Handle<j_rect>, filter: Handle<j_boolexpr>);
            user: (which_group: &Agent<j_group>, r: &Agent<j_rect>, filter: &Agent<j_boolexpr>) {
                raw::group_enum_units_in_rect(which_group.to_handle(), r.to_handle(), filter.to_handle())
            };
        },
        {
            link: "GroupEnumUnitsInRectCounted"
            name: group_enum_units_in_rect_counted
            raw: (which_group: Handle<j_group>, r: Handle<j_rect>, filter: Handle<j_boolexpr>, count_limit: i32);
            user: (which_group: &Agent<j_group>, r: &Agent<j_rect>, filter: &Agent<j_boolexpr>, count_limit: i32) {
                raw::group_enum_units_in_rect_counted(which_group.to_handle(), r.to_handle(), filter.to_handle(), count_limit)
            };
        },
        {
            link: "GroupEnumUnitsInRange"
            name: group_enum_units_in_range
            raw: (which_group: Handle<j_group>, x: f32, y: f32, radius: f32, filter: Handle<j_boolexpr>);
            user: (which_group: &Agent<j_group>, x: f32, y: f32, radius: f32, filter: &Agent<j_boolexpr>) {
                raw::group_enum_units_in_range(which_group.to_handle(), x, y, radius, filter.to_handle())
            };
        },
        {
            link: "GroupEnumUnitsInRangeOfLoc"
            name: group_enum_units_in_range_of_loc
            raw: (which_group: Handle<j_group>, which_location: Handle<j_location>, radius: f32, filter: Handle<j_boolexpr>);
            user: (which_group: &Agent<j_group>, which_location: &Agent<j_location>, radius: f32, filter: &Agent<j_boolexpr>) {
                raw::group_enum_units_in_range_of_loc(which_group.to_handle(), which_location.to_handle(), radius, filter.to_handle())
            };
        },
        {
            link: "GroupEnumUnitsInRangeCounted"
            name: group_enum_units_in_range_counted
            raw: (which_group: Handle<j_group>, x: f32, y: f32, radius: f32, filter: Handle<j_boolexpr>, count_limit: i32);
            user: (which_group: &Agent<j_group>, x: f32, y: f32, radius: f32, filter: &Agent<j_boolexpr>, count_limit: i32) {
                raw::group_enum_units_in_range_counted(which_group.to_handle(), x, y, radius, filter.to_handle(), count_limit)
            };
        },
        {
            link: "GroupEnumUnitsInRangeOfLocCounted"
            name: group_enum_units_in_range_of_loc_counted
            raw: (which_group: Handle<j_group>, which_location: Handle<j_location>, radius: f32, filter: Handle<j_boolexpr>, count_limit: i32);
            user: (which_group: &Agent<j_group>, which_location: &Agent<j_location>, radius: f32, filter: &Agent<j_boolexpr>, count_limit: i32) {
                raw::group_enum_units_in_range_of_loc_counted(which_group.to_handle(), which_location.to_handle(), radius, filter.to_handle(), count_limit)
            };
        },
        {
            link: "GroupEnumUnitsSelected"
            name: group_enum_units_selected
            raw: (which_group: Handle<j_group>, which_player: Handle<j_player>, filter: Handle<j_boolexpr>);
            user: (which_group: &Agent<j_group>, which_player: &Agent<j_player>, filter: &Agent<j_boolexpr>) {
                raw::group_enum_units_selected(which_group.to_handle(), which_player.to_handle(), filter.to_handle())
            };
        },
        {
            link: "GroupImmediateOrder"
            name: group_immediate_order
            raw: (which_group: Handle<j_group>, order: *const i8) -> bool;
            user: (which_group: &Agent<j_group>, order: &CStr) -> bool {
                raw::group_immediate_order(which_group.to_handle(), order.as_ptr())
            };
        },
        {
            link: "GroupImmediateOrderById"
            name: group_immediate_order_by_id
            raw: (which_group: Handle<j_group>, order: i32) -> bool;
            user: (which_group: &Agent<j_group>, order: i32) -> bool {
                raw::group_immediate_order_by_id(which_group.to_handle(), order)
            };
        },
        {
            link: "GroupPointOrder"
            name: group_point_order
            raw: (which_group: Handle<j_group>, order: *const i8, x: f32, y: f32) -> bool;
            user: (which_group: &Agent<j_group>, order: &CStr, x: f32, y: f32) -> bool {
                raw::group_point_order(which_group.to_handle(), order.as_ptr(), x, y)
            };
        },
        {
            link: "GroupPointOrderLoc"
            name: group_point_order_loc
            raw: (which_group: Handle<j_group>, order: *const i8, which_location: Handle<j_location>) -> bool;
            user: (which_group: &Agent<j_group>, order: &CStr, which_location: &Agent<j_location>) -> bool {
                raw::group_point_order_loc(which_group.to_handle(), order.as_ptr(), which_location.to_handle())
            };
        },
        {
            link: "GroupPointOrderById"
            name: group_point_order_by_id
            raw: (which_group: Handle<j_group>, order: i32, x: f32, y: f32) -> bool;
            user: (which_group: &Agent<j_group>, order: i32, x: f32, y: f32) -> bool {
                raw::group_point_order_by_id(which_group.to_handle(), order, x, y)
            };
        },
        {
            link: "GroupPointOrderByIdLoc"
            name: group_point_order_by_id_loc
            raw: (which_group: Handle<j_group>, order: i32, which_location: Handle<j_location>) -> bool;
            user: (which_group: &Agent<j_group>, order: i32, which_location: &Agent<j_location>) -> bool {
                raw::group_point_order_by_id_loc(which_group.to_handle(), order, which_location.to_handle())
            };
        },
        {
            link: "GroupTargetOrder"
            name: group_target_order
            raw: (which_group: Handle<j_group>, order: *const i8, target_widget: Handle<j_widget>) -> bool;
            user: (which_group: &Agent<j_group>, order: &CStr, target_widget: &Agent<j_widget>) -> bool {
                raw::group_target_order(which_group.to_handle(), order.as_ptr(), target_widget.to_handle())
            };
        },
        {
            link: "GroupTargetOrderById"
            name: group_target_order_by_id
            raw: (which_group: Handle<j_group>, order: i32, target_widget: Handle<j_widget>) -> bool;
            user: (which_group: &Agent<j_group>, order: i32, target_widget: &Agent<j_widget>) -> bool {
                raw::group_target_order_by_id(which_group.to_handle(), order, target_widget.to_handle())
            };
        },
        {
            link: "ForGroup"
            name: for_group
            raw: (which_group: Handle<j_group>, callback: CodeId);
            user: (which_group: &Agent<j_group>, callback: CodeId) {
                raw::for_group(which_group.to_handle(), callback)
            };
        },
        {
            link: "FirstOfGroup"
            name: first_of_group
            raw: (which_group: Handle<j_group>) -> Handle<j_unit>;
            user: (which_group: &Agent<j_group>) -> Option<Weak<j_unit>> {
                raw::first_of_group(which_group.to_handle()).to_weak()
            };
        },
        {
            link: "CreateForce"
            name: create_force
            raw: () -> Handle<j_force>;
            user: () -> Option<Weak<j_force>> {
                raw::create_force().to_weak()
            };
        },
        {
            link: "DestroyForce"
            name: destroy_force
            raw: (which_force: Handle<j_force>);
            user: (which_force: &Agent<j_force>) {
                raw::destroy_force(which_force.to_handle())
            };
        },
        {
            link: "ForceAddPlayer"
            name: force_add_player
            raw: (which_force: Handle<j_force>, which_player: Handle<j_player>);
            user: (which_force: &Agent<j_force>, which_player: &Agent<j_player>) {
                raw::force_add_player(which_force.to_handle(), which_player.to_handle())
            };
        },
        {
            link: "ForceRemovePlayer"
            name: force_remove_player
            raw: (which_force: Handle<j_force>, which_player: Handle<j_player>);
            user: (which_force: &Agent<j_force>, which_player: &Agent<j_player>) {
                raw::force_remove_player(which_force.to_handle(), which_player.to_handle())
            };
        },
        {
            link: "BlzForceHasPlayer"
            name: blz_force_has_player
            raw: (which_force: Handle<j_force>, which_player: Handle<j_player>) -> bool;
            user: (which_force: &Agent<j_force>, which_player: &Agent<j_player>) -> bool {
                raw::blz_force_has_player(which_force.to_handle(), which_player.to_handle())
            };
        },
        {
            link: "ForceClear"
            name: force_clear
            raw: (which_force: Handle<j_force>);
            user: (which_force: &Agent<j_force>) {
                raw::force_clear(which_force.to_handle())
            };
        },
        {
            link: "ForceEnumPlayers"
            name: force_enum_players
            raw: (which_force: Handle<j_force>, filter: Handle<j_boolexpr>);
            user: (which_force: &Agent<j_force>, filter: &Agent<j_boolexpr>) {
                raw::force_enum_players(which_force.to_handle(), filter.to_handle())
            };
        },
        {
            link: "ForceEnumPlayersCounted"
            name: force_enum_players_counted
            raw: (which_force: Handle<j_force>, filter: Handle<j_boolexpr>, count_limit: i32);
            user: (which_force: &Agent<j_force>, filter: &Agent<j_boolexpr>, count_limit: i32) {
                raw::force_enum_players_counted(which_force.to_handle(), filter.to_handle(), count_limit)
            };
        },
        {
            link: "ForceEnumAllies"
            name: force_enum_allies
            raw: (which_force: Handle<j_force>, which_player: Handle<j_player>, filter: Handle<j_boolexpr>);
            user: (which_force: &Agent<j_force>, which_player: &Agent<j_player>, filter: &Agent<j_boolexpr>) {
                raw::force_enum_allies(which_force.to_handle(), which_player.to_handle(), filter.to_handle())
            };
        },
        {
            link: "ForceEnumEnemies"
            name: force_enum_enemies
            raw: (which_force: Handle<j_force>, which_player: Handle<j_player>, filter: Handle<j_boolexpr>);
            user: (which_force: &Agent<j_force>, which_player: &Agent<j_player>, filter: &Agent<j_boolexpr>) {
                raw::force_enum_enemies(which_force.to_handle(), which_player.to_handle(), filter.to_handle())
            };
        },
        {
            link: "ForForce"
            name: for_force
            raw: (which_force: Handle<j_force>, callback: CodeId);
            user: (which_force: &Agent<j_force>, callback: CodeId) {
                raw::for_force(which_force.to_handle(), callback)
            };
        },
        {
            link: "Rect"
            name: rect
            raw: (minx: f32, miny: f32, maxx: f32, maxy: f32) -> Handle<j_rect>;
            user: (minx: f32, miny: f32, maxx: f32, maxy: f32) -> Option<Weak<j_rect>> {
                raw::rect(minx, miny, maxx, maxy).to_weak()
            };
        },
        {
            link: "RectFromLoc"
            name: rect_from_loc
            raw: (min: Handle<j_location>, max: Handle<j_location>) -> Handle<j_rect>;
            user: (min: &Agent<j_location>, max: &Agent<j_location>) -> Option<Weak<j_rect>> {
                raw::rect_from_loc(min.to_handle(), max.to_handle()).to_weak()
            };
        },
        {
            link: "RemoveRect"
            name: remove_rect
            raw: (which_rect: Handle<j_rect>);
            user: (which_rect: &Agent<j_rect>) {
                raw::remove_rect(which_rect.to_handle())
            };
        },
        {
            link: "SetRect"
            name: set_rect
            raw: (which_rect: Handle<j_rect>, minx: f32, miny: f32, maxx: f32, maxy: f32);
            user: (which_rect: &Agent<j_rect>, minx: f32, miny: f32, maxx: f32, maxy: f32) {
                raw::set_rect(which_rect.to_handle(), minx, miny, maxx, maxy)
            };
        },
        {
            link: "SetRectFromLoc"
            name: set_rect_from_loc
            raw: (which_rect: Handle<j_rect>, min: Handle<j_location>, max: Handle<j_location>);
            user: (which_rect: &Agent<j_rect>, min: &Agent<j_location>, max: &Agent<j_location>) {
                raw::set_rect_from_loc(which_rect.to_handle(), min.to_handle(), max.to_handle())
            };
        },
        {
            link: "MoveRectTo"
            name: move_rect_to
            raw: (which_rect: Handle<j_rect>, new_center_x: f32, new_center_y: f32);
            user: (which_rect: &Agent<j_rect>, new_center_x: f32, new_center_y: f32) {
                raw::move_rect_to(which_rect.to_handle(), new_center_x, new_center_y)
            };
        },
        {
            link: "MoveRectToLoc"
            name: move_rect_to_loc
            raw: (which_rect: Handle<j_rect>, new_center_loc: Handle<j_location>);
            user: (which_rect: &Agent<j_rect>, new_center_loc: &Agent<j_location>) {
                raw::move_rect_to_loc(which_rect.to_handle(), new_center_loc.to_handle())
            };
        },
        {
            link: "GetRectCenterX"
            name: get_rect_center_x
            raw: (which_rect: Handle<j_rect>) -> f32;
            user: (which_rect: &Agent<j_rect>) -> f32 {
                raw::get_rect_center_x(which_rect.to_handle())
            };
        },
        {
            link: "GetRectCenterY"
            name: get_rect_center_y
            raw: (which_rect: Handle<j_rect>) -> f32;
            user: (which_rect: &Agent<j_rect>) -> f32 {
                raw::get_rect_center_y(which_rect.to_handle())
            };
        },
        {
            link: "GetRectMinX"
            name: get_rect_min_x
            raw: (which_rect: Handle<j_rect>) -> f32;
            user: (which_rect: &Agent<j_rect>) -> f32 {
                raw::get_rect_min_x(which_rect.to_handle())
            };
        },
        {
            link: "GetRectMinY"
            name: get_rect_min_y
            raw: (which_rect: Handle<j_rect>) -> f32;
            user: (which_rect: &Agent<j_rect>) -> f32 {
                raw::get_rect_min_y(which_rect.to_handle())
            };
        },
        {
            link: "GetRectMaxX"
            name: get_rect_max_x
            raw: (which_rect: Handle<j_rect>) -> f32;
            user: (which_rect: &Agent<j_rect>) -> f32 {
                raw::get_rect_max_x(which_rect.to_handle())
            };
        },
        {
            link: "GetRectMaxY"
            name: get_rect_max_y
            raw: (which_rect: Handle<j_rect>) -> f32;
            user: (which_rect: &Agent<j_rect>) -> f32 {
                raw::get_rect_max_y(which_rect.to_handle())
            };
        },
        {
            link: "CreateRegion"
            name: create_region
            raw: () -> Handle<j_region>;
            user: () -> Option<Weak<j_region>> {
                raw::create_region().to_weak()
            };
        },
        {
            link: "RemoveRegion"
            name: remove_region
            raw: (which_region: Handle<j_region>);
            user: (which_region: &Agent<j_region>) {
                raw::remove_region(which_region.to_handle())
            };
        },
        {
            link: "RegionAddRect"
            name: region_add_rect
            raw: (which_region: Handle<j_region>, r: Handle<j_rect>);
            user: (which_region: &Agent<j_region>, r: &Agent<j_rect>) {
                raw::region_add_rect(which_region.to_handle(), r.to_handle())
            };
        },
        {
            link: "RegionClearRect"
            name: region_clear_rect
            raw: (which_region: Handle<j_region>, r: Handle<j_rect>);
            user: (which_region: &Agent<j_region>, r: &Agent<j_rect>) {
                raw::region_clear_rect(which_region.to_handle(), r.to_handle())
            };
        },
        {
            link: "RegionAddCell"
            name: region_add_cell
            raw: (which_region: Handle<j_region>, x: f32, y: f32);
            user: (which_region: &Agent<j_region>, x: f32, y: f32) {
                raw::region_add_cell(which_region.to_handle(), x, y)
            };
        },
        {
            link: "RegionAddCellAtLoc"
            name: region_add_cell_at_loc
            raw: (which_region: Handle<j_region>, which_location: Handle<j_location>);
            user: (which_region: &Agent<j_region>, which_location: &Agent<j_location>) {
                raw::region_add_cell_at_loc(which_region.to_handle(), which_location.to_handle())
            };
        },
        {
            link: "RegionClearCell"
            name: region_clear_cell
            raw: (which_region: Handle<j_region>, x: f32, y: f32);
            user: (which_region: &Agent<j_region>, x: f32, y: f32) {
                raw::region_clear_cell(which_region.to_handle(), x, y)
            };
        },
        {
            link: "RegionClearCellAtLoc"
            name: region_clear_cell_at_loc
            raw: (which_region: Handle<j_region>, which_location: Handle<j_location>);
            user: (which_region: &Agent<j_region>, which_location: &Agent<j_location>) {
                raw::region_clear_cell_at_loc(which_region.to_handle(), which_location.to_handle())
            };
        },
        {
            link: "Location"
            name: location
            raw: (x: f32, y: f32) -> Handle<j_location>;
            user: (x: f32, y: f32) -> Option<Weak<j_location>> {
                raw::location(x, y).to_weak()
            };
        },
        {
            link: "RemoveLocation"
            name: remove_location
            raw: (which_location: Handle<j_location>);
            user: (which_location: &Agent<j_location>) {
                raw::remove_location(which_location.to_handle())
            };
        },
        {
            link: "MoveLocation"
            name: move_location
            raw: (which_location: Handle<j_location>, new_x: f32, new_y: f32);
            user: (which_location: &Agent<j_location>, new_x: f32, new_y: f32) {
                raw::move_location(which_location.to_handle(), new_x, new_y)
            };
        },
        {
            link: "GetLocationX"
            name: get_location_x
            raw: (which_location: Handle<j_location>) -> f32;
            user: (which_location: &Agent<j_location>) -> f32 {
                raw::get_location_x(which_location.to_handle())
            };
        },
        {
            link: "GetLocationY"
            name: get_location_y
            raw: (which_location: Handle<j_location>) -> f32;
            user: (which_location: &Agent<j_location>) -> f32 {
                raw::get_location_y(which_location.to_handle())
            };
        },
        {
            link: "GetLocationZ"
            name: get_location_z
            raw: (which_location: Handle<j_location>) -> f32;
            user: (which_location: &Agent<j_location>) -> f32 {
                raw::get_location_z(which_location.to_handle())
            };
        },
        {
            link: "IsUnitInRegion"
            name: is_unit_in_region
            raw: (which_region: Handle<j_region>, which_unit: Handle<j_unit>) -> bool;
            user: (which_region: &Agent<j_region>, which_unit: &Agent<j_unit>) -> bool {
                raw::is_unit_in_region(which_region.to_handle(), which_unit.to_handle())
            };
        },
        {
            link: "IsPointInRegion"
            name: is_point_in_region
            raw: (which_region: Handle<j_region>, x: f32, y: f32) -> bool;
            user: (which_region: &Agent<j_region>, x: f32, y: f32) -> bool {
                raw::is_point_in_region(which_region.to_handle(), x, y)
            };
        },
        {
            link: "IsLocationInRegion"
            name: is_location_in_region
            raw: (which_region: Handle<j_region>, which_location: Handle<j_location>) -> bool;
            user: (which_region: &Agent<j_region>, which_location: &Agent<j_location>) -> bool {
                raw::is_location_in_region(which_region.to_handle(), which_location.to_handle())
            };
        },
        {
            link: "GetWorldBounds"
            name: get_world_bounds
            raw: () -> Handle<j_rect>;
            user: () -> Option<Weak<j_rect>> {
                raw::get_world_bounds().to_weak()
            };
        },
        {
            link: "CreateTrigger"
            name: create_trigger
            raw: () -> Handle<j_trigger>;
            user: () -> Option<Weak<j_trigger>> {
                raw::create_trigger().to_weak()
            };
        },
        {
            link: "DestroyTrigger"
            name: destroy_trigger
            raw: (which_trigger: Handle<j_trigger>);
            user: (which_trigger: &Agent<j_trigger>) {
                raw::destroy_trigger(which_trigger.to_handle())
            };
        },
        {
            link: "ResetTrigger"
            name: reset_trigger
            raw: (which_trigger: Handle<j_trigger>);
            user: (which_trigger: &Agent<j_trigger>) {
                raw::reset_trigger(which_trigger.to_handle())
            };
        },
        {
            link: "EnableTrigger"
            name: enable_trigger
            raw: (which_trigger: Handle<j_trigger>);
            user: (which_trigger: &Agent<j_trigger>) {
                raw::enable_trigger(which_trigger.to_handle())
            };
        },
        {
            link: "DisableTrigger"
            name: disable_trigger
            raw: (which_trigger: Handle<j_trigger>);
            user: (which_trigger: &Agent<j_trigger>) {
                raw::disable_trigger(which_trigger.to_handle())
            };
        },
        {
            link: "IsTriggerEnabled"
            name: is_trigger_enabled
            raw: (which_trigger: Handle<j_trigger>) -> bool;
            user: (which_trigger: &Agent<j_trigger>) -> bool {
                raw::is_trigger_enabled(which_trigger.to_handle())
            };
        },
        {
            link: "TriggerWaitOnSleeps"
            name: trigger_wait_on_sleeps
            raw: (which_trigger: Handle<j_trigger>, flag: bool);
            user: (which_trigger: &Agent<j_trigger>, flag: bool) {
                raw::trigger_wait_on_sleeps(which_trigger.to_handle(), flag)
            };
        },
        {
            link: "IsTriggerWaitOnSleeps"
            name: is_trigger_wait_on_sleeps
            raw: (which_trigger: Handle<j_trigger>) -> bool;
            user: (which_trigger: &Agent<j_trigger>) -> bool {
                raw::is_trigger_wait_on_sleeps(which_trigger.to_handle())
            };
        },
        {
            link: "GetFilterUnit"
            name: get_filter_unit
            raw: () -> Handle<j_unit>;
            user: () -> Option<Weak<j_unit>> {
                raw::get_filter_unit().to_weak()
            };
        },
        {
            link: "GetEnumUnit"
            name: get_enum_unit
            raw: () -> Handle<j_unit>;
            user: () -> Option<Weak<j_unit>> {
                raw::get_enum_unit().to_weak()
            };
        },
        {
            link: "GetFilterDestructable"
            name: get_filter_destructable
            raw: () -> Handle<j_destructable>;
            user: () -> Option<Weak<j_destructable>> {
                raw::get_filter_destructable().to_weak()
            };
        },
        {
            link: "GetEnumDestructable"
            name: get_enum_destructable
            raw: () -> Handle<j_destructable>;
            user: () -> Option<Weak<j_destructable>> {
                raw::get_enum_destructable().to_weak()
            };
        },
        {
            link: "GetFilterItem"
            name: get_filter_item
            raw: () -> Handle<j_item>;
            user: () -> Option<Weak<j_item>> {
                raw::get_filter_item().to_weak()
            };
        },
        {
            link: "GetEnumItem"
            name: get_enum_item
            raw: () -> Handle<j_item>;
            user: () -> Option<Weak<j_item>> {
                raw::get_enum_item().to_weak()
            };
        },
        {
            link: "ParseTags"
            name: parse_tags
            raw: (tagged_string: *const i8, out: *mut i8, out_len: *mut usize);
            user: (tagged_string: &CStr) -> String {
                raw::parse_tags(tagged_string.as_ptr(), stringret::ptr(), stringret::reset_len()); stringret::read_string()
            };
        },
        {
            link: "GetFilterPlayer"
            name: get_filter_player
            raw: () -> Handle<j_player>;
            user: () -> Option<Weak<j_player>> {
                raw::get_filter_player().to_weak()
            };
        },
        {
            link: "GetEnumPlayer"
            name: get_enum_player
            raw: () -> Handle<j_player>;
            user: () -> Option<Weak<j_player>> {
                raw::get_enum_player().to_weak()
            };
        },
        {
            link: "GetTriggeringTrigger"
            name: get_triggering_trigger
            raw: () -> Handle<j_trigger>;
            user: () -> Option<Weak<j_trigger>> {
                raw::get_triggering_trigger().to_weak()
            };
        },
        {
            link: "GetTriggerEventId"
            name: get_trigger_event_id
            raw: () -> Handle<j_eventid>;
            user: () -> Handle<j_eventid> {
                raw::get_trigger_event_id()
            };
        },
        {
            link: "GetTriggerEvalCount"
            name: get_trigger_eval_count
            raw: (which_trigger: Handle<j_trigger>) -> i32;
            user: (which_trigger: &Agent<j_trigger>) -> i32 {
                raw::get_trigger_eval_count(which_trigger.to_handle())
            };
        },
        {
            link: "GetTriggerExecCount"
            name: get_trigger_exec_count
            raw: (which_trigger: Handle<j_trigger>) -> i32;
            user: (which_trigger: &Agent<j_trigger>) -> i32 {
                raw::get_trigger_exec_count(which_trigger.to_handle())
            };
        },
        {
            link: "ExecuteFunc"
            name: execute_func
            raw: (func_name: *const i8);
            user: (func_name: &CStr) {
                raw::execute_func(func_name.as_ptr())
            };
        },
        {
            link: "And"
            name: and
            raw: (operand_a: Handle<j_boolexpr>, operand_b: Handle<j_boolexpr>) -> Handle<j_boolexpr>;
            user: (operand_a: &Agent<j_boolexpr>, operand_b: &Agent<j_boolexpr>) -> Option<Weak<j_boolexpr>> {
                raw::and(operand_a.to_handle(), operand_b.to_handle()).to_weak()
            };
        },
        {
            link: "Or"
            name: or
            raw: (operand_a: Handle<j_boolexpr>, operand_b: Handle<j_boolexpr>) -> Handle<j_boolexpr>;
            user: (operand_a: &Agent<j_boolexpr>, operand_b: &Agent<j_boolexpr>) -> Option<Weak<j_boolexpr>> {
                raw::or(operand_a.to_handle(), operand_b.to_handle()).to_weak()
            };
        },
        {
            link: "Not"
            name: not
            raw: (operand: Handle<j_boolexpr>) -> Handle<j_boolexpr>;
            user: (operand: &Agent<j_boolexpr>) -> Option<Weak<j_boolexpr>> {
                raw::not(operand.to_handle()).to_weak()
            };
        },
        {
            link: "Condition"
            name: condition
            raw: (func: CodeId) -> Handle<j_conditionfunc>;
            user: (func: CodeId) -> Option<Weak<j_conditionfunc>> {
                raw::condition(func).to_weak()
            };
        },
        {
            link: "DestroyCondition"
            name: destroy_condition
            raw: (c: Handle<j_conditionfunc>);
            user: (c: &Agent<j_conditionfunc>) {
                raw::destroy_condition(c.to_handle())
            };
        },
        {
            link: "Filter"
            name: filter
            raw: (func: CodeId) -> Handle<j_filterfunc>;
            user: (func: CodeId) -> Option<Weak<j_filterfunc>> {
                raw::filter(func).to_weak()
            };
        },
        {
            link: "DestroyFilter"
            name: destroy_filter
            raw: (f: Handle<j_filterfunc>);
            user: (f: &Agent<j_filterfunc>) {
                raw::destroy_filter(f.to_handle())
            };
        },
        {
            link: "DestroyBoolExpr"
            name: destroy_bool_expr
            raw: (e: Handle<j_boolexpr>);
            user: (e: &Agent<j_boolexpr>) {
                raw::destroy_bool_expr(e.to_handle())
            };
        },
        {
            link: "TriggerRegisterVariableEvent"
            name: trigger_register_variable_event
            raw: (which_trigger: Handle<j_trigger>, var_name: *const i8, opcode: i32, limitval: f32) -> Handle<j_event>;
            user: (which_trigger: &Agent<j_trigger>, var_name: &CStr, opcode: i32, limitval: f32) -> Option<Weak<j_event>> {
                raw::trigger_register_variable_event(which_trigger.to_handle(), var_name.as_ptr(), opcode, limitval).to_weak()
            };
        },
        {
            link: "TriggerRegisterTimerEvent"
            name: trigger_register_timer_event
            raw: (which_trigger: Handle<j_trigger>, timeout: f32, periodic: bool) -> Handle<j_event>;
            user: (which_trigger: &Agent<j_trigger>, timeout: f32, periodic: bool) -> Option<Weak<j_event>> {
                raw::trigger_register_timer_event(which_trigger.to_handle(), timeout, periodic).to_weak()
            };
        },
        {
            link: "TriggerRegisterTimerExpireEvent"
            name: trigger_register_timer_expire_event
            raw: (which_trigger: Handle<j_trigger>, t: Handle<j_timer>) -> Handle<j_event>;
            user: (which_trigger: &Agent<j_trigger>, t: &Agent<j_timer>) -> Option<Weak<j_event>> {
                raw::trigger_register_timer_expire_event(which_trigger.to_handle(), t.to_handle()).to_weak()
            };
        },
        {
            link: "TriggerRegisterGameStateEvent"
            name: trigger_register_game_state_event
            raw: (which_trigger: Handle<j_trigger>, which_state: Handle<j_gamestate>, opcode: i32, limitval: f32) -> Handle<j_event>;
            user: (which_trigger: &Agent<j_trigger>, which_state: Handle<j_gamestate>, opcode: i32, limitval: f32) -> Option<Weak<j_event>> {
                raw::trigger_register_game_state_event(which_trigger.to_handle(), which_state, opcode, limitval).to_weak()
            };
        },
        {
            link: "TriggerRegisterDialogEvent"
            name: trigger_register_dialog_event
            raw: (which_trigger: Handle<j_trigger>, which_dialog: Handle<j_dialog>) -> Handle<j_event>;
            user: (which_trigger: &Agent<j_trigger>, which_dialog: &Agent<j_dialog>) -> Option<Weak<j_event>> {
                raw::trigger_register_dialog_event(which_trigger.to_handle(), which_dialog.to_handle()).to_weak()
            };
        },
        {
            link: "TriggerRegisterDialogButtonEvent"
            name: trigger_register_dialog_button_event
            raw: (which_trigger: Handle<j_trigger>, which_button: Handle<j_button>) -> Handle<j_event>;
            user: (which_trigger: &Agent<j_trigger>, which_button: &Agent<j_button>) -> Option<Weak<j_event>> {
                raw::trigger_register_dialog_button_event(which_trigger.to_handle(), which_button.to_handle()).to_weak()
            };
        },
        {
            link: "GetEventGameState"
            name: get_event_game_state
            raw: () -> Handle<j_gamestate>;
            user: () -> Handle<j_gamestate> {
                raw::get_event_game_state()
            };
        },
        {
            link: "TriggerRegisterGameEvent"
            name: trigger_register_game_event
            raw: (which_trigger: Handle<j_trigger>, which_game_event: i32) -> Handle<j_event>;
            user: (which_trigger: &Agent<j_trigger>, which_game_event: i32) -> Option<Weak<j_event>> {
                raw::trigger_register_game_event(which_trigger.to_handle(), which_game_event).to_weak()
            };
        },
        {
            link: "GetWinningPlayer"
            name: get_winning_player
            raw: () -> Handle<j_player>;
            user: () -> Option<Weak<j_player>> {
                raw::get_winning_player().to_weak()
            };
        },
        {
            link: "TriggerRegisterEnterRegion"
            name: trigger_register_enter_region
            raw: (which_trigger: Handle<j_trigger>, which_region: Handle<j_region>, filter: Handle<j_boolexpr>) -> Handle<j_event>;
            user: (which_trigger: &Agent<j_trigger>, which_region: &Agent<j_region>, filter: &Agent<j_boolexpr>) -> Option<Weak<j_event>> {
                raw::trigger_register_enter_region(which_trigger.to_handle(), which_region.to_handle(), filter.to_handle()).to_weak()
            };
        },
        {
            link: "GetTriggeringRegion"
            name: get_triggering_region
            raw: () -> Handle<j_region>;
            user: () -> Option<Weak<j_region>> {
                raw::get_triggering_region().to_weak()
            };
        },
        {
            link: "GetEnteringUnit"
            name: get_entering_unit
            raw: () -> Handle<j_unit>;
            user: () -> Option<Weak<j_unit>> {
                raw::get_entering_unit().to_weak()
            };
        },
        {
            link: "TriggerRegisterLeaveRegion"
            name: trigger_register_leave_region
            raw: (which_trigger: Handle<j_trigger>, which_region: Handle<j_region>, filter: Handle<j_boolexpr>) -> Handle<j_event>;
            user: (which_trigger: &Agent<j_trigger>, which_region: &Agent<j_region>, filter: &Agent<j_boolexpr>) -> Option<Weak<j_event>> {
                raw::trigger_register_leave_region(which_trigger.to_handle(), which_region.to_handle(), filter.to_handle()).to_weak()
            };
        },
        {
            link: "GetLeavingUnit"
            name: get_leaving_unit
            raw: () -> Handle<j_unit>;
            user: () -> Option<Weak<j_unit>> {
                raw::get_leaving_unit().to_weak()
            };
        },
        {
            link: "TriggerRegisterTrackableHitEvent"
            name: trigger_register_trackable_hit_event
            raw: (which_trigger: Handle<j_trigger>, t: Handle<j_trackable>) -> Handle<j_event>;
            user: (which_trigger: &Agent<j_trigger>, t: &Agent<j_trackable>) -> Option<Weak<j_event>> {
                raw::trigger_register_trackable_hit_event(which_trigger.to_handle(), t.to_handle()).to_weak()
            };
        },
        {
            link: "TriggerRegisterTrackableTrackEvent"
            name: trigger_register_trackable_track_event
            raw: (which_trigger: Handle<j_trigger>, t: Handle<j_trackable>) -> Handle<j_event>;
            user: (which_trigger: &Agent<j_trigger>, t: &Agent<j_trackable>) -> Option<Weak<j_event>> {
                raw::trigger_register_trackable_track_event(which_trigger.to_handle(), t.to_handle()).to_weak()
            };
        },
        {
            link: "TriggerRegisterCommandEvent"
            name: trigger_register_command_event
            raw: (which_trigger: Handle<j_trigger>, which_ability: i32, order: *const i8) -> Handle<j_event>;
            user: (which_trigger: &Agent<j_trigger>, which_ability: i32, order: &CStr) -> Option<Weak<j_event>> {
                raw::trigger_register_command_event(which_trigger.to_handle(), which_ability, order.as_ptr()).to_weak()
            };
        },
        {
            link: "TriggerRegisterUpgradeCommandEvent"
            name: trigger_register_upgrade_command_event
            raw: (which_trigger: Handle<j_trigger>, which_upgrade: i32) -> Handle<j_event>;
            user: (which_trigger: &Agent<j_trigger>, which_upgrade: i32) -> Option<Weak<j_event>> {
                raw::trigger_register_upgrade_command_event(which_trigger.to_handle(), which_upgrade).to_weak()
            };
        },
        {
            link: "GetTriggeringTrackable"
            name: get_triggering_trackable
            raw: () -> Handle<j_trackable>;
            user: () -> Option<Weak<j_trackable>> {
                raw::get_triggering_trackable().to_weak()
            };
        },
        {
            link: "GetClickedButton"
            name: get_clicked_button
            raw: () -> Handle<j_button>;
            user: () -> Option<Weak<j_button>> {
                raw::get_clicked_button().to_weak()
            };
        },
        {
            link: "GetClickedDialog"
            name: get_clicked_dialog
            raw: () -> Handle<j_dialog>;
            user: () -> Option<Weak<j_dialog>> {
                raw::get_clicked_dialog().to_weak()
            };
        },
        {
            link: "GetTournamentFinishSoonTimeRemaining"
            name: get_tournament_finish_soon_time_remaining
            raw: () -> f32;
            user: () -> f32 {
                raw::get_tournament_finish_soon_time_remaining()
            };
        },
        {
            link: "GetTournamentFinishNowRule"
            name: get_tournament_finish_now_rule
            raw: () -> i32;
            user: () -> i32 {
                raw::get_tournament_finish_now_rule()
            };
        },
        {
            link: "GetTournamentFinishNowPlayer"
            name: get_tournament_finish_now_player
            raw: () -> Handle<j_player>;
            user: () -> Option<Weak<j_player>> {
                raw::get_tournament_finish_now_player().to_weak()
            };
        },
        {
            link: "GetTournamentScore"
            name: get_tournament_score
            raw: (which_player: Handle<j_player>) -> i32;
            user: (which_player: &Agent<j_player>) -> i32 {
                raw::get_tournament_score(which_player.to_handle())
            };
        },
        {
            link: "GetSaveBasicFilename"
            name: get_save_basic_filename
            raw: (out: *mut i8, out_len: *mut usize);
            user: () -> String {
                raw::get_save_basic_filename(stringret::ptr(), stringret::reset_len()); stringret::read_string()
            };
        },
        {
            link: "TriggerRegisterPlayerEvent"
            name: trigger_register_player_event
            raw: (which_trigger: Handle<j_trigger>, which_player: Handle<j_player>, which_player_event: i32) -> Handle<j_event>;
            user: (which_trigger: &Agent<j_trigger>, which_player: &Agent<j_player>, which_player_event: i32) -> Option<Weak<j_event>> {
                raw::trigger_register_player_event(which_trigger.to_handle(), which_player.to_handle(), which_player_event).to_weak()
            };
        },
        {
            link: "GetTriggerPlayer"
            name: get_trigger_player
            raw: () -> Handle<j_player>;
            user: () -> Option<Weak<j_player>> {
                raw::get_trigger_player().to_weak()
            };
        },
        {
            link: "TriggerRegisterPlayerUnitEvent"
            name: trigger_register_player_unit_event
            raw: (which_trigger: Handle<j_trigger>, which_player: Handle<j_player>, which_player_unit_event: i32, filter: Handle<j_boolexpr>) -> Handle<j_event>;
            user: (which_trigger: &Agent<j_trigger>, which_player: &Agent<j_player>, which_player_unit_event: i32, filter: &Agent<j_boolexpr>) -> Option<Weak<j_event>> {
                raw::trigger_register_player_unit_event(which_trigger.to_handle(), which_player.to_handle(), which_player_unit_event, filter.to_handle()).to_weak()
            };
        },
        {
            link: "GetLevelingUnit"
            name: get_leveling_unit
            raw: () -> Handle<j_unit>;
            user: () -> Option<Weak<j_unit>> {
                raw::get_leveling_unit().to_weak()
            };
        },
        {
            link: "GetLearningUnit"
            name: get_learning_unit
            raw: () -> Handle<j_unit>;
            user: () -> Option<Weak<j_unit>> {
                raw::get_learning_unit().to_weak()
            };
        },
        {
            link: "GetLearnedSkill"
            name: get_learned_skill
            raw: () -> i32;
            user: () -> i32 {
                raw::get_learned_skill()
            };
        },
        {
            link: "GetLearnedSkillLevel"
            name: get_learned_skill_level
            raw: () -> i32;
            user: () -> i32 {
                raw::get_learned_skill_level()
            };
        },
        {
            link: "GetRevivableUnit"
            name: get_revivable_unit
            raw: () -> Handle<j_unit>;
            user: () -> Option<Weak<j_unit>> {
                raw::get_revivable_unit().to_weak()
            };
        },
        {
            link: "GetRevivingUnit"
            name: get_reviving_unit
            raw: () -> Handle<j_unit>;
            user: () -> Option<Weak<j_unit>> {
                raw::get_reviving_unit().to_weak()
            };
        },
        {
            link: "GetAttacker"
            name: get_attacker
            raw: () -> Handle<j_unit>;
            user: () -> Option<Weak<j_unit>> {
                raw::get_attacker().to_weak()
            };
        },
        {
            link: "GetRescuer"
            name: get_rescuer
            raw: () -> Handle<j_unit>;
            user: () -> Option<Weak<j_unit>> {
                raw::get_rescuer().to_weak()
            };
        },
        {
            link: "GetDyingUnit"
            name: get_dying_unit
            raw: () -> Handle<j_unit>;
            user: () -> Option<Weak<j_unit>> {
                raw::get_dying_unit().to_weak()
            };
        },
        {
            link: "GetKillingUnit"
            name: get_killing_unit
            raw: () -> Handle<j_unit>;
            user: () -> Option<Weak<j_unit>> {
                raw::get_killing_unit().to_weak()
            };
        },
        {
            link: "GetDecayingUnit"
            name: get_decaying_unit
            raw: () -> Handle<j_unit>;
            user: () -> Option<Weak<j_unit>> {
                raw::get_decaying_unit().to_weak()
            };
        },
        {
            link: "GetConstructingStructure"
            name: get_constructing_structure
            raw: () -> Handle<j_unit>;
            user: () -> Option<Weak<j_unit>> {
                raw::get_constructing_structure().to_weak()
            };
        },
        {
            link: "GetCancelledStructure"
            name: get_cancelled_structure
            raw: () -> Handle<j_unit>;
            user: () -> Option<Weak<j_unit>> {
                raw::get_cancelled_structure().to_weak()
            };
        },
        {
            link: "GetConstructedStructure"
            name: get_constructed_structure
            raw: () -> Handle<j_unit>;
            user: () -> Option<Weak<j_unit>> {
                raw::get_constructed_structure().to_weak()
            };
        },
        {
            link: "GetResearchingUnit"
            name: get_researching_unit
            raw: () -> Handle<j_unit>;
            user: () -> Option<Weak<j_unit>> {
                raw::get_researching_unit().to_weak()
            };
        },
        {
            link: "GetResearched"
            name: get_researched
            raw: () -> i32;
            user: () -> i32 {
                raw::get_researched()
            };
        },
        {
            link: "GetTrainedUnitType"
            name: get_trained_unit_type
            raw: () -> i32;
            user: () -> i32 {
                raw::get_trained_unit_type()
            };
        },
        {
            link: "GetTrainedUnit"
            name: get_trained_unit
            raw: () -> Handle<j_unit>;
            user: () -> Option<Weak<j_unit>> {
                raw::get_trained_unit().to_weak()
            };
        },
        {
            link: "GetDetectedUnit"
            name: get_detected_unit
            raw: () -> Handle<j_unit>;
            user: () -> Option<Weak<j_unit>> {
                raw::get_detected_unit().to_weak()
            };
        },
        {
            link: "GetSummoningUnit"
            name: get_summoning_unit
            raw: () -> Handle<j_unit>;
            user: () -> Option<Weak<j_unit>> {
                raw::get_summoning_unit().to_weak()
            };
        },
        {
            link: "GetSummonedUnit"
            name: get_summoned_unit
            raw: () -> Handle<j_unit>;
            user: () -> Option<Weak<j_unit>> {
                raw::get_summoned_unit().to_weak()
            };
        },
        {
            link: "GetTransportUnit"
            name: get_transport_unit
            raw: () -> Handle<j_unit>;
            user: () -> Option<Weak<j_unit>> {
                raw::get_transport_unit().to_weak()
            };
        },
        {
            link: "GetLoadedUnit"
            name: get_loaded_unit
            raw: () -> Handle<j_unit>;
            user: () -> Option<Weak<j_unit>> {
                raw::get_loaded_unit().to_weak()
            };
        },
        {
            link: "GetSellingUnit"
            name: get_selling_unit
            raw: () -> Handle<j_unit>;
            user: () -> Option<Weak<j_unit>> {
                raw::get_selling_unit().to_weak()
            };
        },
        {
            link: "GetSoldUnit"
            name: get_sold_unit
            raw: () -> Handle<j_unit>;
            user: () -> Option<Weak<j_unit>> {
                raw::get_sold_unit().to_weak()
            };
        },
        {
            link: "GetBuyingUnit"
            name: get_buying_unit
            raw: () -> Handle<j_unit>;
            user: () -> Option<Weak<j_unit>> {
                raw::get_buying_unit().to_weak()
            };
        },
        {
            link: "GetSoldItem"
            name: get_sold_item
            raw: () -> Handle<j_item>;
            user: () -> Option<Weak<j_item>> {
                raw::get_sold_item().to_weak()
            };
        },
        {
            link: "GetChangingUnit"
            name: get_changing_unit
            raw: () -> Handle<j_unit>;
            user: () -> Option<Weak<j_unit>> {
                raw::get_changing_unit().to_weak()
            };
        },
        {
            link: "GetChangingUnitPrevOwner"
            name: get_changing_unit_prev_owner
            raw: () -> Handle<j_player>;
            user: () -> Option<Weak<j_player>> {
                raw::get_changing_unit_prev_owner().to_weak()
            };
        },
        {
            link: "GetManipulatingUnit"
            name: get_manipulating_unit
            raw: () -> Handle<j_unit>;
            user: () -> Option<Weak<j_unit>> {
                raw::get_manipulating_unit().to_weak()
            };
        },
        {
            link: "GetManipulatedItem"
            name: get_manipulated_item
            raw: () -> Handle<j_item>;
            user: () -> Option<Weak<j_item>> {
                raw::get_manipulated_item().to_weak()
            };
        },
        {
            link: "BlzGetAbsorbingItem"
            name: blz_get_absorbing_item
            raw: () -> Handle<j_item>;
            user: () -> Option<Weak<j_item>> {
                raw::blz_get_absorbing_item().to_weak()
            };
        },
        {
            link: "BlzGetManipulatedItemWasAbsorbed"
            name: blz_get_manipulated_item_was_absorbed
            raw: () -> bool;
            user: () -> bool {
                raw::blz_get_manipulated_item_was_absorbed()
            };
        },
        {
            link: "BlzGetStackingItemSource"
            name: blz_get_stacking_item_source
            raw: () -> Handle<j_item>;
            user: () -> Option<Weak<j_item>> {
                raw::blz_get_stacking_item_source().to_weak()
            };
        },
        {
            link: "BlzGetStackingItemTarget"
            name: blz_get_stacking_item_target
            raw: () -> Handle<j_item>;
            user: () -> Option<Weak<j_item>> {
                raw::blz_get_stacking_item_target().to_weak()
            };
        },
        {
            link: "BlzGetStackingItemTargetPreviousCharges"
            name: blz_get_stacking_item_target_previous_charges
            raw: () -> i32;
            user: () -> i32 {
                raw::blz_get_stacking_item_target_previous_charges()
            };
        },
        {
            link: "GetOrderedUnit"
            name: get_ordered_unit
            raw: () -> Handle<j_unit>;
            user: () -> Option<Weak<j_unit>> {
                raw::get_ordered_unit().to_weak()
            };
        },
        {
            link: "GetIssuedOrderId"
            name: get_issued_order_id
            raw: () -> i32;
            user: () -> i32 {
                raw::get_issued_order_id()
            };
        },
        {
            link: "GetOrderPointX"
            name: get_order_point_x
            raw: () -> f32;
            user: () -> f32 {
                raw::get_order_point_x()
            };
        },
        {
            link: "GetOrderPointY"
            name: get_order_point_y
            raw: () -> f32;
            user: () -> f32 {
                raw::get_order_point_y()
            };
        },
        {
            link: "GetOrderPointLoc"
            name: get_order_point_loc
            raw: () -> Handle<j_location>;
            user: () -> Option<Weak<j_location>> {
                raw::get_order_point_loc().to_weak()
            };
        },
        {
            link: "GetOrderTarget"
            name: get_order_target
            raw: () -> Handle<j_widget>;
            user: () -> Option<Weak<j_widget>> {
                raw::get_order_target().to_weak()
            };
        },
        {
            link: "GetOrderTargetDestructable"
            name: get_order_target_destructable
            raw: () -> Handle<j_destructable>;
            user: () -> Option<Weak<j_destructable>> {
                raw::get_order_target_destructable().to_weak()
            };
        },
        {
            link: "GetOrderTargetItem"
            name: get_order_target_item
            raw: () -> Handle<j_item>;
            user: () -> Option<Weak<j_item>> {
                raw::get_order_target_item().to_weak()
            };
        },
        {
            link: "GetOrderTargetUnit"
            name: get_order_target_unit
            raw: () -> Handle<j_unit>;
            user: () -> Option<Weak<j_unit>> {
                raw::get_order_target_unit().to_weak()
            };
        },
        {
            link: "GetSpellAbilityUnit"
            name: get_spell_ability_unit
            raw: () -> Handle<j_unit>;
            user: () -> Option<Weak<j_unit>> {
                raw::get_spell_ability_unit().to_weak()
            };
        },
        {
            link: "GetSpellAbilityId"
            name: get_spell_ability_id
            raw: () -> i32;
            user: () -> i32 {
                raw::get_spell_ability_id()
            };
        },
        {
            link: "GetSpellAbility"
            name: get_spell_ability
            raw: () -> Handle<j_ability>;
            user: () -> Option<Weak<j_ability>> {
                raw::get_spell_ability().to_weak()
            };
        },
        {
            link: "GetSpellTargetLoc"
            name: get_spell_target_loc
            raw: () -> Handle<j_location>;
            user: () -> Option<Weak<j_location>> {
                raw::get_spell_target_loc().to_weak()
            };
        },
        {
            link: "GetSpellTargetX"
            name: get_spell_target_x
            raw: () -> f32;
            user: () -> f32 {
                raw::get_spell_target_x()
            };
        },
        {
            link: "GetSpellTargetY"
            name: get_spell_target_y
            raw: () -> f32;
            user: () -> f32 {
                raw::get_spell_target_y()
            };
        },
        {
            link: "GetSpellTargetDestructable"
            name: get_spell_target_destructable
            raw: () -> Handle<j_destructable>;
            user: () -> Option<Weak<j_destructable>> {
                raw::get_spell_target_destructable().to_weak()
            };
        },
        {
            link: "GetSpellTargetItem"
            name: get_spell_target_item
            raw: () -> Handle<j_item>;
            user: () -> Option<Weak<j_item>> {
                raw::get_spell_target_item().to_weak()
            };
        },
        {
            link: "GetSpellTargetUnit"
            name: get_spell_target_unit
            raw: () -> Handle<j_unit>;
            user: () -> Option<Weak<j_unit>> {
                raw::get_spell_target_unit().to_weak()
            };
        },
        {
            link: "TriggerRegisterPlayerAllianceChange"
            name: trigger_register_player_alliance_change
            raw: (which_trigger: Handle<j_trigger>, which_player: Handle<j_player>, which_alliance: i32) -> Handle<j_event>;
            user: (which_trigger: &Agent<j_trigger>, which_player: &Agent<j_player>, which_alliance: i32) -> Option<Weak<j_event>> {
                raw::trigger_register_player_alliance_change(which_trigger.to_handle(), which_player.to_handle(), which_alliance).to_weak()
            };
        },
        {
            link: "TriggerRegisterPlayerStateEvent"
            name: trigger_register_player_state_event
            raw: (which_trigger: Handle<j_trigger>, which_player: Handle<j_player>, which_state: i32, opcode: i32, limitval: f32) -> Handle<j_event>;
            user: (which_trigger: &Agent<j_trigger>, which_player: &Agent<j_player>, which_state: i32, opcode: i32, limitval: f32) -> Option<Weak<j_event>> {
                raw::trigger_register_player_state_event(which_trigger.to_handle(), which_player.to_handle(), which_state, opcode, limitval).to_weak()
            };
        },
        {
            link: "GetEventPlayerState"
            name: get_event_player_state
            raw: () -> i32;
            user: () -> i32 {
                raw::get_event_player_state()
            };
        },
        {
            link: "TriggerRegisterPlayerChatEvent"
            name: trigger_register_player_chat_event
            raw: (which_trigger: Handle<j_trigger>, which_player: Handle<j_player>, chat_message_to_detect: *const i8, exact_match_only: bool) -> Handle<j_event>;
            user: (which_trigger: &Agent<j_trigger>, which_player: &Agent<j_player>, chat_message_to_detect: &CStr, exact_match_only: bool) -> Option<Weak<j_event>> {
                raw::trigger_register_player_chat_event(which_trigger.to_handle(), which_player.to_handle(), chat_message_to_detect.as_ptr(), exact_match_only).to_weak()
            };
        },
        {
            link: "GetEventPlayerChatString"
            name: get_event_player_chat_string
            raw: (out: *mut i8, out_len: *mut usize);
            user: () -> String {
                raw::get_event_player_chat_string(stringret::ptr(), stringret::reset_len()); stringret::read_string()
            };
        },
        {
            link: "GetEventPlayerChatStringMatched"
            name: get_event_player_chat_string_matched
            raw: (out: *mut i8, out_len: *mut usize);
            user: () -> String {
                raw::get_event_player_chat_string_matched(stringret::ptr(), stringret::reset_len()); stringret::read_string()
            };
        },
        {
            link: "TriggerRegisterDeathEvent"
            name: trigger_register_death_event
            raw: (which_trigger: Handle<j_trigger>, which_widget: Handle<j_widget>) -> Handle<j_event>;
            user: (which_trigger: &Agent<j_trigger>, which_widget: &Agent<j_widget>) -> Option<Weak<j_event>> {
                raw::trigger_register_death_event(which_trigger.to_handle(), which_widget.to_handle()).to_weak()
            };
        },
        {
            link: "GetTriggerUnit"
            name: get_trigger_unit
            raw: () -> Handle<j_unit>;
            user: () -> Option<Weak<j_unit>> {
                raw::get_trigger_unit().to_weak()
            };
        },
        {
            link: "TriggerRegisterUnitStateEvent"
            name: trigger_register_unit_state_event
            raw: (which_trigger: Handle<j_trigger>, which_unit: Handle<j_unit>, which_state: i32, opcode: i32, limitval: f32) -> Handle<j_event>;
            user: (which_trigger: &Agent<j_trigger>, which_unit: &Agent<j_unit>, which_state: i32, opcode: i32, limitval: f32) -> Option<Weak<j_event>> {
                raw::trigger_register_unit_state_event(which_trigger.to_handle(), which_unit.to_handle(), which_state, opcode, limitval).to_weak()
            };
        },
        {
            link: "GetEventUnitState"
            name: get_event_unit_state
            raw: () -> i32;
            user: () -> i32 {
                raw::get_event_unit_state()
            };
        },
        {
            link: "TriggerRegisterUnitEvent"
            name: trigger_register_unit_event
            raw: (which_trigger: Handle<j_trigger>, which_unit: Handle<j_unit>, which_event: i32) -> Handle<j_event>;
            user: (which_trigger: &Agent<j_trigger>, which_unit: &Agent<j_unit>, which_event: i32) -> Option<Weak<j_event>> {
                raw::trigger_register_unit_event(which_trigger.to_handle(), which_unit.to_handle(), which_event).to_weak()
            };
        },
        {
            link: "GetEventDamage"
            name: get_event_damage
            raw: () -> f32;
            user: () -> f32 {
                raw::get_event_damage()
            };
        },
        {
            link: "GetEventDamageSource"
            name: get_event_damage_source
            raw: () -> Handle<j_unit>;
            user: () -> Option<Weak<j_unit>> {
                raw::get_event_damage_source().to_weak()
            };
        },
        {
            link: "GetEventDetectingPlayer"
            name: get_event_detecting_player
            raw: () -> Handle<j_player>;
            user: () -> Option<Weak<j_player>> {
                raw::get_event_detecting_player().to_weak()
            };
        },
        {
            link: "TriggerRegisterFilterUnitEvent"
            name: trigger_register_filter_unit_event
            raw: (which_trigger: Handle<j_trigger>, which_unit: Handle<j_unit>, which_event: i32, filter: Handle<j_boolexpr>) -> Handle<j_event>;
            user: (which_trigger: &Agent<j_trigger>, which_unit: &Agent<j_unit>, which_event: i32, filter: &Agent<j_boolexpr>) -> Option<Weak<j_event>> {
                raw::trigger_register_filter_unit_event(which_trigger.to_handle(), which_unit.to_handle(), which_event, filter.to_handle()).to_weak()
            };
        },
        {
            link: "GetEventTargetUnit"
            name: get_event_target_unit
            raw: () -> Handle<j_unit>;
            user: () -> Option<Weak<j_unit>> {
                raw::get_event_target_unit().to_weak()
            };
        },
        {
            link: "TriggerRegisterUnitInRange"
            name: trigger_register_unit_in_range
            raw: (which_trigger: Handle<j_trigger>, which_unit: Handle<j_unit>, range: f32, filter: Handle<j_boolexpr>) -> Handle<j_event>;
            user: (which_trigger: &Agent<j_trigger>, which_unit: &Agent<j_unit>, range: f32, filter: &Agent<j_boolexpr>) -> Option<Weak<j_event>> {
                raw::trigger_register_unit_in_range(which_trigger.to_handle(), which_unit.to_handle(), range, filter.to_handle()).to_weak()
            };
        },
        {
            link: "TriggerAddCondition"
            name: trigger_add_condition
            raw: (which_trigger: Handle<j_trigger>, condition: Handle<j_boolexpr>) -> Handle<j_triggercondition>;
            user: (which_trigger: &Agent<j_trigger>, condition: &Agent<j_boolexpr>) -> Option<Weak<j_triggercondition>> {
                raw::trigger_add_condition(which_trigger.to_handle(), condition.to_handle()).to_weak()
            };
        },
        {
            link: "TriggerRemoveCondition"
            name: trigger_remove_condition
            raw: (which_trigger: Handle<j_trigger>, which_condition: Handle<j_triggercondition>);
            user: (which_trigger: &Agent<j_trigger>, which_condition: &Agent<j_triggercondition>) {
                raw::trigger_remove_condition(which_trigger.to_handle(), which_condition.to_handle())
            };
        },
        {
            link: "TriggerClearConditions"
            name: trigger_clear_conditions
            raw: (which_trigger: Handle<j_trigger>);
            user: (which_trigger: &Agent<j_trigger>) {
                raw::trigger_clear_conditions(which_trigger.to_handle())
            };
        },
        {
            link: "TriggerAddAction"
            name: trigger_add_action
            raw: (which_trigger: Handle<j_trigger>, action_func: CodeId) -> Handle<j_triggeraction>;
            user: (which_trigger: &Agent<j_trigger>, action_func: CodeId) -> Handle<j_triggeraction> {
                raw::trigger_add_action(which_trigger.to_handle(), action_func)
            };
        },
        {
            link: "TriggerRemoveAction"
            name: trigger_remove_action
            raw: (which_trigger: Handle<j_trigger>, which_action: Handle<j_triggeraction>);
            user: (which_trigger: &Agent<j_trigger>, which_action: Handle<j_triggeraction>) {
                raw::trigger_remove_action(which_trigger.to_handle(), which_action)
            };
        },
        {
            link: "TriggerClearActions"
            name: trigger_clear_actions
            raw: (which_trigger: Handle<j_trigger>);
            user: (which_trigger: &Agent<j_trigger>) {
                raw::trigger_clear_actions(which_trigger.to_handle())
            };
        },
        {
            link: "TriggerSleepAction"
            name: trigger_sleep_action
            raw: (timeout: f32);
            user: (timeout: f32) {
                raw::trigger_sleep_action(timeout)
            };
        },
        {
            link: "TriggerWaitForSound"
            name: trigger_wait_for_sound
            raw: (s: Handle<j_sound>, offset: f32);
            user: (s: &Agent<j_sound>, offset: f32) {
                raw::trigger_wait_for_sound(s.to_handle(), offset)
            };
        },
        {
            link: "TriggerEvaluate"
            name: trigger_evaluate
            raw: (which_trigger: Handle<j_trigger>) -> bool;
            user: (which_trigger: &Agent<j_trigger>) -> bool {
                raw::trigger_evaluate(which_trigger.to_handle())
            };
        },
        {
            link: "TriggerExecute"
            name: trigger_execute
            raw: (which_trigger: Handle<j_trigger>);
            user: (which_trigger: &Agent<j_trigger>) {
                raw::trigger_execute(which_trigger.to_handle())
            };
        },
        {
            link: "TriggerExecuteWait"
            name: trigger_execute_wait
            raw: (which_trigger: Handle<j_trigger>);
            user: (which_trigger: &Agent<j_trigger>) {
                raw::trigger_execute_wait(which_trigger.to_handle())
            };
        },
        {
            link: "TriggerSyncStart"
            name: trigger_sync_start
            raw: ();
            user: () {
                raw::trigger_sync_start()
            };
        },
        {
            link: "TriggerSyncReady"
            name: trigger_sync_ready
            raw: ();
            user: () {
                raw::trigger_sync_ready()
            };
        },
        {
            link: "GetWidgetLife"
            name: get_widget_life
            raw: (which_widget: Handle<j_widget>) -> f32;
            user: (which_widget: &Agent<j_widget>) -> f32 {
                raw::get_widget_life(which_widget.to_handle())
            };
        },
        {
            link: "SetWidgetLife"
            name: set_widget_life
            raw: (which_widget: Handle<j_widget>, new_life: f32);
            user: (which_widget: &Agent<j_widget>, new_life: f32) {
                raw::set_widget_life(which_widget.to_handle(), new_life)
            };
        },
        {
            link: "GetWidgetX"
            name: get_widget_x
            raw: (which_widget: Handle<j_widget>) -> f32;
            user: (which_widget: &Agent<j_widget>) -> f32 {
                raw::get_widget_x(which_widget.to_handle())
            };
        },
        {
            link: "GetWidgetY"
            name: get_widget_y
            raw: (which_widget: Handle<j_widget>) -> f32;
            user: (which_widget: &Agent<j_widget>) -> f32 {
                raw::get_widget_y(which_widget.to_handle())
            };
        },
        {
            link: "GetTriggerWidget"
            name: get_trigger_widget
            raw: () -> Handle<j_widget>;
            user: () -> Option<Weak<j_widget>> {
                raw::get_trigger_widget().to_weak()
            };
        },
        {
            link: "CreateDestructable"
            name: create_destructable
            raw: (objectid: i32, x: f32, y: f32, face: f32, scale: f32, variation: i32) -> Handle<j_destructable>;
            user: (objectid: i32, x: f32, y: f32, face: f32, scale: f32, variation: i32) -> Option<Weak<j_destructable>> {
                raw::create_destructable(objectid, x, y, face, scale, variation).to_weak()
            };
        },
        {
            link: "CreateDestructableZ"
            name: create_destructable_z
            raw: (objectid: i32, x: f32, y: f32, z: f32, face: f32, scale: f32, variation: i32) -> Handle<j_destructable>;
            user: (objectid: i32, x: f32, y: f32, z: f32, face: f32, scale: f32, variation: i32) -> Option<Weak<j_destructable>> {
                raw::create_destructable_z(objectid, x, y, z, face, scale, variation).to_weak()
            };
        },
        {
            link: "CreateDeadDestructable"
            name: create_dead_destructable
            raw: (objectid: i32, x: f32, y: f32, face: f32, scale: f32, variation: i32) -> Handle<j_destructable>;
            user: (objectid: i32, x: f32, y: f32, face: f32, scale: f32, variation: i32) -> Option<Weak<j_destructable>> {
                raw::create_dead_destructable(objectid, x, y, face, scale, variation).to_weak()
            };
        },
        {
            link: "CreateDeadDestructableZ"
            name: create_dead_destructable_z
            raw: (objectid: i32, x: f32, y: f32, z: f32, face: f32, scale: f32, variation: i32) -> Handle<j_destructable>;
            user: (objectid: i32, x: f32, y: f32, z: f32, face: f32, scale: f32, variation: i32) -> Option<Weak<j_destructable>> {
                raw::create_dead_destructable_z(objectid, x, y, z, face, scale, variation).to_weak()
            };
        },
        {
            link: "RemoveDestructable"
            name: remove_destructable
            raw: (d: Handle<j_destructable>);
            user: (d: &Agent<j_destructable>) {
                raw::remove_destructable(d.to_handle())
            };
        },
        {
            link: "KillDestructable"
            name: kill_destructable
            raw: (d: Handle<j_destructable>);
            user: (d: &Agent<j_destructable>) {
                raw::kill_destructable(d.to_handle())
            };
        },
        {
            link: "SetDestructableInvulnerable"
            name: set_destructable_invulnerable
            raw: (d: Handle<j_destructable>, flag: bool);
            user: (d: &Agent<j_destructable>, flag: bool) {
                raw::set_destructable_invulnerable(d.to_handle(), flag)
            };
        },
        {
            link: "IsDestructableInvulnerable"
            name: is_destructable_invulnerable
            raw: (d: Handle<j_destructable>) -> bool;
            user: (d: &Agent<j_destructable>) -> bool {
                raw::is_destructable_invulnerable(d.to_handle())
            };
        },
        {
            link: "EnumDestructablesInRect"
            name: enum_destructables_in_rect
            raw: (r: Handle<j_rect>, filter: Handle<j_boolexpr>, action_func: CodeId);
            user: (r: &Agent<j_rect>, filter: &Agent<j_boolexpr>, action_func: CodeId) {
                raw::enum_destructables_in_rect(r.to_handle(), filter.to_handle(), action_func)
            };
        },
        {
            link: "GetDestructableTypeId"
            name: get_destructable_type_id
            raw: (d: Handle<j_destructable>) -> i32;
            user: (d: &Agent<j_destructable>) -> i32 {
                raw::get_destructable_type_id(d.to_handle())
            };
        },
        {
            link: "GetDestructableX"
            name: get_destructable_x
            raw: (d: Handle<j_destructable>) -> f32;
            user: (d: &Agent<j_destructable>) -> f32 {
                raw::get_destructable_x(d.to_handle())
            };
        },
        {
            link: "GetDestructableY"
            name: get_destructable_y
            raw: (d: Handle<j_destructable>) -> f32;
            user: (d: &Agent<j_destructable>) -> f32 {
                raw::get_destructable_y(d.to_handle())
            };
        },
        {
            link: "SetDestructableLife"
            name: set_destructable_life
            raw: (d: Handle<j_destructable>, life: f32);
            user: (d: &Agent<j_destructable>, life: f32) {
                raw::set_destructable_life(d.to_handle(), life)
            };
        },
        {
            link: "GetDestructableLife"
            name: get_destructable_life
            raw: (d: Handle<j_destructable>) -> f32;
            user: (d: &Agent<j_destructable>) -> f32 {
                raw::get_destructable_life(d.to_handle())
            };
        },
        {
            link: "SetDestructableMaxLife"
            name: set_destructable_max_life
            raw: (d: Handle<j_destructable>, max: f32);
            user: (d: &Agent<j_destructable>, max: f32) {
                raw::set_destructable_max_life(d.to_handle(), max)
            };
        },
        {
            link: "GetDestructableMaxLife"
            name: get_destructable_max_life
            raw: (d: Handle<j_destructable>) -> f32;
            user: (d: &Agent<j_destructable>) -> f32 {
                raw::get_destructable_max_life(d.to_handle())
            };
        },
        {
            link: "DestructableRestoreLife"
            name: destructable_restore_life
            raw: (d: Handle<j_destructable>, life: f32, birth: bool);
            user: (d: &Agent<j_destructable>, life: f32, birth: bool) {
                raw::destructable_restore_life(d.to_handle(), life, birth)
            };
        },
        {
            link: "QueueDestructableAnimation"
            name: queue_destructable_animation
            raw: (d: Handle<j_destructable>, which_animation: *const i8);
            user: (d: &Agent<j_destructable>, which_animation: &CStr) {
                raw::queue_destructable_animation(d.to_handle(), which_animation.as_ptr())
            };
        },
        {
            link: "SetDestructableAnimation"
            name: set_destructable_animation
            raw: (d: Handle<j_destructable>, which_animation: *const i8);
            user: (d: &Agent<j_destructable>, which_animation: &CStr) {
                raw::set_destructable_animation(d.to_handle(), which_animation.as_ptr())
            };
        },
        {
            link: "SetDestructableAnimationSpeed"
            name: set_destructable_animation_speed
            raw: (d: Handle<j_destructable>, speed_factor: f32);
            user: (d: &Agent<j_destructable>, speed_factor: f32) {
                raw::set_destructable_animation_speed(d.to_handle(), speed_factor)
            };
        },
        {
            link: "ShowDestructable"
            name: show_destructable
            raw: (d: Handle<j_destructable>, flag: bool);
            user: (d: &Agent<j_destructable>, flag: bool) {
                raw::show_destructable(d.to_handle(), flag)
            };
        },
        {
            link: "GetDestructableOccluderHeight"
            name: get_destructable_occluder_height
            raw: (d: Handle<j_destructable>) -> f32;
            user: (d: &Agent<j_destructable>) -> f32 {
                raw::get_destructable_occluder_height(d.to_handle())
            };
        },
        {
            link: "SetDestructableOccluderHeight"
            name: set_destructable_occluder_height
            raw: (d: Handle<j_destructable>, height: f32);
            user: (d: &Agent<j_destructable>, height: f32) {
                raw::set_destructable_occluder_height(d.to_handle(), height)
            };
        },
        {
            link: "GetDestructableName"
            name: get_destructable_name
            raw: (d: Handle<j_destructable>, out: *mut i8, out_len: *mut usize);
            user: (d: &Agent<j_destructable>) -> String {
                raw::get_destructable_name(d.to_handle(), stringret::ptr(), stringret::reset_len()); stringret::read_string()
            };
        },
        {
            link: "GetTriggerDestructable"
            name: get_trigger_destructable
            raw: () -> Handle<j_destructable>;
            user: () -> Option<Weak<j_destructable>> {
                raw::get_trigger_destructable().to_weak()
            };
        },
        {
            link: "CreateItem"
            name: create_item
            raw: (itemid: i32, x: f32, y: f32) -> Handle<j_item>;
            user: (itemid: i32, x: f32, y: f32) -> Option<Weak<j_item>> {
                raw::create_item(itemid, x, y).to_weak()
            };
        },
        {
            link: "RemoveItem"
            name: remove_item
            raw: (which_item: Handle<j_item>);
            user: (which_item: &Agent<j_item>) {
                raw::remove_item(which_item.to_handle())
            };
        },
        {
            link: "GetItemPlayer"
            name: get_item_player
            raw: (which_item: Handle<j_item>) -> Handle<j_player>;
            user: (which_item: &Agent<j_item>) -> Option<Weak<j_player>> {
                raw::get_item_player(which_item.to_handle()).to_weak()
            };
        },
        {
            link: "GetItemTypeId"
            name: get_item_type_id
            raw: (i: Handle<j_item>) -> i32;
            user: (i: &Agent<j_item>) -> i32 {
                raw::get_item_type_id(i.to_handle())
            };
        },
        {
            link: "GetItemX"
            name: get_item_x
            raw: (i: Handle<j_item>) -> f32;
            user: (i: &Agent<j_item>) -> f32 {
                raw::get_item_x(i.to_handle())
            };
        },
        {
            link: "GetItemY"
            name: get_item_y
            raw: (i: Handle<j_item>) -> f32;
            user: (i: &Agent<j_item>) -> f32 {
                raw::get_item_y(i.to_handle())
            };
        },
        {
            link: "SetItemPosition"
            name: set_item_position
            raw: (i: Handle<j_item>, x: f32, y: f32);
            user: (i: &Agent<j_item>, x: f32, y: f32) {
                raw::set_item_position(i.to_handle(), x, y)
            };
        },
        {
            link: "SetItemDropOnDeath"
            name: set_item_drop_on_death
            raw: (which_item: Handle<j_item>, flag: bool);
            user: (which_item: &Agent<j_item>, flag: bool) {
                raw::set_item_drop_on_death(which_item.to_handle(), flag)
            };
        },
        {
            link: "SetItemDroppable"
            name: set_item_droppable
            raw: (i: Handle<j_item>, flag: bool);
            user: (i: &Agent<j_item>, flag: bool) {
                raw::set_item_droppable(i.to_handle(), flag)
            };
        },
        {
            link: "SetItemPawnable"
            name: set_item_pawnable
            raw: (i: Handle<j_item>, flag: bool);
            user: (i: &Agent<j_item>, flag: bool) {
                raw::set_item_pawnable(i.to_handle(), flag)
            };
        },
        {
            link: "SetItemPlayer"
            name: set_item_player
            raw: (which_item: Handle<j_item>, which_player: Handle<j_player>, change_color: bool);
            user: (which_item: &Agent<j_item>, which_player: &Agent<j_player>, change_color: bool) {
                raw::set_item_player(which_item.to_handle(), which_player.to_handle(), change_color)
            };
        },
        {
            link: "SetItemInvulnerable"
            name: set_item_invulnerable
            raw: (which_item: Handle<j_item>, flag: bool);
            user: (which_item: &Agent<j_item>, flag: bool) {
                raw::set_item_invulnerable(which_item.to_handle(), flag)
            };
        },
        {
            link: "IsItemInvulnerable"
            name: is_item_invulnerable
            raw: (which_item: Handle<j_item>) -> bool;
            user: (which_item: &Agent<j_item>) -> bool {
                raw::is_item_invulnerable(which_item.to_handle())
            };
        },
        {
            link: "SetItemVisible"
            name: set_item_visible
            raw: (which_item: Handle<j_item>, show: bool);
            user: (which_item: &Agent<j_item>, show: bool) {
                raw::set_item_visible(which_item.to_handle(), show)
            };
        },
        {
            link: "IsItemVisible"
            name: is_item_visible
            raw: (which_item: Handle<j_item>) -> bool;
            user: (which_item: &Agent<j_item>) -> bool {
                raw::is_item_visible(which_item.to_handle())
            };
        },
        {
            link: "IsItemOwned"
            name: is_item_owned
            raw: (which_item: Handle<j_item>) -> bool;
            user: (which_item: &Agent<j_item>) -> bool {
                raw::is_item_owned(which_item.to_handle())
            };
        },
        {
            link: "IsItemPowerup"
            name: is_item_powerup
            raw: (which_item: Handle<j_item>) -> bool;
            user: (which_item: &Agent<j_item>) -> bool {
                raw::is_item_powerup(which_item.to_handle())
            };
        },
        {
            link: "IsItemSellable"
            name: is_item_sellable
            raw: (which_item: Handle<j_item>) -> bool;
            user: (which_item: &Agent<j_item>) -> bool {
                raw::is_item_sellable(which_item.to_handle())
            };
        },
        {
            link: "IsItemPawnable"
            name: is_item_pawnable
            raw: (which_item: Handle<j_item>) -> bool;
            user: (which_item: &Agent<j_item>) -> bool {
                raw::is_item_pawnable(which_item.to_handle())
            };
        },
        {
            link: "IsItemIdPowerup"
            name: is_item_id_powerup
            raw: (item_id: i32) -> bool;
            user: (item_id: i32) -> bool {
                raw::is_item_id_powerup(item_id)
            };
        },
        {
            link: "IsItemIdSellable"
            name: is_item_id_sellable
            raw: (item_id: i32) -> bool;
            user: (item_id: i32) -> bool {
                raw::is_item_id_sellable(item_id)
            };
        },
        {
            link: "IsItemIdPawnable"
            name: is_item_id_pawnable
            raw: (item_id: i32) -> bool;
            user: (item_id: i32) -> bool {
                raw::is_item_id_pawnable(item_id)
            };
        },
        {
            link: "EnumItemsInRect"
            name: enum_items_in_rect
            raw: (r: Handle<j_rect>, filter: Handle<j_boolexpr>, action_func: CodeId);
            user: (r: &Agent<j_rect>, filter: &Agent<j_boolexpr>, action_func: CodeId) {
                raw::enum_items_in_rect(r.to_handle(), filter.to_handle(), action_func)
            };
        },
        {
            link: "GetItemLevel"
            name: get_item_level
            raw: (which_item: Handle<j_item>) -> i32;
            user: (which_item: &Agent<j_item>) -> i32 {
                raw::get_item_level(which_item.to_handle())
            };
        },
        {
            link: "GetItemType"
            name: get_item_type
            raw: (which_item: Handle<j_item>) -> i32;
            user: (which_item: &Agent<j_item>) -> i32 {
                raw::get_item_type(which_item.to_handle())
            };
        },
        {
            link: "SetItemDropID"
            name: set_item_drop_i_d
            raw: (which_item: Handle<j_item>, unit_id: i32);
            user: (which_item: &Agent<j_item>, unit_id: i32) {
                raw::set_item_drop_i_d(which_item.to_handle(), unit_id)
            };
        },
        {
            link: "GetItemName"
            name: get_item_name
            raw: (which_item: Handle<j_item>, out: *mut i8, out_len: *mut usize);
            user: (which_item: &Agent<j_item>) -> String {
                raw::get_item_name(which_item.to_handle(), stringret::ptr(), stringret::reset_len()); stringret::read_string()
            };
        },
        {
            link: "GetItemCharges"
            name: get_item_charges
            raw: (which_item: Handle<j_item>) -> i32;
            user: (which_item: &Agent<j_item>) -> i32 {
                raw::get_item_charges(which_item.to_handle())
            };
        },
        {
            link: "SetItemCharges"
            name: set_item_charges
            raw: (which_item: Handle<j_item>, charges: i32);
            user: (which_item: &Agent<j_item>, charges: i32) {
                raw::set_item_charges(which_item.to_handle(), charges)
            };
        },
        {
            link: "GetItemUserData"
            name: get_item_user_data
            raw: (which_item: Handle<j_item>) -> i32;
            user: (which_item: &Agent<j_item>) -> i32 {
                raw::get_item_user_data(which_item.to_handle())
            };
        },
        {
            link: "SetItemUserData"
            name: set_item_user_data
            raw: (which_item: Handle<j_item>, data: i32);
            user: (which_item: &Agent<j_item>, data: i32) {
                raw::set_item_user_data(which_item.to_handle(), data)
            };
        },
        {
            link: "CreateUnit"
            name: create_unit
            raw: (id: Handle<j_player>, unitid: i32, x: f32, y: f32, face: f32) -> Handle<j_unit>;
            user: (id: &Agent<j_player>, unitid: i32, x: f32, y: f32, face: f32) -> Option<Weak<j_unit>> {
                raw::create_unit(id.to_handle(), unitid, x, y, face).to_weak()
            };
        },
        {
            link: "CreateUnitByName"
            name: create_unit_by_name
            raw: (which_player: Handle<j_player>, unitname: *const i8, x: f32, y: f32, face: f32) -> Handle<j_unit>;
            user: (which_player: &Agent<j_player>, unitname: &CStr, x: f32, y: f32, face: f32) -> Option<Weak<j_unit>> {
                raw::create_unit_by_name(which_player.to_handle(), unitname.as_ptr(), x, y, face).to_weak()
            };
        },
        {
            link: "CreateUnitAtLoc"
            name: create_unit_at_loc
            raw: (id: Handle<j_player>, unitid: i32, which_location: Handle<j_location>, face: f32) -> Handle<j_unit>;
            user: (id: &Agent<j_player>, unitid: i32, which_location: &Agent<j_location>, face: f32) -> Option<Weak<j_unit>> {
                raw::create_unit_at_loc(id.to_handle(), unitid, which_location.to_handle(), face).to_weak()
            };
        },
        {
            link: "CreateUnitAtLocByName"
            name: create_unit_at_loc_by_name
            raw: (id: Handle<j_player>, unitname: *const i8, which_location: Handle<j_location>, face: f32) -> Handle<j_unit>;
            user: (id: &Agent<j_player>, unitname: &CStr, which_location: &Agent<j_location>, face: f32) -> Option<Weak<j_unit>> {
                raw::create_unit_at_loc_by_name(id.to_handle(), unitname.as_ptr(), which_location.to_handle(), face).to_weak()
            };
        },
        {
            link: "CreateCorpse"
            name: create_corpse
            raw: (which_player: Handle<j_player>, unitid: i32, x: f32, y: f32, face: f32) -> Handle<j_unit>;
            user: (which_player: &Agent<j_player>, unitid: i32, x: f32, y: f32, face: f32) -> Option<Weak<j_unit>> {
                raw::create_corpse(which_player.to_handle(), unitid, x, y, face).to_weak()
            };
        },
        {
            link: "KillUnit"
            name: kill_unit
            raw: (which_unit: Handle<j_unit>);
            user: (which_unit: &Agent<j_unit>) {
                raw::kill_unit(which_unit.to_handle())
            };
        },
        {
            link: "RemoveUnit"
            name: remove_unit
            raw: (which_unit: Handle<j_unit>);
            user: (which_unit: &Agent<j_unit>) {
                raw::remove_unit(which_unit.to_handle())
            };
        },
        {
            link: "ShowUnit"
            name: show_unit
            raw: (which_unit: Handle<j_unit>, show: bool);
            user: (which_unit: &Agent<j_unit>, show: bool) {
                raw::show_unit(which_unit.to_handle(), show)
            };
        },
        {
            link: "SetUnitState"
            name: set_unit_state
            raw: (which_unit: Handle<j_unit>, which_unit_state: i32, new_val: f32);
            user: (which_unit: &Agent<j_unit>, which_unit_state: i32, new_val: f32) {
                raw::set_unit_state(which_unit.to_handle(), which_unit_state, new_val)
            };
        },
        {
            link: "SetUnitX"
            name: set_unit_x
            raw: (which_unit: Handle<j_unit>, new_x: f32);
            user: (which_unit: &Agent<j_unit>, new_x: f32) {
                raw::set_unit_x(which_unit.to_handle(), new_x)
            };
        },
        {
            link: "SetUnitY"
            name: set_unit_y
            raw: (which_unit: Handle<j_unit>, new_y: f32);
            user: (which_unit: &Agent<j_unit>, new_y: f32) {
                raw::set_unit_y(which_unit.to_handle(), new_y)
            };
        },
        {
            link: "SetUnitPosition"
            name: set_unit_position
            raw: (which_unit: Handle<j_unit>, new_x: f32, new_y: f32);
            user: (which_unit: &Agent<j_unit>, new_x: f32, new_y: f32) {
                raw::set_unit_position(which_unit.to_handle(), new_x, new_y)
            };
        },
        {
            link: "SetUnitPositionLoc"
            name: set_unit_position_loc
            raw: (which_unit: Handle<j_unit>, which_location: Handle<j_location>);
            user: (which_unit: &Agent<j_unit>, which_location: &Agent<j_location>) {
                raw::set_unit_position_loc(which_unit.to_handle(), which_location.to_handle())
            };
        },
        {
            link: "SetUnitFacing"
            name: set_unit_facing
            raw: (which_unit: Handle<j_unit>, facing_angle: f32);
            user: (which_unit: &Agent<j_unit>, facing_angle: f32) {
                raw::set_unit_facing(which_unit.to_handle(), facing_angle)
            };
        },
        {
            link: "SetUnitFacingTimed"
            name: set_unit_facing_timed
            raw: (which_unit: Handle<j_unit>, facing_angle: f32, duration: f32);
            user: (which_unit: &Agent<j_unit>, facing_angle: f32, duration: f32) {
                raw::set_unit_facing_timed(which_unit.to_handle(), facing_angle, duration)
            };
        },
        {
            link: "SetUnitMoveSpeed"
            name: set_unit_move_speed
            raw: (which_unit: Handle<j_unit>, new_speed: f32);
            user: (which_unit: &Agent<j_unit>, new_speed: f32) {
                raw::set_unit_move_speed(which_unit.to_handle(), new_speed)
            };
        },
        {
            link: "SetUnitFlyHeight"
            name: set_unit_fly_height
            raw: (which_unit: Handle<j_unit>, new_height: f32, rate: f32);
            user: (which_unit: &Agent<j_unit>, new_height: f32, rate: f32) {
                raw::set_unit_fly_height(which_unit.to_handle(), new_height, rate)
            };
        },
        {
            link: "SetUnitTurnSpeed"
            name: set_unit_turn_speed
            raw: (which_unit: Handle<j_unit>, new_turn_speed: f32);
            user: (which_unit: &Agent<j_unit>, new_turn_speed: f32) {
                raw::set_unit_turn_speed(which_unit.to_handle(), new_turn_speed)
            };
        },
        {
            link: "SetUnitPropWindow"
            name: set_unit_prop_window
            raw: (which_unit: Handle<j_unit>, new_prop_window_angle: f32);
            user: (which_unit: &Agent<j_unit>, new_prop_window_angle: f32) {
                raw::set_unit_prop_window(which_unit.to_handle(), new_prop_window_angle)
            };
        },
        {
            link: "SetUnitAcquireRange"
            name: set_unit_acquire_range
            raw: (which_unit: Handle<j_unit>, new_acquire_range: f32);
            user: (which_unit: &Agent<j_unit>, new_acquire_range: f32) {
                raw::set_unit_acquire_range(which_unit.to_handle(), new_acquire_range)
            };
        },
        {
            link: "SetUnitCreepGuard"
            name: set_unit_creep_guard
            raw: (which_unit: Handle<j_unit>, creep_guard: bool);
            user: (which_unit: &Agent<j_unit>, creep_guard: bool) {
                raw::set_unit_creep_guard(which_unit.to_handle(), creep_guard)
            };
        },
        {
            link: "GetUnitAcquireRange"
            name: get_unit_acquire_range
            raw: (which_unit: Handle<j_unit>) -> f32;
            user: (which_unit: &Agent<j_unit>) -> f32 {
                raw::get_unit_acquire_range(which_unit.to_handle())
            };
        },
        {
            link: "GetUnitTurnSpeed"
            name: get_unit_turn_speed
            raw: (which_unit: Handle<j_unit>) -> f32;
            user: (which_unit: &Agent<j_unit>) -> f32 {
                raw::get_unit_turn_speed(which_unit.to_handle())
            };
        },
        {
            link: "GetUnitPropWindow"
            name: get_unit_prop_window
            raw: (which_unit: Handle<j_unit>) -> f32;
            user: (which_unit: &Agent<j_unit>) -> f32 {
                raw::get_unit_prop_window(which_unit.to_handle())
            };
        },
        {
            link: "GetUnitFlyHeight"
            name: get_unit_fly_height
            raw: (which_unit: Handle<j_unit>) -> f32;
            user: (which_unit: &Agent<j_unit>) -> f32 {
                raw::get_unit_fly_height(which_unit.to_handle())
            };
        },
        {
            link: "GetUnitDefaultAcquireRange"
            name: get_unit_default_acquire_range
            raw: (which_unit: Handle<j_unit>) -> f32;
            user: (which_unit: &Agent<j_unit>) -> f32 {
                raw::get_unit_default_acquire_range(which_unit.to_handle())
            };
        },
        {
            link: "GetUnitDefaultTurnSpeed"
            name: get_unit_default_turn_speed
            raw: (which_unit: Handle<j_unit>) -> f32;
            user: (which_unit: &Agent<j_unit>) -> f32 {
                raw::get_unit_default_turn_speed(which_unit.to_handle())
            };
        },
        {
            link: "GetUnitDefaultPropWindow"
            name: get_unit_default_prop_window
            raw: (which_unit: Handle<j_unit>) -> f32;
            user: (which_unit: &Agent<j_unit>) -> f32 {
                raw::get_unit_default_prop_window(which_unit.to_handle())
            };
        },
        {
            link: "GetUnitDefaultFlyHeight"
            name: get_unit_default_fly_height
            raw: (which_unit: Handle<j_unit>) -> f32;
            user: (which_unit: &Agent<j_unit>) -> f32 {
                raw::get_unit_default_fly_height(which_unit.to_handle())
            };
        },
        {
            link: "SetUnitOwner"
            name: set_unit_owner
            raw: (which_unit: Handle<j_unit>, which_player: Handle<j_player>, change_color: bool);
            user: (which_unit: &Agent<j_unit>, which_player: &Agent<j_player>, change_color: bool) {
                raw::set_unit_owner(which_unit.to_handle(), which_player.to_handle(), change_color)
            };
        },
        {
            link: "SetUnitColor"
            name: set_unit_color
            raw: (which_unit: Handle<j_unit>, which_color: i32);
            user: (which_unit: &Agent<j_unit>, which_color: i32) {
                raw::set_unit_color(which_unit.to_handle(), which_color)
            };
        },
        {
            link: "SetUnitScale"
            name: set_unit_scale
            raw: (which_unit: Handle<j_unit>, scale_x: f32, scale_y: f32, scale_z: f32);
            user: (which_unit: &Agent<j_unit>, scale_x: f32, scale_y: f32, scale_z: f32) {
                raw::set_unit_scale(which_unit.to_handle(), scale_x, scale_y, scale_z)
            };
        },
        {
            link: "SetUnitTimeScale"
            name: set_unit_time_scale
            raw: (which_unit: Handle<j_unit>, time_scale: f32);
            user: (which_unit: &Agent<j_unit>, time_scale: f32) {
                raw::set_unit_time_scale(which_unit.to_handle(), time_scale)
            };
        },
        {
            link: "SetUnitBlendTime"
            name: set_unit_blend_time
            raw: (which_unit: Handle<j_unit>, blend_time: f32);
            user: (which_unit: &Agent<j_unit>, blend_time: f32) {
                raw::set_unit_blend_time(which_unit.to_handle(), blend_time)
            };
        },
        {
            link: "SetUnitVertexColor"
            name: set_unit_vertex_color
            raw: (which_unit: Handle<j_unit>, red: i32, green: i32, blue: i32, alpha: i32);
            user: (which_unit: &Agent<j_unit>, red: i32, green: i32, blue: i32, alpha: i32) {
                raw::set_unit_vertex_color(which_unit.to_handle(), red, green, blue, alpha)
            };
        },
        {
            link: "QueueUnitAnimation"
            name: queue_unit_animation
            raw: (which_unit: Handle<j_unit>, which_animation: *const i8);
            user: (which_unit: &Agent<j_unit>, which_animation: &CStr) {
                raw::queue_unit_animation(which_unit.to_handle(), which_animation.as_ptr())
            };
        },
        {
            link: "SetUnitAnimation"
            name: set_unit_animation
            raw: (which_unit: Handle<j_unit>, which_animation: *const i8);
            user: (which_unit: &Agent<j_unit>, which_animation: &CStr) {
                raw::set_unit_animation(which_unit.to_handle(), which_animation.as_ptr())
            };
        },
        {
            link: "SetUnitAnimationByIndex"
            name: set_unit_animation_by_index
            raw: (which_unit: Handle<j_unit>, which_animation: i32);
            user: (which_unit: &Agent<j_unit>, which_animation: i32) {
                raw::set_unit_animation_by_index(which_unit.to_handle(), which_animation)
            };
        },
        {
            link: "SetUnitAnimationWithRarity"
            name: set_unit_animation_with_rarity
            raw: (which_unit: Handle<j_unit>, which_animation: *const i8, rarity: i32);
            user: (which_unit: &Agent<j_unit>, which_animation: &CStr, rarity: i32) {
                raw::set_unit_animation_with_rarity(which_unit.to_handle(), which_animation.as_ptr(), rarity)
            };
        },
        {
            link: "AddUnitAnimationProperties"
            name: add_unit_animation_properties
            raw: (which_unit: Handle<j_unit>, anim_properties: *const i8, add: bool);
            user: (which_unit: &Agent<j_unit>, anim_properties: &CStr, add: bool) {
                raw::add_unit_animation_properties(which_unit.to_handle(), anim_properties.as_ptr(), add)
            };
        },
        {
            link: "SetUnitLookAt"
            name: set_unit_look_at
            raw: (which_unit: Handle<j_unit>, which_bone: *const i8, look_at_target: Handle<j_unit>, offset_x: f32, offset_y: f32, offset_z: f32);
            user: (which_unit: &Agent<j_unit>, which_bone: &CStr, look_at_target: &Agent<j_unit>, offset_x: f32, offset_y: f32, offset_z: f32) {
                raw::set_unit_look_at(which_unit.to_handle(), which_bone.as_ptr(), look_at_target.to_handle(), offset_x, offset_y, offset_z)
            };
        },
        {
            link: "ResetUnitLookAt"
            name: reset_unit_look_at
            raw: (which_unit: Handle<j_unit>);
            user: (which_unit: &Agent<j_unit>) {
                raw::reset_unit_look_at(which_unit.to_handle())
            };
        },
        {
            link: "SetUnitRescuable"
            name: set_unit_rescuable
            raw: (which_unit: Handle<j_unit>, by_which_player: Handle<j_player>, flag: bool);
            user: (which_unit: &Agent<j_unit>, by_which_player: &Agent<j_player>, flag: bool) {
                raw::set_unit_rescuable(which_unit.to_handle(), by_which_player.to_handle(), flag)
            };
        },
        {
            link: "SetUnitRescueRange"
            name: set_unit_rescue_range
            raw: (which_unit: Handle<j_unit>, range: f32);
            user: (which_unit: &Agent<j_unit>, range: f32) {
                raw::set_unit_rescue_range(which_unit.to_handle(), range)
            };
        },
        {
            link: "SetHeroStr"
            name: set_hero_str
            raw: (which_hero: Handle<j_unit>, new_str: i32, permanent: bool);
            user: (which_hero: &Agent<j_unit>, new_str: i32, permanent: bool) {
                raw::set_hero_str(which_hero.to_handle(), new_str, permanent)
            };
        },
        {
            link: "SetHeroAgi"
            name: set_hero_agi
            raw: (which_hero: Handle<j_unit>, new_agi: i32, permanent: bool);
            user: (which_hero: &Agent<j_unit>, new_agi: i32, permanent: bool) {
                raw::set_hero_agi(which_hero.to_handle(), new_agi, permanent)
            };
        },
        {
            link: "SetHeroInt"
            name: set_hero_int
            raw: (which_hero: Handle<j_unit>, new_int: i32, permanent: bool);
            user: (which_hero: &Agent<j_unit>, new_int: i32, permanent: bool) {
                raw::set_hero_int(which_hero.to_handle(), new_int, permanent)
            };
        },
        {
            link: "GetHeroStr"
            name: get_hero_str
            raw: (which_hero: Handle<j_unit>, include_bonuses: bool) -> i32;
            user: (which_hero: &Agent<j_unit>, include_bonuses: bool) -> i32 {
                raw::get_hero_str(which_hero.to_handle(), include_bonuses)
            };
        },
        {
            link: "GetHeroAgi"
            name: get_hero_agi
            raw: (which_hero: Handle<j_unit>, include_bonuses: bool) -> i32;
            user: (which_hero: &Agent<j_unit>, include_bonuses: bool) -> i32 {
                raw::get_hero_agi(which_hero.to_handle(), include_bonuses)
            };
        },
        {
            link: "GetHeroInt"
            name: get_hero_int
            raw: (which_hero: Handle<j_unit>, include_bonuses: bool) -> i32;
            user: (which_hero: &Agent<j_unit>, include_bonuses: bool) -> i32 {
                raw::get_hero_int(which_hero.to_handle(), include_bonuses)
            };
        },
        {
            link: "UnitStripHeroLevel"
            name: unit_strip_hero_level
            raw: (which_hero: Handle<j_unit>, how_many_levels: i32) -> bool;
            user: (which_hero: &Agent<j_unit>, how_many_levels: i32) -> bool {
                raw::unit_strip_hero_level(which_hero.to_handle(), how_many_levels)
            };
        },
        {
            link: "GetHeroXP"
            name: get_hero_x_p
            raw: (which_hero: Handle<j_unit>) -> i32;
            user: (which_hero: &Agent<j_unit>) -> i32 {
                raw::get_hero_x_p(which_hero.to_handle())
            };
        },
        {
            link: "SetHeroXP"
            name: set_hero_x_p
            raw: (which_hero: Handle<j_unit>, new_xp_val: i32, show_eye_candy: bool);
            user: (which_hero: &Agent<j_unit>, new_xp_val: i32, show_eye_candy: bool) {
                raw::set_hero_x_p(which_hero.to_handle(), new_xp_val, show_eye_candy)
            };
        },
        {
            link: "GetHeroSkillPoints"
            name: get_hero_skill_points
            raw: (which_hero: Handle<j_unit>) -> i32;
            user: (which_hero: &Agent<j_unit>) -> i32 {
                raw::get_hero_skill_points(which_hero.to_handle())
            };
        },
        {
            link: "UnitModifySkillPoints"
            name: unit_modify_skill_points
            raw: (which_hero: Handle<j_unit>, skill_point_delta: i32) -> bool;
            user: (which_hero: &Agent<j_unit>, skill_point_delta: i32) -> bool {
                raw::unit_modify_skill_points(which_hero.to_handle(), skill_point_delta)
            };
        },
        {
            link: "AddHeroXP"
            name: add_hero_x_p
            raw: (which_hero: Handle<j_unit>, xp_to_add: i32, show_eye_candy: bool);
            user: (which_hero: &Agent<j_unit>, xp_to_add: i32, show_eye_candy: bool) {
                raw::add_hero_x_p(which_hero.to_handle(), xp_to_add, show_eye_candy)
            };
        },
        {
            link: "SetHeroLevel"
            name: set_hero_level
            raw: (which_hero: Handle<j_unit>, level: i32, show_eye_candy: bool);
            user: (which_hero: &Agent<j_unit>, level: i32, show_eye_candy: bool) {
                raw::set_hero_level(which_hero.to_handle(), level, show_eye_candy)
            };
        },
        {
            link: "GetHeroLevel"
            name: get_hero_level
            raw: (which_hero: Handle<j_unit>) -> i32;
            user: (which_hero: &Agent<j_unit>) -> i32 {
                raw::get_hero_level(which_hero.to_handle())
            };
        },
        {
            link: "GetUnitLevel"
            name: get_unit_level
            raw: (which_unit: Handle<j_unit>) -> i32;
            user: (which_unit: &Agent<j_unit>) -> i32 {
                raw::get_unit_level(which_unit.to_handle())
            };
        },
        {
            link: "GetHeroProperName"
            name: get_hero_proper_name
            raw: (which_hero: Handle<j_unit>, out: *mut i8, out_len: *mut usize);
            user: (which_hero: &Agent<j_unit>) -> String {
                raw::get_hero_proper_name(which_hero.to_handle(), stringret::ptr(), stringret::reset_len()); stringret::read_string()
            };
        },
        {
            link: "SuspendHeroXP"
            name: suspend_hero_x_p
            raw: (which_hero: Handle<j_unit>, flag: bool);
            user: (which_hero: &Agent<j_unit>, flag: bool) {
                raw::suspend_hero_x_p(which_hero.to_handle(), flag)
            };
        },
        {
            link: "IsSuspendedXP"
            name: is_suspended_x_p
            raw: (which_hero: Handle<j_unit>) -> bool;
            user: (which_hero: &Agent<j_unit>) -> bool {
                raw::is_suspended_x_p(which_hero.to_handle())
            };
        },
        {
            link: "SelectHeroSkill"
            name: select_hero_skill
            raw: (which_hero: Handle<j_unit>, abilcode: i32);
            user: (which_hero: &Agent<j_unit>, abilcode: i32) {
                raw::select_hero_skill(which_hero.to_handle(), abilcode)
            };
        },
        {
            link: "GetUnitAbilityLevel"
            name: get_unit_ability_level
            raw: (which_unit: Handle<j_unit>, abilcode: i32) -> i32;
            user: (which_unit: &Agent<j_unit>, abilcode: i32) -> i32 {
                raw::get_unit_ability_level(which_unit.to_handle(), abilcode)
            };
        },
        {
            link: "DecUnitAbilityLevel"
            name: dec_unit_ability_level
            raw: (which_unit: Handle<j_unit>, abilcode: i32) -> i32;
            user: (which_unit: &Agent<j_unit>, abilcode: i32) -> i32 {
                raw::dec_unit_ability_level(which_unit.to_handle(), abilcode)
            };
        },
        {
            link: "IncUnitAbilityLevel"
            name: inc_unit_ability_level
            raw: (which_unit: Handle<j_unit>, abilcode: i32) -> i32;
            user: (which_unit: &Agent<j_unit>, abilcode: i32) -> i32 {
                raw::inc_unit_ability_level(which_unit.to_handle(), abilcode)
            };
        },
        {
            link: "SetUnitAbilityLevel"
            name: set_unit_ability_level
            raw: (which_unit: Handle<j_unit>, abilcode: i32, level: i32) -> i32;
            user: (which_unit: &Agent<j_unit>, abilcode: i32, level: i32) -> i32 {
                raw::set_unit_ability_level(which_unit.to_handle(), abilcode, level)
            };
        },
        {
            link: "ReviveHero"
            name: revive_hero
            raw: (which_hero: Handle<j_unit>, x: f32, y: f32, do_eyecandy: bool) -> bool;
            user: (which_hero: &Agent<j_unit>, x: f32, y: f32, do_eyecandy: bool) -> bool {
                raw::revive_hero(which_hero.to_handle(), x, y, do_eyecandy)
            };
        },
        {
            link: "ReviveHeroLoc"
            name: revive_hero_loc
            raw: (which_hero: Handle<j_unit>, loc: Handle<j_location>, do_eyecandy: bool) -> bool;
            user: (which_hero: &Agent<j_unit>, loc: &Agent<j_location>, do_eyecandy: bool) -> bool {
                raw::revive_hero_loc(which_hero.to_handle(), loc.to_handle(), do_eyecandy)
            };
        },
        {
            link: "SetUnitExploded"
            name: set_unit_exploded
            raw: (which_unit: Handle<j_unit>, exploded: bool);
            user: (which_unit: &Agent<j_unit>, exploded: bool) {
                raw::set_unit_exploded(which_unit.to_handle(), exploded)
            };
        },
        {
            link: "SetUnitInvulnerable"
            name: set_unit_invulnerable
            raw: (which_unit: Handle<j_unit>, flag: bool);
            user: (which_unit: &Agent<j_unit>, flag: bool) {
                raw::set_unit_invulnerable(which_unit.to_handle(), flag)
            };
        },
        {
            link: "PauseUnit"
            name: pause_unit
            raw: (which_unit: Handle<j_unit>, flag: bool);
            user: (which_unit: &Agent<j_unit>, flag: bool) {
                raw::pause_unit(which_unit.to_handle(), flag)
            };
        },
        {
            link: "IsUnitPaused"
            name: is_unit_paused
            raw: (which_hero: Handle<j_unit>) -> bool;
            user: (which_hero: &Agent<j_unit>) -> bool {
                raw::is_unit_paused(which_hero.to_handle())
            };
        },
        {
            link: "SetUnitPathing"
            name: set_unit_pathing
            raw: (which_unit: Handle<j_unit>, flag: bool);
            user: (which_unit: &Agent<j_unit>, flag: bool) {
                raw::set_unit_pathing(which_unit.to_handle(), flag)
            };
        },
        {
            link: "ClearSelection"
            name: clear_selection
            raw: ();
            user: () {
                raw::clear_selection()
            };
        },
        {
            link: "SelectUnit"
            name: select_unit
            raw: (which_unit: Handle<j_unit>, flag: bool);
            user: (which_unit: &Agent<j_unit>, flag: bool) {
                raw::select_unit(which_unit.to_handle(), flag)
            };
        },
        {
            link: "GetUnitPointValue"
            name: get_unit_point_value
            raw: (which_unit: Handle<j_unit>) -> i32;
            user: (which_unit: &Agent<j_unit>) -> i32 {
                raw::get_unit_point_value(which_unit.to_handle())
            };
        },
        {
            link: "GetUnitPointValueByType"
            name: get_unit_point_value_by_type
            raw: (unit_type: i32) -> i32;
            user: (unit_type: i32) -> i32 {
                raw::get_unit_point_value_by_type(unit_type)
            };
        },
        {
            link: "UnitAddItem"
            name: unit_add_item
            raw: (which_unit: Handle<j_unit>, which_item: Handle<j_item>) -> bool;
            user: (which_unit: &Agent<j_unit>, which_item: &Agent<j_item>) -> bool {
                raw::unit_add_item(which_unit.to_handle(), which_item.to_handle())
            };
        },
        {
            link: "UnitAddItemById"
            name: unit_add_item_by_id
            raw: (which_unit: Handle<j_unit>, item_id: i32) -> Handle<j_item>;
            user: (which_unit: &Agent<j_unit>, item_id: i32) -> Option<Weak<j_item>> {
                raw::unit_add_item_by_id(which_unit.to_handle(), item_id).to_weak()
            };
        },
        {
            link: "UnitAddItemToSlotById"
            name: unit_add_item_to_slot_by_id
            raw: (which_unit: Handle<j_unit>, item_id: i32, item_slot: i32) -> bool;
            user: (which_unit: &Agent<j_unit>, item_id: i32, item_slot: i32) -> bool {
                raw::unit_add_item_to_slot_by_id(which_unit.to_handle(), item_id, item_slot)
            };
        },
        {
            link: "UnitRemoveItem"
            name: unit_remove_item
            raw: (which_unit: Handle<j_unit>, which_item: Handle<j_item>);
            user: (which_unit: &Agent<j_unit>, which_item: &Agent<j_item>) {
                raw::unit_remove_item(which_unit.to_handle(), which_item.to_handle())
            };
        },
        {
            link: "UnitRemoveItemFromSlot"
            name: unit_remove_item_from_slot
            raw: (which_unit: Handle<j_unit>, item_slot: i32) -> Handle<j_item>;
            user: (which_unit: &Agent<j_unit>, item_slot: i32) -> Option<Weak<j_item>> {
                raw::unit_remove_item_from_slot(which_unit.to_handle(), item_slot).to_weak()
            };
        },
        {
            link: "UnitHasItem"
            name: unit_has_item
            raw: (which_unit: Handle<j_unit>, which_item: Handle<j_item>) -> bool;
            user: (which_unit: &Agent<j_unit>, which_item: &Agent<j_item>) -> bool {
                raw::unit_has_item(which_unit.to_handle(), which_item.to_handle())
            };
        },
        {
            link: "UnitItemInSlot"
            name: unit_item_in_slot
            raw: (which_unit: Handle<j_unit>, item_slot: i32) -> Handle<j_item>;
            user: (which_unit: &Agent<j_unit>, item_slot: i32) -> Option<Weak<j_item>> {
                raw::unit_item_in_slot(which_unit.to_handle(), item_slot).to_weak()
            };
        },
        {
            link: "UnitInventorySize"
            name: unit_inventory_size
            raw: (which_unit: Handle<j_unit>) -> i32;
            user: (which_unit: &Agent<j_unit>) -> i32 {
                raw::unit_inventory_size(which_unit.to_handle())
            };
        },
        {
            link: "UnitDropItemPoint"
            name: unit_drop_item_point
            raw: (which_unit: Handle<j_unit>, which_item: Handle<j_item>, x: f32, y: f32) -> bool;
            user: (which_unit: &Agent<j_unit>, which_item: &Agent<j_item>, x: f32, y: f32) -> bool {
                raw::unit_drop_item_point(which_unit.to_handle(), which_item.to_handle(), x, y)
            };
        },
        {
            link: "UnitDropItemSlot"
            name: unit_drop_item_slot
            raw: (which_unit: Handle<j_unit>, which_item: Handle<j_item>, slot: i32) -> bool;
            user: (which_unit: &Agent<j_unit>, which_item: &Agent<j_item>, slot: i32) -> bool {
                raw::unit_drop_item_slot(which_unit.to_handle(), which_item.to_handle(), slot)
            };
        },
        {
            link: "UnitDropItemTarget"
            name: unit_drop_item_target
            raw: (which_unit: Handle<j_unit>, which_item: Handle<j_item>, target: Handle<j_widget>) -> bool;
            user: (which_unit: &Agent<j_unit>, which_item: &Agent<j_item>, target: &Agent<j_widget>) -> bool {
                raw::unit_drop_item_target(which_unit.to_handle(), which_item.to_handle(), target.to_handle())
            };
        },
        {
            link: "UnitUseItem"
            name: unit_use_item
            raw: (which_unit: Handle<j_unit>, which_item: Handle<j_item>) -> bool;
            user: (which_unit: &Agent<j_unit>, which_item: &Agent<j_item>) -> bool {
                raw::unit_use_item(which_unit.to_handle(), which_item.to_handle())
            };
        },
        {
            link: "UnitUseItemPoint"
            name: unit_use_item_point
            raw: (which_unit: Handle<j_unit>, which_item: Handle<j_item>, x: f32, y: f32) -> bool;
            user: (which_unit: &Agent<j_unit>, which_item: &Agent<j_item>, x: f32, y: f32) -> bool {
                raw::unit_use_item_point(which_unit.to_handle(), which_item.to_handle(), x, y)
            };
        },
        {
            link: "UnitUseItemTarget"
            name: unit_use_item_target
            raw: (which_unit: Handle<j_unit>, which_item: Handle<j_item>, target: Handle<j_widget>) -> bool;
            user: (which_unit: &Agent<j_unit>, which_item: &Agent<j_item>, target: &Agent<j_widget>) -> bool {
                raw::unit_use_item_target(which_unit.to_handle(), which_item.to_handle(), target.to_handle())
            };
        },
        {
            link: "GetUnitX"
            name: get_unit_x
            raw: (which_unit: Handle<j_unit>) -> f32;
            user: (which_unit: &Agent<j_unit>) -> f32 {
                raw::get_unit_x(which_unit.to_handle())
            };
        },
        {
            link: "GetUnitY"
            name: get_unit_y
            raw: (which_unit: Handle<j_unit>) -> f32;
            user: (which_unit: &Agent<j_unit>) -> f32 {
                raw::get_unit_y(which_unit.to_handle())
            };
        },
        {
            link: "GetUnitLoc"
            name: get_unit_loc
            raw: (which_unit: Handle<j_unit>) -> Handle<j_location>;
            user: (which_unit: &Agent<j_unit>) -> Option<Weak<j_location>> {
                raw::get_unit_loc(which_unit.to_handle()).to_weak()
            };
        },
        {
            link: "GetUnitFacing"
            name: get_unit_facing
            raw: (which_unit: Handle<j_unit>) -> f32;
            user: (which_unit: &Agent<j_unit>) -> f32 {
                raw::get_unit_facing(which_unit.to_handle())
            };
        },
        {
            link: "GetUnitMoveSpeed"
            name: get_unit_move_speed
            raw: (which_unit: Handle<j_unit>) -> f32;
            user: (which_unit: &Agent<j_unit>) -> f32 {
                raw::get_unit_move_speed(which_unit.to_handle())
            };
        },
        {
            link: "GetUnitDefaultMoveSpeed"
            name: get_unit_default_move_speed
            raw: (which_unit: Handle<j_unit>) -> f32;
            user: (which_unit: &Agent<j_unit>) -> f32 {
                raw::get_unit_default_move_speed(which_unit.to_handle())
            };
        },
        {
            link: "GetUnitState"
            name: get_unit_state
            raw: (which_unit: Handle<j_unit>, which_unit_state: i32) -> f32;
            user: (which_unit: &Agent<j_unit>, which_unit_state: i32) -> f32 {
                raw::get_unit_state(which_unit.to_handle(), which_unit_state)
            };
        },
        {
            link: "GetOwningPlayer"
            name: get_owning_player
            raw: (which_unit: Handle<j_unit>) -> Handle<j_player>;
            user: (which_unit: &Agent<j_unit>) -> Option<Weak<j_player>> {
                raw::get_owning_player(which_unit.to_handle()).to_weak()
            };
        },
        {
            link: "GetUnitTypeId"
            name: get_unit_type_id
            raw: (which_unit: Handle<j_unit>) -> i32;
            user: (which_unit: &Agent<j_unit>) -> i32 {
                raw::get_unit_type_id(which_unit.to_handle())
            };
        },
        {
            link: "GetUnitRace"
            name: get_unit_race
            raw: (which_unit: Handle<j_unit>) -> i32;
            user: (which_unit: &Agent<j_unit>) -> i32 {
                raw::get_unit_race(which_unit.to_handle())
            };
        },
        {
            link: "GetUnitName"
            name: get_unit_name
            raw: (which_unit: Handle<j_unit>, out: *mut i8, out_len: *mut usize);
            user: (which_unit: &Agent<j_unit>) -> String {
                raw::get_unit_name(which_unit.to_handle(), stringret::ptr(), stringret::reset_len()); stringret::read_string()
            };
        },
        {
            link: "GetUnitFoodUsed"
            name: get_unit_food_used
            raw: (which_unit: Handle<j_unit>) -> i32;
            user: (which_unit: &Agent<j_unit>) -> i32 {
                raw::get_unit_food_used(which_unit.to_handle())
            };
        },
        {
            link: "GetUnitFoodMade"
            name: get_unit_food_made
            raw: (which_unit: Handle<j_unit>) -> i32;
            user: (which_unit: &Agent<j_unit>) -> i32 {
                raw::get_unit_food_made(which_unit.to_handle())
            };
        },
        {
            link: "GetFoodMade"
            name: get_food_made
            raw: (unit_id: i32) -> i32;
            user: (unit_id: i32) -> i32 {
                raw::get_food_made(unit_id)
            };
        },
        {
            link: "GetFoodUsed"
            name: get_food_used
            raw: (unit_id: i32) -> i32;
            user: (unit_id: i32) -> i32 {
                raw::get_food_used(unit_id)
            };
        },
        {
            link: "SetUnitUseFood"
            name: set_unit_use_food
            raw: (which_unit: Handle<j_unit>, use_food: bool);
            user: (which_unit: &Agent<j_unit>, use_food: bool) {
                raw::set_unit_use_food(which_unit.to_handle(), use_food)
            };
        },
        {
            link: "GetUnitRallyPoint"
            name: get_unit_rally_point
            raw: (which_unit: Handle<j_unit>) -> Handle<j_location>;
            user: (which_unit: &Agent<j_unit>) -> Option<Weak<j_location>> {
                raw::get_unit_rally_point(which_unit.to_handle()).to_weak()
            };
        },
        {
            link: "GetUnitRallyUnit"
            name: get_unit_rally_unit
            raw: (which_unit: Handle<j_unit>) -> Handle<j_unit>;
            user: (which_unit: &Agent<j_unit>) -> Option<Weak<j_unit>> {
                raw::get_unit_rally_unit(which_unit.to_handle()).to_weak()
            };
        },
        {
            link: "GetUnitRallyDestructable"
            name: get_unit_rally_destructable
            raw: (which_unit: Handle<j_unit>) -> Handle<j_destructable>;
            user: (which_unit: &Agent<j_unit>) -> Option<Weak<j_destructable>> {
                raw::get_unit_rally_destructable(which_unit.to_handle()).to_weak()
            };
        },
        {
            link: "IsUnitInGroup"
            name: is_unit_in_group
            raw: (which_unit: Handle<j_unit>, which_group: Handle<j_group>) -> bool;
            user: (which_unit: &Agent<j_unit>, which_group: &Agent<j_group>) -> bool {
                raw::is_unit_in_group(which_unit.to_handle(), which_group.to_handle())
            };
        },
        {
            link: "IsUnitInForce"
            name: is_unit_in_force
            raw: (which_unit: Handle<j_unit>, which_force: Handle<j_force>) -> bool;
            user: (which_unit: &Agent<j_unit>, which_force: &Agent<j_force>) -> bool {
                raw::is_unit_in_force(which_unit.to_handle(), which_force.to_handle())
            };
        },
        {
            link: "IsUnitOwnedByPlayer"
            name: is_unit_owned_by_player
            raw: (which_unit: Handle<j_unit>, which_player: Handle<j_player>) -> bool;
            user: (which_unit: &Agent<j_unit>, which_player: &Agent<j_player>) -> bool {
                raw::is_unit_owned_by_player(which_unit.to_handle(), which_player.to_handle())
            };
        },
        {
            link: "IsUnitAlly"
            name: is_unit_ally
            raw: (which_unit: Handle<j_unit>, which_player: Handle<j_player>) -> bool;
            user: (which_unit: &Agent<j_unit>, which_player: &Agent<j_player>) -> bool {
                raw::is_unit_ally(which_unit.to_handle(), which_player.to_handle())
            };
        },
        {
            link: "IsUnitEnemy"
            name: is_unit_enemy
            raw: (which_unit: Handle<j_unit>, which_player: Handle<j_player>) -> bool;
            user: (which_unit: &Agent<j_unit>, which_player: &Agent<j_player>) -> bool {
                raw::is_unit_enemy(which_unit.to_handle(), which_player.to_handle())
            };
        },
        {
            link: "IsUnitVisible"
            name: is_unit_visible
            raw: (which_unit: Handle<j_unit>, which_player: Handle<j_player>) -> bool;
            user: (which_unit: &Agent<j_unit>, which_player: &Agent<j_player>) -> bool {
                raw::is_unit_visible(which_unit.to_handle(), which_player.to_handle())
            };
        },
        {
            link: "IsUnitDetected"
            name: is_unit_detected
            raw: (which_unit: Handle<j_unit>, which_player: Handle<j_player>) -> bool;
            user: (which_unit: &Agent<j_unit>, which_player: &Agent<j_player>) -> bool {
                raw::is_unit_detected(which_unit.to_handle(), which_player.to_handle())
            };
        },
        {
            link: "IsUnitInvisible"
            name: is_unit_invisible
            raw: (which_unit: Handle<j_unit>, which_player: Handle<j_player>) -> bool;
            user: (which_unit: &Agent<j_unit>, which_player: &Agent<j_player>) -> bool {
                raw::is_unit_invisible(which_unit.to_handle(), which_player.to_handle())
            };
        },
        {
            link: "IsUnitFogged"
            name: is_unit_fogged
            raw: (which_unit: Handle<j_unit>, which_player: Handle<j_player>) -> bool;
            user: (which_unit: &Agent<j_unit>, which_player: &Agent<j_player>) -> bool {
                raw::is_unit_fogged(which_unit.to_handle(), which_player.to_handle())
            };
        },
        {
            link: "IsUnitMasked"
            name: is_unit_masked
            raw: (which_unit: Handle<j_unit>, which_player: Handle<j_player>) -> bool;
            user: (which_unit: &Agent<j_unit>, which_player: &Agent<j_player>) -> bool {
                raw::is_unit_masked(which_unit.to_handle(), which_player.to_handle())
            };
        },
        {
            link: "IsUnitSelected"
            name: is_unit_selected
            raw: (which_unit: Handle<j_unit>, which_player: Handle<j_player>) -> bool;
            user: (which_unit: &Agent<j_unit>, which_player: &Agent<j_player>) -> bool {
                raw::is_unit_selected(which_unit.to_handle(), which_player.to_handle())
            };
        },
        {
            link: "IsUnitRace"
            name: is_unit_race
            raw: (which_unit: Handle<j_unit>, which_race: i32) -> bool;
            user: (which_unit: &Agent<j_unit>, which_race: i32) -> bool {
                raw::is_unit_race(which_unit.to_handle(), which_race)
            };
        },
        {
            link: "IsUnitType"
            name: is_unit_type
            raw: (which_unit: Handle<j_unit>, which_unit_type: i32) -> bool;
            user: (which_unit: &Agent<j_unit>, which_unit_type: i32) -> bool {
                raw::is_unit_type(which_unit.to_handle(), which_unit_type)
            };
        },
        {
            link: "IsUnit"
            name: is_unit
            raw: (which_unit: Handle<j_unit>, which_specified_unit: Handle<j_unit>) -> bool;
            user: (which_unit: &Agent<j_unit>, which_specified_unit: &Agent<j_unit>) -> bool {
                raw::is_unit(which_unit.to_handle(), which_specified_unit.to_handle())
            };
        },
        {
            link: "IsUnitInRange"
            name: is_unit_in_range
            raw: (which_unit: Handle<j_unit>, other_unit: Handle<j_unit>, distance: f32) -> bool;
            user: (which_unit: &Agent<j_unit>, other_unit: &Agent<j_unit>, distance: f32) -> bool {
                raw::is_unit_in_range(which_unit.to_handle(), other_unit.to_handle(), distance)
            };
        },
        {
            link: "IsUnitInRangeXY"
            name: is_unit_in_range_x_y
            raw: (which_unit: Handle<j_unit>, x: f32, y: f32, distance: f32) -> bool;
            user: (which_unit: &Agent<j_unit>, x: f32, y: f32, distance: f32) -> bool {
                raw::is_unit_in_range_x_y(which_unit.to_handle(), x, y, distance)
            };
        },
        {
            link: "IsUnitInRangeLoc"
            name: is_unit_in_range_loc
            raw: (which_unit: Handle<j_unit>, which_location: Handle<j_location>, distance: f32) -> bool;
            user: (which_unit: &Agent<j_unit>, which_location: &Agent<j_location>, distance: f32) -> bool {
                raw::is_unit_in_range_loc(which_unit.to_handle(), which_location.to_handle(), distance)
            };
        },
        {
            link: "IsUnitHidden"
            name: is_unit_hidden
            raw: (which_unit: Handle<j_unit>) -> bool;
            user: (which_unit: &Agent<j_unit>) -> bool {
                raw::is_unit_hidden(which_unit.to_handle())
            };
        },
        {
            link: "IsUnitIllusion"
            name: is_unit_illusion
            raw: (which_unit: Handle<j_unit>) -> bool;
            user: (which_unit: &Agent<j_unit>) -> bool {
                raw::is_unit_illusion(which_unit.to_handle())
            };
        },
        {
            link: "IsUnitInTransport"
            name: is_unit_in_transport
            raw: (which_unit: Handle<j_unit>, which_transport: Handle<j_unit>) -> bool;
            user: (which_unit: &Agent<j_unit>, which_transport: &Agent<j_unit>) -> bool {
                raw::is_unit_in_transport(which_unit.to_handle(), which_transport.to_handle())
            };
        },
        {
            link: "IsUnitLoaded"
            name: is_unit_loaded
            raw: (which_unit: Handle<j_unit>) -> bool;
            user: (which_unit: &Agent<j_unit>) -> bool {
                raw::is_unit_loaded(which_unit.to_handle())
            };
        },
        {
            link: "IsHeroUnitId"
            name: is_hero_unit_id
            raw: (unit_id: i32) -> bool;
            user: (unit_id: i32) -> bool {
                raw::is_hero_unit_id(unit_id)
            };
        },
        {
            link: "IsUnitIdType"
            name: is_unit_id_type
            raw: (unit_id: i32, which_unit_type: i32) -> bool;
            user: (unit_id: i32, which_unit_type: i32) -> bool {
                raw::is_unit_id_type(unit_id, which_unit_type)
            };
        },
        {
            link: "UnitShareVision"
            name: unit_share_vision
            raw: (which_unit: Handle<j_unit>, which_player: Handle<j_player>, share: bool);
            user: (which_unit: &Agent<j_unit>, which_player: &Agent<j_player>, share: bool) {
                raw::unit_share_vision(which_unit.to_handle(), which_player.to_handle(), share)
            };
        },
        {
            link: "UnitSuspendDecay"
            name: unit_suspend_decay
            raw: (which_unit: Handle<j_unit>, suspend: bool);
            user: (which_unit: &Agent<j_unit>, suspend: bool) {
                raw::unit_suspend_decay(which_unit.to_handle(), suspend)
            };
        },
        {
            link: "UnitAddType"
            name: unit_add_type
            raw: (which_unit: Handle<j_unit>, which_unit_type: i32) -> bool;
            user: (which_unit: &Agent<j_unit>, which_unit_type: i32) -> bool {
                raw::unit_add_type(which_unit.to_handle(), which_unit_type)
            };
        },
        {
            link: "UnitRemoveType"
            name: unit_remove_type
            raw: (which_unit: Handle<j_unit>, which_unit_type: i32) -> bool;
            user: (which_unit: &Agent<j_unit>, which_unit_type: i32) -> bool {
                raw::unit_remove_type(which_unit.to_handle(), which_unit_type)
            };
        },
        {
            link: "UnitAddAbility"
            name: unit_add_ability
            raw: (which_unit: Handle<j_unit>, ability_id: i32) -> bool;
            user: (which_unit: &Agent<j_unit>, ability_id: i32) -> bool {
                raw::unit_add_ability(which_unit.to_handle(), ability_id)
            };
        },
        {
            link: "UnitRemoveAbility"
            name: unit_remove_ability
            raw: (which_unit: Handle<j_unit>, ability_id: i32) -> bool;
            user: (which_unit: &Agent<j_unit>, ability_id: i32) -> bool {
                raw::unit_remove_ability(which_unit.to_handle(), ability_id)
            };
        },
        {
            link: "UnitMakeAbilityPermanent"
            name: unit_make_ability_permanent
            raw: (which_unit: Handle<j_unit>, permanent: bool, ability_id: i32) -> bool;
            user: (which_unit: &Agent<j_unit>, permanent: bool, ability_id: i32) -> bool {
                raw::unit_make_ability_permanent(which_unit.to_handle(), permanent, ability_id)
            };
        },
        {
            link: "UnitRemoveBuffs"
            name: unit_remove_buffs
            raw: (which_unit: Handle<j_unit>, remove_positive: bool, remove_negative: bool);
            user: (which_unit: &Agent<j_unit>, remove_positive: bool, remove_negative: bool) {
                raw::unit_remove_buffs(which_unit.to_handle(), remove_positive, remove_negative)
            };
        },
        {
            link: "UnitRemoveBuffsEx"
            name: unit_remove_buffs_ex
            raw: (which_unit: Handle<j_unit>, remove_positive: bool, remove_negative: bool, magic: bool, physical: bool, timed_life: bool, aura: bool, auto_dispel: bool);
            user: (which_unit: &Agent<j_unit>, remove_positive: bool, remove_negative: bool, magic: bool, physical: bool, timed_life: bool, aura: bool, auto_dispel: bool) {
                raw::unit_remove_buffs_ex(which_unit.to_handle(), remove_positive, remove_negative, magic, physical, timed_life, aura, auto_dispel)
            };
        },
        {
            link: "UnitHasBuffsEx"
            name: unit_has_buffs_ex
            raw: (which_unit: Handle<j_unit>, remove_positive: bool, remove_negative: bool, magic: bool, physical: bool, timed_life: bool, aura: bool, auto_dispel: bool) -> bool;
            user: (which_unit: &Agent<j_unit>, remove_positive: bool, remove_negative: bool, magic: bool, physical: bool, timed_life: bool, aura: bool, auto_dispel: bool) -> bool {
                raw::unit_has_buffs_ex(which_unit.to_handle(), remove_positive, remove_negative, magic, physical, timed_life, aura, auto_dispel)
            };
        },
        {
            link: "UnitCountBuffsEx"
            name: unit_count_buffs_ex
            raw: (which_unit: Handle<j_unit>, remove_positive: bool, remove_negative: bool, magic: bool, physical: bool, timed_life: bool, aura: bool, auto_dispel: bool) -> i32;
            user: (which_unit: &Agent<j_unit>, remove_positive: bool, remove_negative: bool, magic: bool, physical: bool, timed_life: bool, aura: bool, auto_dispel: bool) -> i32 {
                raw::unit_count_buffs_ex(which_unit.to_handle(), remove_positive, remove_negative, magic, physical, timed_life, aura, auto_dispel)
            };
        },
        {
            link: "UnitAddSleep"
            name: unit_add_sleep
            raw: (which_unit: Handle<j_unit>, add: bool);
            user: (which_unit: &Agent<j_unit>, add: bool) {
                raw::unit_add_sleep(which_unit.to_handle(), add)
            };
        },
        {
            link: "UnitCanSleep"
            name: unit_can_sleep
            raw: (which_unit: Handle<j_unit>) -> bool;
            user: (which_unit: &Agent<j_unit>) -> bool {
                raw::unit_can_sleep(which_unit.to_handle())
            };
        },
        {
            link: "UnitAddSleepPerm"
            name: unit_add_sleep_perm
            raw: (which_unit: Handle<j_unit>, add: bool);
            user: (which_unit: &Agent<j_unit>, add: bool) {
                raw::unit_add_sleep_perm(which_unit.to_handle(), add)
            };
        },
        {
            link: "UnitCanSleepPerm"
            name: unit_can_sleep_perm
            raw: (which_unit: Handle<j_unit>) -> bool;
            user: (which_unit: &Agent<j_unit>) -> bool {
                raw::unit_can_sleep_perm(which_unit.to_handle())
            };
        },
        {
            link: "UnitIsSleeping"
            name: unit_is_sleeping
            raw: (which_unit: Handle<j_unit>) -> bool;
            user: (which_unit: &Agent<j_unit>) -> bool {
                raw::unit_is_sleeping(which_unit.to_handle())
            };
        },
        {
            link: "UnitWakeUp"
            name: unit_wake_up
            raw: (which_unit: Handle<j_unit>);
            user: (which_unit: &Agent<j_unit>) {
                raw::unit_wake_up(which_unit.to_handle())
            };
        },
        {
            link: "UnitApplyTimedLife"
            name: unit_apply_timed_life
            raw: (which_unit: Handle<j_unit>, buff_id: i32, duration: f32);
            user: (which_unit: &Agent<j_unit>, buff_id: i32, duration: f32) {
                raw::unit_apply_timed_life(which_unit.to_handle(), buff_id, duration)
            };
        },
        {
            link: "UnitIgnoreAlarm"
            name: unit_ignore_alarm
            raw: (which_unit: Handle<j_unit>, flag: bool) -> bool;
            user: (which_unit: &Agent<j_unit>, flag: bool) -> bool {
                raw::unit_ignore_alarm(which_unit.to_handle(), flag)
            };
        },
        {
            link: "UnitIgnoreAlarmToggled"
            name: unit_ignore_alarm_toggled
            raw: (which_unit: Handle<j_unit>) -> bool;
            user: (which_unit: &Agent<j_unit>) -> bool {
                raw::unit_ignore_alarm_toggled(which_unit.to_handle())
            };
        },
        {
            link: "UnitResetCooldown"
            name: unit_reset_cooldown
            raw: (which_unit: Handle<j_unit>);
            user: (which_unit: &Agent<j_unit>) {
                raw::unit_reset_cooldown(which_unit.to_handle())
            };
        },
        {
            link: "UnitSetConstructionProgress"
            name: unit_set_construction_progress
            raw: (which_unit: Handle<j_unit>, construction_percentage: i32);
            user: (which_unit: &Agent<j_unit>, construction_percentage: i32) {
                raw::unit_set_construction_progress(which_unit.to_handle(), construction_percentage)
            };
        },
        {
            link: "UnitSetUpgradeProgress"
            name: unit_set_upgrade_progress
            raw: (which_unit: Handle<j_unit>, upgrade_percentage: i32);
            user: (which_unit: &Agent<j_unit>, upgrade_percentage: i32) {
                raw::unit_set_upgrade_progress(which_unit.to_handle(), upgrade_percentage)
            };
        },
        {
            link: "UnitPauseTimedLife"
            name: unit_pause_timed_life
            raw: (which_unit: Handle<j_unit>, flag: bool);
            user: (which_unit: &Agent<j_unit>, flag: bool) {
                raw::unit_pause_timed_life(which_unit.to_handle(), flag)
            };
        },
        {
            link: "UnitSetUsesAltIcon"
            name: unit_set_uses_alt_icon
            raw: (which_unit: Handle<j_unit>, flag: bool);
            user: (which_unit: &Agent<j_unit>, flag: bool) {
                raw::unit_set_uses_alt_icon(which_unit.to_handle(), flag)
            };
        },
        {
            link: "UnitDamagePoint"
            name: unit_damage_point
            raw: (which_unit: Handle<j_unit>, delay: f32, radius: f32, x: f32, y: f32, amount: f32, attack: bool, ranged: bool, attack_type: i32, damage_type: i32, weapon_type: i32) -> bool;
            user: (which_unit: &Agent<j_unit>, delay: f32, radius: f32, x: f32, y: f32, amount: f32, attack: bool, ranged: bool, attack_type: i32, damage_type: i32, weapon_type: i32) -> bool {
                raw::unit_damage_point(which_unit.to_handle(), delay, radius, x, y, amount, attack, ranged, attack_type, damage_type, weapon_type)
            };
        },
        {
            link: "UnitDamageTarget"
            name: unit_damage_target
            raw: (which_unit: Handle<j_unit>, target: Handle<j_widget>, amount: f32, attack: bool, ranged: bool, attack_type: i32, damage_type: i32, weapon_type: i32) -> bool;
            user: (which_unit: &Agent<j_unit>, target: &Agent<j_widget>, amount: f32, attack: bool, ranged: bool, attack_type: i32, damage_type: i32, weapon_type: i32) -> bool {
                raw::unit_damage_target(which_unit.to_handle(), target.to_handle(), amount, attack, ranged, attack_type, damage_type, weapon_type)
            };
        },
        {
            link: "IssueImmediateOrder"
            name: issue_immediate_order
            raw: (which_unit: Handle<j_unit>, order: *const i8) -> bool;
            user: (which_unit: &Agent<j_unit>, order: &CStr) -> bool {
                raw::issue_immediate_order(which_unit.to_handle(), order.as_ptr())
            };
        },
        {
            link: "IssueImmediateOrderById"
            name: issue_immediate_order_by_id
            raw: (which_unit: Handle<j_unit>, order: i32) -> bool;
            user: (which_unit: &Agent<j_unit>, order: i32) -> bool {
                raw::issue_immediate_order_by_id(which_unit.to_handle(), order)
            };
        },
        {
            link: "IssuePointOrder"
            name: issue_point_order
            raw: (which_unit: Handle<j_unit>, order: *const i8, x: f32, y: f32) -> bool;
            user: (which_unit: &Agent<j_unit>, order: &CStr, x: f32, y: f32) -> bool {
                raw::issue_point_order(which_unit.to_handle(), order.as_ptr(), x, y)
            };
        },
        {
            link: "IssuePointOrderLoc"
            name: issue_point_order_loc
            raw: (which_unit: Handle<j_unit>, order: *const i8, which_location: Handle<j_location>) -> bool;
            user: (which_unit: &Agent<j_unit>, order: &CStr, which_location: &Agent<j_location>) -> bool {
                raw::issue_point_order_loc(which_unit.to_handle(), order.as_ptr(), which_location.to_handle())
            };
        },
        {
            link: "IssuePointOrderById"
            name: issue_point_order_by_id
            raw: (which_unit: Handle<j_unit>, order: i32, x: f32, y: f32) -> bool;
            user: (which_unit: &Agent<j_unit>, order: i32, x: f32, y: f32) -> bool {
                raw::issue_point_order_by_id(which_unit.to_handle(), order, x, y)
            };
        },
        {
            link: "IssuePointOrderByIdLoc"
            name: issue_point_order_by_id_loc
            raw: (which_unit: Handle<j_unit>, order: i32, which_location: Handle<j_location>) -> bool;
            user: (which_unit: &Agent<j_unit>, order: i32, which_location: &Agent<j_location>) -> bool {
                raw::issue_point_order_by_id_loc(which_unit.to_handle(), order, which_location.to_handle())
            };
        },
        {
            link: "IssueTargetOrder"
            name: issue_target_order
            raw: (which_unit: Handle<j_unit>, order: *const i8, target_widget: Handle<j_widget>) -> bool;
            user: (which_unit: &Agent<j_unit>, order: &CStr, target_widget: &Agent<j_widget>) -> bool {
                raw::issue_target_order(which_unit.to_handle(), order.as_ptr(), target_widget.to_handle())
            };
        },
        {
            link: "IssueTargetOrderById"
            name: issue_target_order_by_id
            raw: (which_unit: Handle<j_unit>, order: i32, target_widget: Handle<j_widget>) -> bool;
            user: (which_unit: &Agent<j_unit>, order: i32, target_widget: &Agent<j_widget>) -> bool {
                raw::issue_target_order_by_id(which_unit.to_handle(), order, target_widget.to_handle())
            };
        },
        {
            link: "IssueInstantPointOrder"
            name: issue_instant_point_order
            raw: (which_unit: Handle<j_unit>, order: *const i8, x: f32, y: f32, instant_target_widget: Handle<j_widget>) -> bool;
            user: (which_unit: &Agent<j_unit>, order: &CStr, x: f32, y: f32, instant_target_widget: &Agent<j_widget>) -> bool {
                raw::issue_instant_point_order(which_unit.to_handle(), order.as_ptr(), x, y, instant_target_widget.to_handle())
            };
        },
        {
            link: "IssueInstantPointOrderById"
            name: issue_instant_point_order_by_id
            raw: (which_unit: Handle<j_unit>, order: i32, x: f32, y: f32, instant_target_widget: Handle<j_widget>) -> bool;
            user: (which_unit: &Agent<j_unit>, order: i32, x: f32, y: f32, instant_target_widget: &Agent<j_widget>) -> bool {
                raw::issue_instant_point_order_by_id(which_unit.to_handle(), order, x, y, instant_target_widget.to_handle())
            };
        },
        {
            link: "IssueInstantTargetOrder"
            name: issue_instant_target_order
            raw: (which_unit: Handle<j_unit>, order: *const i8, target_widget: Handle<j_widget>, instant_target_widget: Handle<j_widget>) -> bool;
            user: (which_unit: &Agent<j_unit>, order: &CStr, target_widget: &Agent<j_widget>, instant_target_widget: &Agent<j_widget>) -> bool {
                raw::issue_instant_target_order(which_unit.to_handle(), order.as_ptr(), target_widget.to_handle(), instant_target_widget.to_handle())
            };
        },
        {
            link: "IssueInstantTargetOrderById"
            name: issue_instant_target_order_by_id
            raw: (which_unit: Handle<j_unit>, order: i32, target_widget: Handle<j_widget>, instant_target_widget: Handle<j_widget>) -> bool;
            user: (which_unit: &Agent<j_unit>, order: i32, target_widget: &Agent<j_widget>, instant_target_widget: &Agent<j_widget>) -> bool {
                raw::issue_instant_target_order_by_id(which_unit.to_handle(), order, target_widget.to_handle(), instant_target_widget.to_handle())
            };
        },
        {
            link: "IssueBuildOrder"
            name: issue_build_order
            raw: (which_peon: Handle<j_unit>, unit_to_build: *const i8, x: f32, y: f32) -> bool;
            user: (which_peon: &Agent<j_unit>, unit_to_build: &CStr, x: f32, y: f32) -> bool {
                raw::issue_build_order(which_peon.to_handle(), unit_to_build.as_ptr(), x, y)
            };
        },
        {
            link: "IssueBuildOrderById"
            name: issue_build_order_by_id
            raw: (which_peon: Handle<j_unit>, unit_id: i32, x: f32, y: f32) -> bool;
            user: (which_peon: &Agent<j_unit>, unit_id: i32, x: f32, y: f32) -> bool {
                raw::issue_build_order_by_id(which_peon.to_handle(), unit_id, x, y)
            };
        },
        {
            link: "IssueNeutralImmediateOrder"
            name: issue_neutral_immediate_order
            raw: (for_which_player: Handle<j_player>, neutral_structure: Handle<j_unit>, unit_to_build: *const i8) -> bool;
            user: (for_which_player: &Agent<j_player>, neutral_structure: &Agent<j_unit>, unit_to_build: &CStr) -> bool {
                raw::issue_neutral_immediate_order(for_which_player.to_handle(), neutral_structure.to_handle(), unit_to_build.as_ptr())
            };
        },
        {
            link: "IssueNeutralImmediateOrderById"
            name: issue_neutral_immediate_order_by_id
            raw: (for_which_player: Handle<j_player>, neutral_structure: Handle<j_unit>, unit_id: i32) -> bool;
            user: (for_which_player: &Agent<j_player>, neutral_structure: &Agent<j_unit>, unit_id: i32) -> bool {
                raw::issue_neutral_immediate_order_by_id(for_which_player.to_handle(), neutral_structure.to_handle(), unit_id)
            };
        },
        {
            link: "IssueNeutralPointOrder"
            name: issue_neutral_point_order
            raw: (for_which_player: Handle<j_player>, neutral_structure: Handle<j_unit>, unit_to_build: *const i8, x: f32, y: f32) -> bool;
            user: (for_which_player: &Agent<j_player>, neutral_structure: &Agent<j_unit>, unit_to_build: &CStr, x: f32, y: f32) -> bool {
                raw::issue_neutral_point_order(for_which_player.to_handle(), neutral_structure.to_handle(), unit_to_build.as_ptr(), x, y)
            };
        },
        {
            link: "IssueNeutralPointOrderById"
            name: issue_neutral_point_order_by_id
            raw: (for_which_player: Handle<j_player>, neutral_structure: Handle<j_unit>, unit_id: i32, x: f32, y: f32) -> bool;
            user: (for_which_player: &Agent<j_player>, neutral_structure: &Agent<j_unit>, unit_id: i32, x: f32, y: f32) -> bool {
                raw::issue_neutral_point_order_by_id(for_which_player.to_handle(), neutral_structure.to_handle(), unit_id, x, y)
            };
        },
        {
            link: "IssueNeutralTargetOrder"
            name: issue_neutral_target_order
            raw: (for_which_player: Handle<j_player>, neutral_structure: Handle<j_unit>, unit_to_build: *const i8, target: Handle<j_widget>) -> bool;
            user: (for_which_player: &Agent<j_player>, neutral_structure: &Agent<j_unit>, unit_to_build: &CStr, target: &Agent<j_widget>) -> bool {
                raw::issue_neutral_target_order(for_which_player.to_handle(), neutral_structure.to_handle(), unit_to_build.as_ptr(), target.to_handle())
            };
        },
        {
            link: "IssueNeutralTargetOrderById"
            name: issue_neutral_target_order_by_id
            raw: (for_which_player: Handle<j_player>, neutral_structure: Handle<j_unit>, unit_id: i32, target: Handle<j_widget>) -> bool;
            user: (for_which_player: &Agent<j_player>, neutral_structure: &Agent<j_unit>, unit_id: i32, target: &Agent<j_widget>) -> bool {
                raw::issue_neutral_target_order_by_id(for_which_player.to_handle(), neutral_structure.to_handle(), unit_id, target.to_handle())
            };
        },
        {
            link: "GetUnitCurrentOrder"
            name: get_unit_current_order
            raw: (which_unit: Handle<j_unit>) -> i32;
            user: (which_unit: &Agent<j_unit>) -> i32 {
                raw::get_unit_current_order(which_unit.to_handle())
            };
        },
        {
            link: "SetResourceAmount"
            name: set_resource_amount
            raw: (which_unit: Handle<j_unit>, amount: i32);
            user: (which_unit: &Agent<j_unit>, amount: i32) {
                raw::set_resource_amount(which_unit.to_handle(), amount)
            };
        },
        {
            link: "AddResourceAmount"
            name: add_resource_amount
            raw: (which_unit: Handle<j_unit>, amount: i32);
            user: (which_unit: &Agent<j_unit>, amount: i32) {
                raw::add_resource_amount(which_unit.to_handle(), amount)
            };
        },
        {
            link: "GetResourceAmount"
            name: get_resource_amount
            raw: (which_unit: Handle<j_unit>) -> i32;
            user: (which_unit: &Agent<j_unit>) -> i32 {
                raw::get_resource_amount(which_unit.to_handle())
            };
        },
        {
            link: "WaygateGetDestinationX"
            name: waygate_get_destination_x
            raw: (waygate: Handle<j_unit>) -> f32;
            user: (waygate: &Agent<j_unit>) -> f32 {
                raw::waygate_get_destination_x(waygate.to_handle())
            };
        },
        {
            link: "WaygateGetDestinationY"
            name: waygate_get_destination_y
            raw: (waygate: Handle<j_unit>) -> f32;
            user: (waygate: &Agent<j_unit>) -> f32 {
                raw::waygate_get_destination_y(waygate.to_handle())
            };
        },
        {
            link: "WaygateSetDestination"
            name: waygate_set_destination
            raw: (waygate: Handle<j_unit>, x: f32, y: f32);
            user: (waygate: &Agent<j_unit>, x: f32, y: f32) {
                raw::waygate_set_destination(waygate.to_handle(), x, y)
            };
        },
        {
            link: "WaygateActivate"
            name: waygate_activate
            raw: (waygate: Handle<j_unit>, activate: bool);
            user: (waygate: &Agent<j_unit>, activate: bool) {
                raw::waygate_activate(waygate.to_handle(), activate)
            };
        },
        {
            link: "WaygateIsActive"
            name: waygate_is_active
            raw: (waygate: Handle<j_unit>) -> bool;
            user: (waygate: &Agent<j_unit>) -> bool {
                raw::waygate_is_active(waygate.to_handle())
            };
        },
        {
            link: "AddItemToAllStock"
            name: add_item_to_all_stock
            raw: (item_id: i32, current_stock: i32, stock_max: i32);
            user: (item_id: i32, current_stock: i32, stock_max: i32) {
                raw::add_item_to_all_stock(item_id, current_stock, stock_max)
            };
        },
        {
            link: "AddItemToStock"
            name: add_item_to_stock
            raw: (which_unit: Handle<j_unit>, item_id: i32, current_stock: i32, stock_max: i32);
            user: (which_unit: &Agent<j_unit>, item_id: i32, current_stock: i32, stock_max: i32) {
                raw::add_item_to_stock(which_unit.to_handle(), item_id, current_stock, stock_max)
            };
        },
        {
            link: "AddUnitToAllStock"
            name: add_unit_to_all_stock
            raw: (unit_id: i32, current_stock: i32, stock_max: i32);
            user: (unit_id: i32, current_stock: i32, stock_max: i32) {
                raw::add_unit_to_all_stock(unit_id, current_stock, stock_max)
            };
        },
        {
            link: "AddUnitToStock"
            name: add_unit_to_stock
            raw: (which_unit: Handle<j_unit>, unit_id: i32, current_stock: i32, stock_max: i32);
            user: (which_unit: &Agent<j_unit>, unit_id: i32, current_stock: i32, stock_max: i32) {
                raw::add_unit_to_stock(which_unit.to_handle(), unit_id, current_stock, stock_max)
            };
        },
        {
            link: "RemoveItemFromAllStock"
            name: remove_item_from_all_stock
            raw: (item_id: i32);
            user: (item_id: i32) {
                raw::remove_item_from_all_stock(item_id)
            };
        },
        {
            link: "RemoveItemFromStock"
            name: remove_item_from_stock
            raw: (which_unit: Handle<j_unit>, item_id: i32);
            user: (which_unit: &Agent<j_unit>, item_id: i32) {
                raw::remove_item_from_stock(which_unit.to_handle(), item_id)
            };
        },
        {
            link: "RemoveUnitFromAllStock"
            name: remove_unit_from_all_stock
            raw: (unit_id: i32);
            user: (unit_id: i32) {
                raw::remove_unit_from_all_stock(unit_id)
            };
        },
        {
            link: "RemoveUnitFromStock"
            name: remove_unit_from_stock
            raw: (which_unit: Handle<j_unit>, unit_id: i32);
            user: (which_unit: &Agent<j_unit>, unit_id: i32) {
                raw::remove_unit_from_stock(which_unit.to_handle(), unit_id)
            };
        },
        {
            link: "SetAllItemTypeSlots"
            name: set_all_item_type_slots
            raw: (slots: i32);
            user: (slots: i32) {
                raw::set_all_item_type_slots(slots)
            };
        },
        {
            link: "SetAllUnitTypeSlots"
            name: set_all_unit_type_slots
            raw: (slots: i32);
            user: (slots: i32) {
                raw::set_all_unit_type_slots(slots)
            };
        },
        {
            link: "SetItemTypeSlots"
            name: set_item_type_slots
            raw: (which_unit: Handle<j_unit>, slots: i32);
            user: (which_unit: &Agent<j_unit>, slots: i32) {
                raw::set_item_type_slots(which_unit.to_handle(), slots)
            };
        },
        {
            link: "SetUnitTypeSlots"
            name: set_unit_type_slots
            raw: (which_unit: Handle<j_unit>, slots: i32);
            user: (which_unit: &Agent<j_unit>, slots: i32) {
                raw::set_unit_type_slots(which_unit.to_handle(), slots)
            };
        },
        {
            link: "GetUnitUserData"
            name: get_unit_user_data
            raw: (which_unit: Handle<j_unit>) -> i32;
            user: (which_unit: &Agent<j_unit>) -> i32 {
                raw::get_unit_user_data(which_unit.to_handle())
            };
        },
        {
            link: "SetUnitUserData"
            name: set_unit_user_data
            raw: (which_unit: Handle<j_unit>, data: i32);
            user: (which_unit: &Agent<j_unit>, data: i32) {
                raw::set_unit_user_data(which_unit.to_handle(), data)
            };
        },
        {
            link: "Player"
            name: player
            raw: (number: i32) -> Handle<j_player>;
            user: (number: i32) -> Option<Weak<j_player>> {
                raw::player(number).to_weak()
            };
        },
        {
            link: "GetLocalPlayer"
            name: get_local_player
            raw: () -> Handle<j_player>;
            user: () -> Option<Weak<j_player>> {
                raw::get_local_player().to_weak()
            };
        },
        {
            link: "IsPlayerAlly"
            name: is_player_ally
            raw: (which_player: Handle<j_player>, other_player: Handle<j_player>) -> bool;
            user: (which_player: &Agent<j_player>, other_player: &Agent<j_player>) -> bool {
                raw::is_player_ally(which_player.to_handle(), other_player.to_handle())
            };
        },
        {
            link: "IsPlayerEnemy"
            name: is_player_enemy
            raw: (which_player: Handle<j_player>, other_player: Handle<j_player>) -> bool;
            user: (which_player: &Agent<j_player>, other_player: &Agent<j_player>) -> bool {
                raw::is_player_enemy(which_player.to_handle(), other_player.to_handle())
            };
        },
        {
            link: "IsPlayerInForce"
            name: is_player_in_force
            raw: (which_player: Handle<j_player>, which_force: Handle<j_force>) -> bool;
            user: (which_player: &Agent<j_player>, which_force: &Agent<j_force>) -> bool {
                raw::is_player_in_force(which_player.to_handle(), which_force.to_handle())
            };
        },
        {
            link: "IsPlayerObserver"
            name: is_player_observer
            raw: (which_player: Handle<j_player>) -> bool;
            user: (which_player: &Agent<j_player>) -> bool {
                raw::is_player_observer(which_player.to_handle())
            };
        },
        {
            link: "IsVisibleToPlayer"
            name: is_visible_to_player
            raw: (x: f32, y: f32, which_player: Handle<j_player>) -> bool;
            user: (x: f32, y: f32, which_player: &Agent<j_player>) -> bool {
                raw::is_visible_to_player(x, y, which_player.to_handle())
            };
        },
        {
            link: "IsLocationVisibleToPlayer"
            name: is_location_visible_to_player
            raw: (which_location: Handle<j_location>, which_player: Handle<j_player>) -> bool;
            user: (which_location: &Agent<j_location>, which_player: &Agent<j_player>) -> bool {
                raw::is_location_visible_to_player(which_location.to_handle(), which_player.to_handle())
            };
        },
        {
            link: "IsFoggedToPlayer"
            name: is_fogged_to_player
            raw: (x: f32, y: f32, which_player: Handle<j_player>) -> bool;
            user: (x: f32, y: f32, which_player: &Agent<j_player>) -> bool {
                raw::is_fogged_to_player(x, y, which_player.to_handle())
            };
        },
        {
            link: "IsLocationFoggedToPlayer"
            name: is_location_fogged_to_player
            raw: (which_location: Handle<j_location>, which_player: Handle<j_player>) -> bool;
            user: (which_location: &Agent<j_location>, which_player: &Agent<j_player>) -> bool {
                raw::is_location_fogged_to_player(which_location.to_handle(), which_player.to_handle())
            };
        },
        {
            link: "IsMaskedToPlayer"
            name: is_masked_to_player
            raw: (x: f32, y: f32, which_player: Handle<j_player>) -> bool;
            user: (x: f32, y: f32, which_player: &Agent<j_player>) -> bool {
                raw::is_masked_to_player(x, y, which_player.to_handle())
            };
        },
        {
            link: "IsLocationMaskedToPlayer"
            name: is_location_masked_to_player
            raw: (which_location: Handle<j_location>, which_player: Handle<j_player>) -> bool;
            user: (which_location: &Agent<j_location>, which_player: &Agent<j_player>) -> bool {
                raw::is_location_masked_to_player(which_location.to_handle(), which_player.to_handle())
            };
        },
        {
            link: "GetPlayerRace"
            name: get_player_race
            raw: (which_player: Handle<j_player>) -> i32;
            user: (which_player: &Agent<j_player>) -> i32 {
                raw::get_player_race(which_player.to_handle())
            };
        },
        {
            link: "GetPlayerId"
            name: get_player_id
            raw: (which_player: Handle<j_player>) -> i32;
            user: (which_player: &Agent<j_player>) -> i32 {
                raw::get_player_id(which_player.to_handle())
            };
        },
        {
            link: "GetPlayerUnitCount"
            name: get_player_unit_count
            raw: (which_player: Handle<j_player>, include_incomplete: bool) -> i32;
            user: (which_player: &Agent<j_player>, include_incomplete: bool) -> i32 {
                raw::get_player_unit_count(which_player.to_handle(), include_incomplete)
            };
        },
        {
            link: "GetPlayerTypedUnitCount"
            name: get_player_typed_unit_count
            raw: (which_player: Handle<j_player>, unit_name: *const i8, include_incomplete: bool, include_upgrades: bool) -> i32;
            user: (which_player: &Agent<j_player>, unit_name: &CStr, include_incomplete: bool, include_upgrades: bool) -> i32 {
                raw::get_player_typed_unit_count(which_player.to_handle(), unit_name.as_ptr(), include_incomplete, include_upgrades)
            };
        },
        {
            link: "GetPlayerStructureCount"
            name: get_player_structure_count
            raw: (which_player: Handle<j_player>, include_incomplete: bool) -> i32;
            user: (which_player: &Agent<j_player>, include_incomplete: bool) -> i32 {
                raw::get_player_structure_count(which_player.to_handle(), include_incomplete)
            };
        },
        {
            link: "GetPlayerState"
            name: get_player_state
            raw: (which_player: Handle<j_player>, which_player_state: i32) -> i32;
            user: (which_player: &Agent<j_player>, which_player_state: i32) -> i32 {
                raw::get_player_state(which_player.to_handle(), which_player_state)
            };
        },
        {
            link: "GetPlayerScore"
            name: get_player_score
            raw: (which_player: Handle<j_player>, which_player_score: i32) -> i32;
            user: (which_player: &Agent<j_player>, which_player_score: i32) -> i32 {
                raw::get_player_score(which_player.to_handle(), which_player_score)
            };
        },
        {
            link: "GetPlayerAlliance"
            name: get_player_alliance
            raw: (source_player: Handle<j_player>, other_player: Handle<j_player>, which_alliance_setting: i32) -> bool;
            user: (source_player: &Agent<j_player>, other_player: &Agent<j_player>, which_alliance_setting: i32) -> bool {
                raw::get_player_alliance(source_player.to_handle(), other_player.to_handle(), which_alliance_setting)
            };
        },
        {
            link: "GetPlayerHandicap"
            name: get_player_handicap
            raw: (which_player: Handle<j_player>) -> f32;
            user: (which_player: &Agent<j_player>) -> f32 {
                raw::get_player_handicap(which_player.to_handle())
            };
        },
        {
            link: "GetPlayerHandicapXP"
            name: get_player_handicap_x_p
            raw: (which_player: Handle<j_player>) -> f32;
            user: (which_player: &Agent<j_player>) -> f32 {
                raw::get_player_handicap_x_p(which_player.to_handle())
            };
        },
        {
            link: "GetPlayerHandicapReviveTime"
            name: get_player_handicap_revive_time
            raw: (which_player: Handle<j_player>) -> f32;
            user: (which_player: &Agent<j_player>) -> f32 {
                raw::get_player_handicap_revive_time(which_player.to_handle())
            };
        },
        {
            link: "GetPlayerHandicapDamage"
            name: get_player_handicap_damage
            raw: (which_player: Handle<j_player>) -> f32;
            user: (which_player: &Agent<j_player>) -> f32 {
                raw::get_player_handicap_damage(which_player.to_handle())
            };
        },
        {
            link: "SetPlayerHandicap"
            name: set_player_handicap
            raw: (which_player: Handle<j_player>, handicap: f32);
            user: (which_player: &Agent<j_player>, handicap: f32) {
                raw::set_player_handicap(which_player.to_handle(), handicap)
            };
        },
        {
            link: "SetPlayerHandicapXP"
            name: set_player_handicap_x_p
            raw: (which_player: Handle<j_player>, handicap: f32);
            user: (which_player: &Agent<j_player>, handicap: f32) {
                raw::set_player_handicap_x_p(which_player.to_handle(), handicap)
            };
        },
        {
            link: "SetPlayerHandicapReviveTime"
            name: set_player_handicap_revive_time
            raw: (which_player: Handle<j_player>, handicap: f32);
            user: (which_player: &Agent<j_player>, handicap: f32) {
                raw::set_player_handicap_revive_time(which_player.to_handle(), handicap)
            };
        },
        {
            link: "SetPlayerHandicapDamage"
            name: set_player_handicap_damage
            raw: (which_player: Handle<j_player>, handicap: f32);
            user: (which_player: &Agent<j_player>, handicap: f32) {
                raw::set_player_handicap_damage(which_player.to_handle(), handicap)
            };
        },
        {
            link: "SetPlayerTechMaxAllowed"
            name: set_player_tech_max_allowed
            raw: (which_player: Handle<j_player>, techid: i32, maximum: i32);
            user: (which_player: &Agent<j_player>, techid: i32, maximum: i32) {
                raw::set_player_tech_max_allowed(which_player.to_handle(), techid, maximum)
            };
        },
        {
            link: "GetPlayerTechMaxAllowed"
            name: get_player_tech_max_allowed
            raw: (which_player: Handle<j_player>, techid: i32) -> i32;
            user: (which_player: &Agent<j_player>, techid: i32) -> i32 {
                raw::get_player_tech_max_allowed(which_player.to_handle(), techid)
            };
        },
        {
            link: "AddPlayerTechResearched"
            name: add_player_tech_researched
            raw: (which_player: Handle<j_player>, techid: i32, levels: i32);
            user: (which_player: &Agent<j_player>, techid: i32, levels: i32) {
                raw::add_player_tech_researched(which_player.to_handle(), techid, levels)
            };
        },
        {
            link: "SetPlayerTechResearched"
            name: set_player_tech_researched
            raw: (which_player: Handle<j_player>, techid: i32, set_to_level: i32);
            user: (which_player: &Agent<j_player>, techid: i32, set_to_level: i32) {
                raw::set_player_tech_researched(which_player.to_handle(), techid, set_to_level)
            };
        },
        {
            link: "GetPlayerTechResearched"
            name: get_player_tech_researched
            raw: (which_player: Handle<j_player>, techid: i32, specificonly: bool) -> bool;
            user: (which_player: &Agent<j_player>, techid: i32, specificonly: bool) -> bool {
                raw::get_player_tech_researched(which_player.to_handle(), techid, specificonly)
            };
        },
        {
            link: "GetPlayerTechCount"
            name: get_player_tech_count
            raw: (which_player: Handle<j_player>, techid: i32, specificonly: bool) -> i32;
            user: (which_player: &Agent<j_player>, techid: i32, specificonly: bool) -> i32 {
                raw::get_player_tech_count(which_player.to_handle(), techid, specificonly)
            };
        },
        {
            link: "SetPlayerUnitsOwner"
            name: set_player_units_owner
            raw: (which_player: Handle<j_player>, new_owner: i32);
            user: (which_player: &Agent<j_player>, new_owner: i32) {
                raw::set_player_units_owner(which_player.to_handle(), new_owner)
            };
        },
        {
            link: "CripplePlayer"
            name: cripple_player
            raw: (which_player: Handle<j_player>, to_which_players: Handle<j_force>, flag: bool);
            user: (which_player: &Agent<j_player>, to_which_players: &Agent<j_force>, flag: bool) {
                raw::cripple_player(which_player.to_handle(), to_which_players.to_handle(), flag)
            };
        },
        {
            link: "SetPlayerAbilityAvailable"
            name: set_player_ability_available
            raw: (which_player: Handle<j_player>, abilid: i32, avail: bool);
            user: (which_player: &Agent<j_player>, abilid: i32, avail: bool) {
                raw::set_player_ability_available(which_player.to_handle(), abilid, avail)
            };
        },
        {
            link: "SetPlayerState"
            name: set_player_state
            raw: (which_player: Handle<j_player>, which_player_state: i32, value: i32);
            user: (which_player: &Agent<j_player>, which_player_state: i32, value: i32) {
                raw::set_player_state(which_player.to_handle(), which_player_state, value)
            };
        },
        {
            link: "RemovePlayer"
            name: remove_player
            raw: (which_player: Handle<j_player>, game_result: i32);
            user: (which_player: &Agent<j_player>, game_result: i32) {
                raw::remove_player(which_player.to_handle(), game_result)
            };
        },
        {
            link: "CachePlayerHeroData"
            name: cache_player_hero_data
            raw: (which_player: Handle<j_player>);
            user: (which_player: &Agent<j_player>) {
                raw::cache_player_hero_data(which_player.to_handle())
            };
        },
        {
            link: "SetFogStateRect"
            name: set_fog_state_rect
            raw: (for_which_player: Handle<j_player>, which_state: i32, _where: Handle<j_rect>, use_shared_vision: bool);
            user: (for_which_player: &Agent<j_player>, which_state: i32, _where: &Agent<j_rect>, use_shared_vision: bool) {
                raw::set_fog_state_rect(for_which_player.to_handle(), which_state, _where.to_handle(), use_shared_vision)
            };
        },
        {
            link: "SetFogStateRadius"
            name: set_fog_state_radius
            raw: (for_which_player: Handle<j_player>, which_state: i32, centerx: f32, center_y: f32, radius: f32, use_shared_vision: bool);
            user: (for_which_player: &Agent<j_player>, which_state: i32, centerx: f32, center_y: f32, radius: f32, use_shared_vision: bool) {
                raw::set_fog_state_radius(for_which_player.to_handle(), which_state, centerx, center_y, radius, use_shared_vision)
            };
        },
        {
            link: "SetFogStateRadiusLoc"
            name: set_fog_state_radius_loc
            raw: (for_which_player: Handle<j_player>, which_state: i32, center: Handle<j_location>, radius: f32, use_shared_vision: bool);
            user: (for_which_player: &Agent<j_player>, which_state: i32, center: &Agent<j_location>, radius: f32, use_shared_vision: bool) {
                raw::set_fog_state_radius_loc(for_which_player.to_handle(), which_state, center.to_handle(), radius, use_shared_vision)
            };
        },
        {
            link: "FogMaskEnable"
            name: fog_mask_enable
            raw: (enable: bool);
            user: (enable: bool) {
                raw::fog_mask_enable(enable)
            };
        },
        {
            link: "IsFogMaskEnabled"
            name: is_fog_mask_enabled
            raw: () -> bool;
            user: () -> bool {
                raw::is_fog_mask_enabled()
            };
        },
        {
            link: "FogEnable"
            name: fog_enable
            raw: (enable: bool);
            user: (enable: bool) {
                raw::fog_enable(enable)
            };
        },
        {
            link: "IsFogEnabled"
            name: is_fog_enabled
            raw: () -> bool;
            user: () -> bool {
                raw::is_fog_enabled()
            };
        },
        {
            link: "CreateFogModifierRect"
            name: create_fog_modifier_rect
            raw: (for_which_player: Handle<j_player>, which_state: i32, _where: Handle<j_rect>, use_shared_vision: bool, after_units: bool) -> Handle<j_fogmodifier>;
            user: (for_which_player: &Agent<j_player>, which_state: i32, _where: &Agent<j_rect>, use_shared_vision: bool, after_units: bool) -> Option<Weak<j_fogmodifier>> {
                raw::create_fog_modifier_rect(for_which_player.to_handle(), which_state, _where.to_handle(), use_shared_vision, after_units).to_weak()
            };
        },
        {
            link: "CreateFogModifierRadius"
            name: create_fog_modifier_radius
            raw: (for_which_player: Handle<j_player>, which_state: i32, centerx: f32, center_y: f32, radius: f32, use_shared_vision: bool, after_units: bool) -> Handle<j_fogmodifier>;
            user: (for_which_player: &Agent<j_player>, which_state: i32, centerx: f32, center_y: f32, radius: f32, use_shared_vision: bool, after_units: bool) -> Option<Weak<j_fogmodifier>> {
                raw::create_fog_modifier_radius(for_which_player.to_handle(), which_state, centerx, center_y, radius, use_shared_vision, after_units).to_weak()
            };
        },
        {
            link: "CreateFogModifierRadiusLoc"
            name: create_fog_modifier_radius_loc
            raw: (for_which_player: Handle<j_player>, which_state: i32, center: Handle<j_location>, radius: f32, use_shared_vision: bool, after_units: bool) -> Handle<j_fogmodifier>;
            user: (for_which_player: &Agent<j_player>, which_state: i32, center: &Agent<j_location>, radius: f32, use_shared_vision: bool, after_units: bool) -> Option<Weak<j_fogmodifier>> {
                raw::create_fog_modifier_radius_loc(for_which_player.to_handle(), which_state, center.to_handle(), radius, use_shared_vision, after_units).to_weak()
            };
        },
        {
            link: "DestroyFogModifier"
            name: destroy_fog_modifier
            raw: (which_fog_modifier: Handle<j_fogmodifier>);
            user: (which_fog_modifier: &Agent<j_fogmodifier>) {
                raw::destroy_fog_modifier(which_fog_modifier.to_handle())
            };
        },
        {
            link: "FogModifierStart"
            name: fog_modifier_start
            raw: (which_fog_modifier: Handle<j_fogmodifier>);
            user: (which_fog_modifier: &Agent<j_fogmodifier>) {
                raw::fog_modifier_start(which_fog_modifier.to_handle())
            };
        },
        {
            link: "FogModifierStop"
            name: fog_modifier_stop
            raw: (which_fog_modifier: Handle<j_fogmodifier>);
            user: (which_fog_modifier: &Agent<j_fogmodifier>) {
                raw::fog_modifier_stop(which_fog_modifier.to_handle())
            };
        },
        {
            link: "VersionGet"
            name: version_get
            raw: () -> i32;
            user: () -> i32 {
                raw::version_get()
            };
        },
        {
            link: "VersionCompatible"
            name: version_compatible
            raw: (which_version: i32) -> bool;
            user: (which_version: i32) -> bool {
                raw::version_compatible(which_version)
            };
        },
        {
            link: "VersionSupported"
            name: version_supported
            raw: (which_version: i32) -> bool;
            user: (which_version: i32) -> bool {
                raw::version_supported(which_version)
            };
        },
        {
            link: "EndGame"
            name: end_game
            raw: (do_score_screen: bool);
            user: (do_score_screen: bool) {
                raw::end_game(do_score_screen)
            };
        },
        {
            link: "ChangeLevel"
            name: change_level
            raw: (new_level: *const i8, do_score_screen: bool);
            user: (new_level: &CStr, do_score_screen: bool) {
                raw::change_level(new_level.as_ptr(), do_score_screen)
            };
        },
        {
            link: "RestartGame"
            name: restart_game
            raw: (do_score_screen: bool);
            user: (do_score_screen: bool) {
                raw::restart_game(do_score_screen)
            };
        },
        {
            link: "ReloadGame"
            name: reload_game
            raw: ();
            user: () {
                raw::reload_game()
            };
        },
        {
            link: "SetCampaignMenuRace"
            name: set_campaign_menu_race
            raw: (r: i32);
            user: (r: i32) {
                raw::set_campaign_menu_race(r)
            };
        },
        {
            link: "SetCampaignMenuRaceEx"
            name: set_campaign_menu_race_ex
            raw: (campaign_index: i32);
            user: (campaign_index: i32) {
                raw::set_campaign_menu_race_ex(campaign_index)
            };
        },
        {
            link: "ForceCampaignSelectScreen"
            name: force_campaign_select_screen
            raw: ();
            user: () {
                raw::force_campaign_select_screen()
            };
        },
        {
            link: "LoadGame"
            name: load_game
            raw: (save_file_name: *const i8, do_score_screen: bool);
            user: (save_file_name: &CStr, do_score_screen: bool) {
                raw::load_game(save_file_name.as_ptr(), do_score_screen)
            };
        },
        {
            link: "SaveGame"
            name: save_game
            raw: (save_file_name: *const i8);
            user: (save_file_name: &CStr) {
                raw::save_game(save_file_name.as_ptr())
            };
        },
        {
            link: "RenameSaveDirectory"
            name: rename_save_directory
            raw: (source_dir_name: *const i8, dest_dir_name: *const i8) -> bool;
            user: (source_dir_name: &CStr, dest_dir_name: &CStr) -> bool {
                raw::rename_save_directory(source_dir_name.as_ptr(), dest_dir_name.as_ptr())
            };
        },
        {
            link: "RemoveSaveDirectory"
            name: remove_save_directory
            raw: (source_dir_name: *const i8) -> bool;
            user: (source_dir_name: &CStr) -> bool {
                raw::remove_save_directory(source_dir_name.as_ptr())
            };
        },
        {
            link: "CopySaveGame"
            name: copy_save_game
            raw: (source_save_name: *const i8, dest_save_name: *const i8) -> bool;
            user: (source_save_name: &CStr, dest_save_name: &CStr) -> bool {
                raw::copy_save_game(source_save_name.as_ptr(), dest_save_name.as_ptr())
            };
        },
        {
            link: "SaveGameExists"
            name: save_game_exists
            raw: (save_name: *const i8) -> bool;
            user: (save_name: &CStr) -> bool {
                raw::save_game_exists(save_name.as_ptr())
            };
        },
        {
            link: "SetMaxCheckpointSaves"
            name: set_max_checkpoint_saves
            raw: (max_checkpoint_saves: i32);
            user: (max_checkpoint_saves: i32) {
                raw::set_max_checkpoint_saves(max_checkpoint_saves)
            };
        },
        {
            link: "SaveGameCheckpoint"
            name: save_game_checkpoint
            raw: (save_file_name: *const i8, show_window: bool);
            user: (save_file_name: &CStr, show_window: bool) {
                raw::save_game_checkpoint(save_file_name.as_ptr(), show_window)
            };
        },
        {
            link: "SyncSelections"
            name: sync_selections
            raw: ();
            user: () {
                raw::sync_selections()
            };
        },
        {
            link: "SetFloatGameState"
            name: set_float_game_state
            raw: (which_float_game_state: i32, value: f32);
            user: (which_float_game_state: i32, value: f32) {
                raw::set_float_game_state(which_float_game_state, value)
            };
        },
        {
            link: "GetFloatGameState"
            name: get_float_game_state
            raw: (which_float_game_state: i32) -> f32;
            user: (which_float_game_state: i32) -> f32 {
                raw::get_float_game_state(which_float_game_state)
            };
        },
        {
            link: "SetIntegerGameState"
            name: set_integer_game_state
            raw: (which_integer_game_state: i32, value: i32);
            user: (which_integer_game_state: i32, value: i32) {
                raw::set_integer_game_state(which_integer_game_state, value)
            };
        },
        {
            link: "GetIntegerGameState"
            name: get_integer_game_state
            raw: (which_integer_game_state: i32) -> i32;
            user: (which_integer_game_state: i32) -> i32 {
                raw::get_integer_game_state(which_integer_game_state)
            };
        },
        {
            link: "SetTutorialCleared"
            name: set_tutorial_cleared
            raw: (cleared: bool);
            user: (cleared: bool) {
                raw::set_tutorial_cleared(cleared)
            };
        },
        {
            link: "SetMissionAvailable"
            name: set_mission_available
            raw: (campaign_number: i32, mission_number: i32, available: bool);
            user: (campaign_number: i32, mission_number: i32, available: bool) {
                raw::set_mission_available(campaign_number, mission_number, available)
            };
        },
        {
            link: "SetCampaignAvailable"
            name: set_campaign_available
            raw: (campaign_number: i32, available: bool);
            user: (campaign_number: i32, available: bool) {
                raw::set_campaign_available(campaign_number, available)
            };
        },
        {
            link: "SetOpCinematicAvailable"
            name: set_op_cinematic_available
            raw: (campaign_number: i32, available: bool);
            user: (campaign_number: i32, available: bool) {
                raw::set_op_cinematic_available(campaign_number, available)
            };
        },
        {
            link: "SetEdCinematicAvailable"
            name: set_ed_cinematic_available
            raw: (campaign_number: i32, available: bool);
            user: (campaign_number: i32, available: bool) {
                raw::set_ed_cinematic_available(campaign_number, available)
            };
        },
        {
            link: "GetDefaultDifficulty"
            name: get_default_difficulty
            raw: () -> i32;
            user: () -> i32 {
                raw::get_default_difficulty()
            };
        },
        {
            link: "SetDefaultDifficulty"
            name: set_default_difficulty
            raw: (g: i32);
            user: (g: i32) {
                raw::set_default_difficulty(g)
            };
        },
        {
            link: "SetCustomCampaignButtonVisible"
            name: set_custom_campaign_button_visible
            raw: (which_button: i32, visible: bool);
            user: (which_button: i32, visible: bool) {
                raw::set_custom_campaign_button_visible(which_button, visible)
            };
        },
        {
            link: "GetCustomCampaignButtonVisible"
            name: get_custom_campaign_button_visible
            raw: (which_button: i32) -> bool;
            user: (which_button: i32) -> bool {
                raw::get_custom_campaign_button_visible(which_button)
            };
        },
        {
            link: "DoNotSaveReplay"
            name: do_not_save_replay
            raw: ();
            user: () {
                raw::do_not_save_replay()
            };
        },
        {
            link: "DialogCreate"
            name: dialog_create
            raw: () -> Handle<j_dialog>;
            user: () -> Option<Weak<j_dialog>> {
                raw::dialog_create().to_weak()
            };
        },
        {
            link: "DialogDestroy"
            name: dialog_destroy
            raw: (which_dialog: Handle<j_dialog>);
            user: (which_dialog: &Agent<j_dialog>) {
                raw::dialog_destroy(which_dialog.to_handle())
            };
        },
        {
            link: "DialogClear"
            name: dialog_clear
            raw: (which_dialog: Handle<j_dialog>);
            user: (which_dialog: &Agent<j_dialog>) {
                raw::dialog_clear(which_dialog.to_handle())
            };
        },
        {
            link: "DialogSetMessage"
            name: dialog_set_message
            raw: (which_dialog: Handle<j_dialog>, message_text: *const i8);
            user: (which_dialog: &Agent<j_dialog>, message_text: &CStr) {
                raw::dialog_set_message(which_dialog.to_handle(), message_text.as_ptr())
            };
        },
        {
            link: "DialogAddButton"
            name: dialog_add_button
            raw: (which_dialog: Handle<j_dialog>, button_text: *const i8, hotkey: i32) -> Handle<j_button>;
            user: (which_dialog: &Agent<j_dialog>, button_text: &CStr, hotkey: i32) -> Option<Weak<j_button>> {
                raw::dialog_add_button(which_dialog.to_handle(), button_text.as_ptr(), hotkey).to_weak()
            };
        },
        {
            link: "DialogAddQuitButton"
            name: dialog_add_quit_button
            raw: (which_dialog: Handle<j_dialog>, do_score_screen: bool, button_text: *const i8, hotkey: i32) -> Handle<j_button>;
            user: (which_dialog: &Agent<j_dialog>, do_score_screen: bool, button_text: &CStr, hotkey: i32) -> Option<Weak<j_button>> {
                raw::dialog_add_quit_button(which_dialog.to_handle(), do_score_screen, button_text.as_ptr(), hotkey).to_weak()
            };
        },
        {
            link: "DialogDisplay"
            name: dialog_display
            raw: (which_player: Handle<j_player>, which_dialog: Handle<j_dialog>, flag: bool);
            user: (which_player: &Agent<j_player>, which_dialog: &Agent<j_dialog>, flag: bool) {
                raw::dialog_display(which_player.to_handle(), which_dialog.to_handle(), flag)
            };
        },
        {
            link: "ReloadGameCachesFromDisk"
            name: reload_game_caches_from_disk
            raw: () -> bool;
            user: () -> bool {
                raw::reload_game_caches_from_disk()
            };
        },
        {
            link: "InitGameCache"
            name: init_game_cache
            raw: (campaign_file: *const i8) -> Handle<j_gamecache>;
            user: (campaign_file: &CStr) -> Option<Weak<j_gamecache>> {
                raw::init_game_cache(campaign_file.as_ptr()).to_weak()
            };
        },
        {
            link: "SaveGameCache"
            name: save_game_cache
            raw: (which_cache: Handle<j_gamecache>) -> bool;
            user: (which_cache: &Agent<j_gamecache>) -> bool {
                raw::save_game_cache(which_cache.to_handle())
            };
        },
        {
            link: "StoreInteger"
            name: store_integer
            raw: (cache: Handle<j_gamecache>, mission_key: *const i8, key: *const i8, value: i32);
            user: (cache: &Agent<j_gamecache>, mission_key: &CStr, key: &CStr, value: i32) {
                raw::store_integer(cache.to_handle(), mission_key.as_ptr(), key.as_ptr(), value)
            };
        },
        {
            link: "StoreReal"
            name: store_real
            raw: (cache: Handle<j_gamecache>, mission_key: *const i8, key: *const i8, value: f32);
            user: (cache: &Agent<j_gamecache>, mission_key: &CStr, key: &CStr, value: f32) {
                raw::store_real(cache.to_handle(), mission_key.as_ptr(), key.as_ptr(), value)
            };
        },
        {
            link: "StoreBoolean"
            name: store_boolean
            raw: (cache: Handle<j_gamecache>, mission_key: *const i8, key: *const i8, value: bool);
            user: (cache: &Agent<j_gamecache>, mission_key: &CStr, key: &CStr, value: bool) {
                raw::store_boolean(cache.to_handle(), mission_key.as_ptr(), key.as_ptr(), value)
            };
        },
        {
            link: "StoreUnit"
            name: store_unit
            raw: (cache: Handle<j_gamecache>, mission_key: *const i8, key: *const i8, which_unit: Handle<j_unit>) -> bool;
            user: (cache: &Agent<j_gamecache>, mission_key: &CStr, key: &CStr, which_unit: &Agent<j_unit>) -> bool {
                raw::store_unit(cache.to_handle(), mission_key.as_ptr(), key.as_ptr(), which_unit.to_handle())
            };
        },
        {
            link: "StoreString"
            name: store_string
            raw: (cache: Handle<j_gamecache>, mission_key: *const i8, key: *const i8, value: *const i8) -> bool;
            user: (cache: &Agent<j_gamecache>, mission_key: &CStr, key: &CStr, value: &CStr) -> bool {
                raw::store_string(cache.to_handle(), mission_key.as_ptr(), key.as_ptr(), value.as_ptr())
            };
        },
        {
            link: "SyncStoredInteger"
            name: sync_stored_integer
            raw: (cache: Handle<j_gamecache>, mission_key: *const i8, key: *const i8);
            user: (cache: &Agent<j_gamecache>, mission_key: &CStr, key: &CStr) {
                raw::sync_stored_integer(cache.to_handle(), mission_key.as_ptr(), key.as_ptr())
            };
        },
        {
            link: "SyncStoredReal"
            name: sync_stored_real
            raw: (cache: Handle<j_gamecache>, mission_key: *const i8, key: *const i8);
            user: (cache: &Agent<j_gamecache>, mission_key: &CStr, key: &CStr) {
                raw::sync_stored_real(cache.to_handle(), mission_key.as_ptr(), key.as_ptr())
            };
        },
        {
            link: "SyncStoredBoolean"
            name: sync_stored_boolean
            raw: (cache: Handle<j_gamecache>, mission_key: *const i8, key: *const i8);
            user: (cache: &Agent<j_gamecache>, mission_key: &CStr, key: &CStr) {
                raw::sync_stored_boolean(cache.to_handle(), mission_key.as_ptr(), key.as_ptr())
            };
        },
        {
            link: "SyncStoredUnit"
            name: sync_stored_unit
            raw: (cache: Handle<j_gamecache>, mission_key: *const i8, key: *const i8);
            user: (cache: &Agent<j_gamecache>, mission_key: &CStr, key: &CStr) {
                raw::sync_stored_unit(cache.to_handle(), mission_key.as_ptr(), key.as_ptr())
            };
        },
        {
            link: "SyncStoredString"
            name: sync_stored_string
            raw: (cache: Handle<j_gamecache>, mission_key: *const i8, key: *const i8);
            user: (cache: &Agent<j_gamecache>, mission_key: &CStr, key: &CStr) {
                raw::sync_stored_string(cache.to_handle(), mission_key.as_ptr(), key.as_ptr())
            };
        },
        {
            link: "HaveStoredInteger"
            name: have_stored_integer
            raw: (cache: Handle<j_gamecache>, mission_key: *const i8, key: *const i8) -> bool;
            user: (cache: &Agent<j_gamecache>, mission_key: &CStr, key: &CStr) -> bool {
                raw::have_stored_integer(cache.to_handle(), mission_key.as_ptr(), key.as_ptr())
            };
        },
        {
            link: "HaveStoredReal"
            name: have_stored_real
            raw: (cache: Handle<j_gamecache>, mission_key: *const i8, key: *const i8) -> bool;
            user: (cache: &Agent<j_gamecache>, mission_key: &CStr, key: &CStr) -> bool {
                raw::have_stored_real(cache.to_handle(), mission_key.as_ptr(), key.as_ptr())
            };
        },
        {
            link: "HaveStoredBoolean"
            name: have_stored_boolean
            raw: (cache: Handle<j_gamecache>, mission_key: *const i8, key: *const i8) -> bool;
            user: (cache: &Agent<j_gamecache>, mission_key: &CStr, key: &CStr) -> bool {
                raw::have_stored_boolean(cache.to_handle(), mission_key.as_ptr(), key.as_ptr())
            };
        },
        {
            link: "HaveStoredUnit"
            name: have_stored_unit
            raw: (cache: Handle<j_gamecache>, mission_key: *const i8, key: *const i8) -> bool;
            user: (cache: &Agent<j_gamecache>, mission_key: &CStr, key: &CStr) -> bool {
                raw::have_stored_unit(cache.to_handle(), mission_key.as_ptr(), key.as_ptr())
            };
        },
        {
            link: "HaveStoredString"
            name: have_stored_string
            raw: (cache: Handle<j_gamecache>, mission_key: *const i8, key: *const i8) -> bool;
            user: (cache: &Agent<j_gamecache>, mission_key: &CStr, key: &CStr) -> bool {
                raw::have_stored_string(cache.to_handle(), mission_key.as_ptr(), key.as_ptr())
            };
        },
        {
            link: "FlushGameCache"
            name: flush_game_cache
            raw: (cache: Handle<j_gamecache>);
            user: (cache: &Agent<j_gamecache>) {
                raw::flush_game_cache(cache.to_handle())
            };
        },
        {
            link: "FlushStoredMission"
            name: flush_stored_mission
            raw: (cache: Handle<j_gamecache>, mission_key: *const i8);
            user: (cache: &Agent<j_gamecache>, mission_key: &CStr) {
                raw::flush_stored_mission(cache.to_handle(), mission_key.as_ptr())
            };
        },
        {
            link: "FlushStoredInteger"
            name: flush_stored_integer
            raw: (cache: Handle<j_gamecache>, mission_key: *const i8, key: *const i8);
            user: (cache: &Agent<j_gamecache>, mission_key: &CStr, key: &CStr) {
                raw::flush_stored_integer(cache.to_handle(), mission_key.as_ptr(), key.as_ptr())
            };
        },
        {
            link: "FlushStoredReal"
            name: flush_stored_real
            raw: (cache: Handle<j_gamecache>, mission_key: *const i8, key: *const i8);
            user: (cache: &Agent<j_gamecache>, mission_key: &CStr, key: &CStr) {
                raw::flush_stored_real(cache.to_handle(), mission_key.as_ptr(), key.as_ptr())
            };
        },
        {
            link: "FlushStoredBoolean"
            name: flush_stored_boolean
            raw: (cache: Handle<j_gamecache>, mission_key: *const i8, key: *const i8);
            user: (cache: &Agent<j_gamecache>, mission_key: &CStr, key: &CStr) {
                raw::flush_stored_boolean(cache.to_handle(), mission_key.as_ptr(), key.as_ptr())
            };
        },
        {
            link: "FlushStoredUnit"
            name: flush_stored_unit
            raw: (cache: Handle<j_gamecache>, mission_key: *const i8, key: *const i8);
            user: (cache: &Agent<j_gamecache>, mission_key: &CStr, key: &CStr) {
                raw::flush_stored_unit(cache.to_handle(), mission_key.as_ptr(), key.as_ptr())
            };
        },
        {
            link: "FlushStoredString"
            name: flush_stored_string
            raw: (cache: Handle<j_gamecache>, mission_key: *const i8, key: *const i8);
            user: (cache: &Agent<j_gamecache>, mission_key: &CStr, key: &CStr) {
                raw::flush_stored_string(cache.to_handle(), mission_key.as_ptr(), key.as_ptr())
            };
        },
        {
            link: "GetStoredInteger"
            name: get_stored_integer
            raw: (cache: Handle<j_gamecache>, mission_key: *const i8, key: *const i8) -> i32;
            user: (cache: &Agent<j_gamecache>, mission_key: &CStr, key: &CStr) -> i32 {
                raw::get_stored_integer(cache.to_handle(), mission_key.as_ptr(), key.as_ptr())
            };
        },
        {
            link: "GetStoredReal"
            name: get_stored_real
            raw: (cache: Handle<j_gamecache>, mission_key: *const i8, key: *const i8) -> f32;
            user: (cache: &Agent<j_gamecache>, mission_key: &CStr, key: &CStr) -> f32 {
                raw::get_stored_real(cache.to_handle(), mission_key.as_ptr(), key.as_ptr())
            };
        },
        {
            link: "GetStoredBoolean"
            name: get_stored_boolean
            raw: (cache: Handle<j_gamecache>, mission_key: *const i8, key: *const i8) -> bool;
            user: (cache: &Agent<j_gamecache>, mission_key: &CStr, key: &CStr) -> bool {
                raw::get_stored_boolean(cache.to_handle(), mission_key.as_ptr(), key.as_ptr())
            };
        },
        {
            link: "GetStoredString"
            name: get_stored_string
            raw: (cache: Handle<j_gamecache>, mission_key: *const i8, key: *const i8, out: *mut i8, out_len: *mut usize);
            user: (cache: &Agent<j_gamecache>, mission_key: &CStr, key: &CStr) -> String {
                raw::get_stored_string(cache.to_handle(), mission_key.as_ptr(), key.as_ptr(), stringret::ptr(), stringret::reset_len()); stringret::read_string()
            };
        },
        {
            link: "RestoreUnit"
            name: restore_unit
            raw: (cache: Handle<j_gamecache>, mission_key: *const i8, key: *const i8, for_which_player: Handle<j_player>, x: f32, y: f32, facing: f32) -> Handle<j_unit>;
            user: (cache: &Agent<j_gamecache>, mission_key: &CStr, key: &CStr, for_which_player: &Agent<j_player>, x: f32, y: f32, facing: f32) -> Option<Weak<j_unit>> {
                raw::restore_unit(cache.to_handle(), mission_key.as_ptr(), key.as_ptr(), for_which_player.to_handle(), x, y, facing).to_weak()
            };
        },
        {
            link: "InitHashtable"
            name: init_hashtable
            raw: () -> Handle<j_hashtable>;
            user: () -> Option<Weak<j_hashtable>> {
                raw::init_hashtable().to_weak()
            };
        },
        {
            link: "SaveInteger"
            name: save_integer
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32, value: i32);
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32, value: i32) {
                raw::save_integer(table.to_handle(), parent_key, child_key, value)
            };
        },
        {
            link: "SaveReal"
            name: save_real
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32, value: f32);
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32, value: f32) {
                raw::save_real(table.to_handle(), parent_key, child_key, value)
            };
        },
        {
            link: "SaveBoolean"
            name: save_boolean
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32, value: bool);
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32, value: bool) {
                raw::save_boolean(table.to_handle(), parent_key, child_key, value)
            };
        },
        {
            link: "SaveStr"
            name: save_str
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32, value: *const i8) -> bool;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32, value: &CStr) -> bool {
                raw::save_str(table.to_handle(), parent_key, child_key, value.as_ptr())
            };
        },
        {
            link: "SavePlayerHandle"
            name: save_player_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32, which_player: Handle<j_player>) -> bool;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32, which_player: &Agent<j_player>) -> bool {
                raw::save_player_handle(table.to_handle(), parent_key, child_key, which_player.to_handle())
            };
        },
        {
            link: "SaveWidgetHandle"
            name: save_widget_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32, which_widget: Handle<j_widget>) -> bool;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32, which_widget: &Agent<j_widget>) -> bool {
                raw::save_widget_handle(table.to_handle(), parent_key, child_key, which_widget.to_handle())
            };
        },
        {
            link: "SaveDestructableHandle"
            name: save_destructable_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32, which_destructable: Handle<j_destructable>) -> bool;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32, which_destructable: &Agent<j_destructable>) -> bool {
                raw::save_destructable_handle(table.to_handle(), parent_key, child_key, which_destructable.to_handle())
            };
        },
        {
            link: "SaveItemHandle"
            name: save_item_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32, which_item: Handle<j_item>) -> bool;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32, which_item: &Agent<j_item>) -> bool {
                raw::save_item_handle(table.to_handle(), parent_key, child_key, which_item.to_handle())
            };
        },
        {
            link: "SaveUnitHandle"
            name: save_unit_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32, which_unit: Handle<j_unit>) -> bool;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32, which_unit: &Agent<j_unit>) -> bool {
                raw::save_unit_handle(table.to_handle(), parent_key, child_key, which_unit.to_handle())
            };
        },
        {
            link: "SaveAbilityHandle"
            name: save_ability_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32, which_ability: Handle<j_ability>) -> bool;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32, which_ability: &Agent<j_ability>) -> bool {
                raw::save_ability_handle(table.to_handle(), parent_key, child_key, which_ability.to_handle())
            };
        },
        {
            link: "SaveTimerHandle"
            name: save_timer_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32, which_timer: Handle<j_timer>) -> bool;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32, which_timer: &Agent<j_timer>) -> bool {
                raw::save_timer_handle(table.to_handle(), parent_key, child_key, which_timer.to_handle())
            };
        },
        {
            link: "SaveTriggerHandle"
            name: save_trigger_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32, which_trigger: Handle<j_trigger>) -> bool;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32, which_trigger: &Agent<j_trigger>) -> bool {
                raw::save_trigger_handle(table.to_handle(), parent_key, child_key, which_trigger.to_handle())
            };
        },
        {
            link: "SaveTriggerConditionHandle"
            name: save_trigger_condition_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32, which_triggercondition: Handle<j_triggercondition>) -> bool;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32, which_triggercondition: &Agent<j_triggercondition>) -> bool {
                raw::save_trigger_condition_handle(table.to_handle(), parent_key, child_key, which_triggercondition.to_handle())
            };
        },
        {
            link: "SaveTriggerActionHandle"
            name: save_trigger_action_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32, which_triggeraction: Handle<j_triggeraction>) -> bool;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32, which_triggeraction: Handle<j_triggeraction>) -> bool {
                raw::save_trigger_action_handle(table.to_handle(), parent_key, child_key, which_triggeraction)
            };
        },
        {
            link: "SaveTriggerEventHandle"
            name: save_trigger_event_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32, which_event: Handle<j_event>) -> bool;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32, which_event: &Agent<j_event>) -> bool {
                raw::save_trigger_event_handle(table.to_handle(), parent_key, child_key, which_event.to_handle())
            };
        },
        {
            link: "SaveForceHandle"
            name: save_force_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32, which_force: Handle<j_force>) -> bool;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32, which_force: &Agent<j_force>) -> bool {
                raw::save_force_handle(table.to_handle(), parent_key, child_key, which_force.to_handle())
            };
        },
        {
            link: "SaveGroupHandle"
            name: save_group_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32, which_group: Handle<j_group>) -> bool;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32, which_group: &Agent<j_group>) -> bool {
                raw::save_group_handle(table.to_handle(), parent_key, child_key, which_group.to_handle())
            };
        },
        {
            link: "SaveLocationHandle"
            name: save_location_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32, which_location: Handle<j_location>) -> bool;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32, which_location: &Agent<j_location>) -> bool {
                raw::save_location_handle(table.to_handle(), parent_key, child_key, which_location.to_handle())
            };
        },
        {
            link: "SaveRectHandle"
            name: save_rect_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32, which_rect: Handle<j_rect>) -> bool;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32, which_rect: &Agent<j_rect>) -> bool {
                raw::save_rect_handle(table.to_handle(), parent_key, child_key, which_rect.to_handle())
            };
        },
        {
            link: "SaveBooleanExprHandle"
            name: save_boolean_expr_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32, which_boolexpr: Handle<j_boolexpr>) -> bool;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32, which_boolexpr: &Agent<j_boolexpr>) -> bool {
                raw::save_boolean_expr_handle(table.to_handle(), parent_key, child_key, which_boolexpr.to_handle())
            };
        },
        {
            link: "SaveSoundHandle"
            name: save_sound_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32, which_sound: Handle<j_sound>) -> bool;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32, which_sound: &Agent<j_sound>) -> bool {
                raw::save_sound_handle(table.to_handle(), parent_key, child_key, which_sound.to_handle())
            };
        },
        {
            link: "SaveEffectHandle"
            name: save_effect_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32, which_effect: Handle<j_effect>) -> bool;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32, which_effect: &Agent<j_effect>) -> bool {
                raw::save_effect_handle(table.to_handle(), parent_key, child_key, which_effect.to_handle())
            };
        },
        {
            link: "SaveUnitPoolHandle"
            name: save_unit_pool_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32, which_unitpool: Handle<j_unitpool>) -> bool;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32, which_unitpool: Handle<j_unitpool>) -> bool {
                raw::save_unit_pool_handle(table.to_handle(), parent_key, child_key, which_unitpool)
            };
        },
        {
            link: "SaveItemPoolHandle"
            name: save_item_pool_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32, which_itempool: Handle<j_itempool>) -> bool;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32, which_itempool: Handle<j_itempool>) -> bool {
                raw::save_item_pool_handle(table.to_handle(), parent_key, child_key, which_itempool)
            };
        },
        {
            link: "SaveQuestHandle"
            name: save_quest_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32, which_quest: Handle<j_quest>) -> bool;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32, which_quest: &Agent<j_quest>) -> bool {
                raw::save_quest_handle(table.to_handle(), parent_key, child_key, which_quest.to_handle())
            };
        },
        {
            link: "SaveQuestItemHandle"
            name: save_quest_item_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32, which_questitem: Handle<j_questitem>) -> bool;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32, which_questitem: &Agent<j_questitem>) -> bool {
                raw::save_quest_item_handle(table.to_handle(), parent_key, child_key, which_questitem.to_handle())
            };
        },
        {
            link: "SaveDefeatConditionHandle"
            name: save_defeat_condition_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32, which_defeatcondition: Handle<j_defeatcondition>) -> bool;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32, which_defeatcondition: &Agent<j_defeatcondition>) -> bool {
                raw::save_defeat_condition_handle(table.to_handle(), parent_key, child_key, which_defeatcondition.to_handle())
            };
        },
        {
            link: "SaveTimerDialogHandle"
            name: save_timer_dialog_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32, which_timerdialog: Handle<j_timerdialog>) -> bool;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32, which_timerdialog: &Agent<j_timerdialog>) -> bool {
                raw::save_timer_dialog_handle(table.to_handle(), parent_key, child_key, which_timerdialog.to_handle())
            };
        },
        {
            link: "SaveLeaderboardHandle"
            name: save_leaderboard_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32, which_leaderboard: Handle<j_leaderboard>) -> bool;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32, which_leaderboard: &Agent<j_leaderboard>) -> bool {
                raw::save_leaderboard_handle(table.to_handle(), parent_key, child_key, which_leaderboard.to_handle())
            };
        },
        {
            link: "SaveMultiboardHandle"
            name: save_multiboard_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32, which_multiboard: Handle<j_multiboard>) -> bool;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32, which_multiboard: &Agent<j_multiboard>) -> bool {
                raw::save_multiboard_handle(table.to_handle(), parent_key, child_key, which_multiboard.to_handle())
            };
        },
        {
            link: "SaveMultiboardItemHandle"
            name: save_multiboard_item_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32, which_multiboarditem: Handle<j_multiboarditem>) -> bool;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32, which_multiboarditem: &Agent<j_multiboarditem>) -> bool {
                raw::save_multiboard_item_handle(table.to_handle(), parent_key, child_key, which_multiboarditem.to_handle())
            };
        },
        {
            link: "SaveTrackableHandle"
            name: save_trackable_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32, which_trackable: Handle<j_trackable>) -> bool;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32, which_trackable: &Agent<j_trackable>) -> bool {
                raw::save_trackable_handle(table.to_handle(), parent_key, child_key, which_trackable.to_handle())
            };
        },
        {
            link: "SaveDialogHandle"
            name: save_dialog_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32, which_dialog: Handle<j_dialog>) -> bool;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32, which_dialog: &Agent<j_dialog>) -> bool {
                raw::save_dialog_handle(table.to_handle(), parent_key, child_key, which_dialog.to_handle())
            };
        },
        {
            link: "SaveButtonHandle"
            name: save_button_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32, which_button: Handle<j_button>) -> bool;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32, which_button: &Agent<j_button>) -> bool {
                raw::save_button_handle(table.to_handle(), parent_key, child_key, which_button.to_handle())
            };
        },
        {
            link: "SaveTextTagHandle"
            name: save_text_tag_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32, which_texttag: Handle<j_texttag>) -> bool;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32, which_texttag: Handle<j_texttag>) -> bool {
                raw::save_text_tag_handle(table.to_handle(), parent_key, child_key, which_texttag)
            };
        },
        {
            link: "SaveLightningHandle"
            name: save_lightning_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32, which_lightning: Handle<j_lightning>) -> bool;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32, which_lightning: Handle<j_lightning>) -> bool {
                raw::save_lightning_handle(table.to_handle(), parent_key, child_key, which_lightning)
            };
        },
        {
            link: "SaveImageHandle"
            name: save_image_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32, which_image: Handle<j_image>) -> bool;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32, which_image: Handle<j_image>) -> bool {
                raw::save_image_handle(table.to_handle(), parent_key, child_key, which_image)
            };
        },
        {
            link: "SaveUbersplatHandle"
            name: save_ubersplat_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32, which_ubersplat: Handle<j_ubersplat>) -> bool;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32, which_ubersplat: Handle<j_ubersplat>) -> bool {
                raw::save_ubersplat_handle(table.to_handle(), parent_key, child_key, which_ubersplat)
            };
        },
        {
            link: "SaveRegionHandle"
            name: save_region_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32, which_region: Handle<j_region>) -> bool;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32, which_region: &Agent<j_region>) -> bool {
                raw::save_region_handle(table.to_handle(), parent_key, child_key, which_region.to_handle())
            };
        },
        {
            link: "SaveFogStateHandle"
            name: save_fog_state_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32, which_fog_state: i32) -> bool;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32, which_fog_state: i32) -> bool {
                raw::save_fog_state_handle(table.to_handle(), parent_key, child_key, which_fog_state)
            };
        },
        {
            link: "SaveFogModifierHandle"
            name: save_fog_modifier_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32, which_fog_modifier: Handle<j_fogmodifier>) -> bool;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32, which_fog_modifier: &Agent<j_fogmodifier>) -> bool {
                raw::save_fog_modifier_handle(table.to_handle(), parent_key, child_key, which_fog_modifier.to_handle())
            };
        },
        {
            link: "SaveAgentHandle"
            name: save_agent_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32, which_agent: Handle<j_agent>) -> bool;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32, which_agent: &Agent<j_agent>) -> bool {
                raw::save_agent_handle(table.to_handle(), parent_key, child_key, which_agent.to_handle())
            };
        },
        {
            link: "SaveHashtableHandle"
            name: save_hashtable_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32, which_hashtable: Handle<j_hashtable>) -> bool;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32, which_hashtable: &Agent<j_hashtable>) -> bool {
                raw::save_hashtable_handle(table.to_handle(), parent_key, child_key, which_hashtable.to_handle())
            };
        },
        {
            link: "SaveFrameHandle"
            name: save_frame_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32, which_frame_handle: Handle<j_framehandle>) -> bool;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32, which_frame_handle: Handle<j_framehandle>) -> bool {
                raw::save_frame_handle(table.to_handle(), parent_key, child_key, which_frame_handle)
            };
        },
        {
            link: "LoadInteger"
            name: load_integer
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32) -> i32;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32) -> i32 {
                raw::load_integer(table.to_handle(), parent_key, child_key)
            };
        },
        {
            link: "LoadReal"
            name: load_real
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32) -> f32;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32) -> f32 {
                raw::load_real(table.to_handle(), parent_key, child_key)
            };
        },
        {
            link: "LoadBoolean"
            name: load_boolean
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32) -> bool;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32) -> bool {
                raw::load_boolean(table.to_handle(), parent_key, child_key)
            };
        },
        {
            link: "LoadStr"
            name: load_str
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32, out: *mut i8, out_len: *mut usize);
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32) -> String {
                raw::load_str(table.to_handle(), parent_key, child_key, stringret::ptr(), stringret::reset_len()); stringret::read_string()
            };
        },
        {
            link: "LoadPlayerHandle"
            name: load_player_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32) -> Handle<j_player>;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32) -> Option<Weak<j_player>> {
                raw::load_player_handle(table.to_handle(), parent_key, child_key).to_weak()
            };
        },
        {
            link: "LoadWidgetHandle"
            name: load_widget_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32) -> Handle<j_widget>;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32) -> Option<Weak<j_widget>> {
                raw::load_widget_handle(table.to_handle(), parent_key, child_key).to_weak()
            };
        },
        {
            link: "LoadDestructableHandle"
            name: load_destructable_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32) -> Handle<j_destructable>;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32) -> Option<Weak<j_destructable>> {
                raw::load_destructable_handle(table.to_handle(), parent_key, child_key).to_weak()
            };
        },
        {
            link: "LoadItemHandle"
            name: load_item_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32) -> Handle<j_item>;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32) -> Option<Weak<j_item>> {
                raw::load_item_handle(table.to_handle(), parent_key, child_key).to_weak()
            };
        },
        {
            link: "LoadUnitHandle"
            name: load_unit_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32) -> Handle<j_unit>;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32) -> Option<Weak<j_unit>> {
                raw::load_unit_handle(table.to_handle(), parent_key, child_key).to_weak()
            };
        },
        {
            link: "LoadAbilityHandle"
            name: load_ability_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32) -> Handle<j_ability>;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32) -> Option<Weak<j_ability>> {
                raw::load_ability_handle(table.to_handle(), parent_key, child_key).to_weak()
            };
        },
        {
            link: "LoadTimerHandle"
            name: load_timer_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32) -> Handle<j_timer>;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32) -> Option<Weak<j_timer>> {
                raw::load_timer_handle(table.to_handle(), parent_key, child_key).to_weak()
            };
        },
        {
            link: "LoadTriggerHandle"
            name: load_trigger_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32) -> Handle<j_trigger>;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32) -> Option<Weak<j_trigger>> {
                raw::load_trigger_handle(table.to_handle(), parent_key, child_key).to_weak()
            };
        },
        {
            link: "LoadTriggerConditionHandle"
            name: load_trigger_condition_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32) -> Handle<j_triggercondition>;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32) -> Option<Weak<j_triggercondition>> {
                raw::load_trigger_condition_handle(table.to_handle(), parent_key, child_key).to_weak()
            };
        },
        {
            link: "LoadTriggerActionHandle"
            name: load_trigger_action_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32) -> Handle<j_triggeraction>;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32) -> Handle<j_triggeraction> {
                raw::load_trigger_action_handle(table.to_handle(), parent_key, child_key)
            };
        },
        {
            link: "LoadTriggerEventHandle"
            name: load_trigger_event_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32) -> Handle<j_event>;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32) -> Option<Weak<j_event>> {
                raw::load_trigger_event_handle(table.to_handle(), parent_key, child_key).to_weak()
            };
        },
        {
            link: "LoadForceHandle"
            name: load_force_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32) -> Handle<j_force>;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32) -> Option<Weak<j_force>> {
                raw::load_force_handle(table.to_handle(), parent_key, child_key).to_weak()
            };
        },
        {
            link: "LoadGroupHandle"
            name: load_group_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32) -> Handle<j_group>;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32) -> Option<Weak<j_group>> {
                raw::load_group_handle(table.to_handle(), parent_key, child_key).to_weak()
            };
        },
        {
            link: "LoadLocationHandle"
            name: load_location_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32) -> Handle<j_location>;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32) -> Option<Weak<j_location>> {
                raw::load_location_handle(table.to_handle(), parent_key, child_key).to_weak()
            };
        },
        {
            link: "LoadRectHandle"
            name: load_rect_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32) -> Handle<j_rect>;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32) -> Option<Weak<j_rect>> {
                raw::load_rect_handle(table.to_handle(), parent_key, child_key).to_weak()
            };
        },
        {
            link: "LoadBooleanExprHandle"
            name: load_boolean_expr_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32) -> Handle<j_boolexpr>;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32) -> Option<Weak<j_boolexpr>> {
                raw::load_boolean_expr_handle(table.to_handle(), parent_key, child_key).to_weak()
            };
        },
        {
            link: "LoadSoundHandle"
            name: load_sound_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32) -> Handle<j_sound>;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32) -> Option<Weak<j_sound>> {
                raw::load_sound_handle(table.to_handle(), parent_key, child_key).to_weak()
            };
        },
        {
            link: "LoadEffectHandle"
            name: load_effect_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32) -> Handle<j_effect>;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32) -> Option<Weak<j_effect>> {
                raw::load_effect_handle(table.to_handle(), parent_key, child_key).to_weak()
            };
        },
        {
            link: "LoadUnitPoolHandle"
            name: load_unit_pool_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32) -> Handle<j_unitpool>;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32) -> Handle<j_unitpool> {
                raw::load_unit_pool_handle(table.to_handle(), parent_key, child_key)
            };
        },
        {
            link: "LoadItemPoolHandle"
            name: load_item_pool_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32) -> Handle<j_itempool>;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32) -> Handle<j_itempool> {
                raw::load_item_pool_handle(table.to_handle(), parent_key, child_key)
            };
        },
        {
            link: "LoadQuestHandle"
            name: load_quest_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32) -> Handle<j_quest>;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32) -> Option<Weak<j_quest>> {
                raw::load_quest_handle(table.to_handle(), parent_key, child_key).to_weak()
            };
        },
        {
            link: "LoadQuestItemHandle"
            name: load_quest_item_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32) -> Handle<j_questitem>;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32) -> Option<Weak<j_questitem>> {
                raw::load_quest_item_handle(table.to_handle(), parent_key, child_key).to_weak()
            };
        },
        {
            link: "LoadDefeatConditionHandle"
            name: load_defeat_condition_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32) -> Handle<j_defeatcondition>;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32) -> Option<Weak<j_defeatcondition>> {
                raw::load_defeat_condition_handle(table.to_handle(), parent_key, child_key).to_weak()
            };
        },
        {
            link: "LoadTimerDialogHandle"
            name: load_timer_dialog_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32) -> Handle<j_timerdialog>;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32) -> Option<Weak<j_timerdialog>> {
                raw::load_timer_dialog_handle(table.to_handle(), parent_key, child_key).to_weak()
            };
        },
        {
            link: "LoadLeaderboardHandle"
            name: load_leaderboard_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32) -> Handle<j_leaderboard>;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32) -> Option<Weak<j_leaderboard>> {
                raw::load_leaderboard_handle(table.to_handle(), parent_key, child_key).to_weak()
            };
        },
        {
            link: "LoadMultiboardHandle"
            name: load_multiboard_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32) -> Handle<j_multiboard>;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32) -> Option<Weak<j_multiboard>> {
                raw::load_multiboard_handle(table.to_handle(), parent_key, child_key).to_weak()
            };
        },
        {
            link: "LoadMultiboardItemHandle"
            name: load_multiboard_item_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32) -> Handle<j_multiboarditem>;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32) -> Option<Weak<j_multiboarditem>> {
                raw::load_multiboard_item_handle(table.to_handle(), parent_key, child_key).to_weak()
            };
        },
        {
            link: "LoadTrackableHandle"
            name: load_trackable_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32) -> Handle<j_trackable>;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32) -> Option<Weak<j_trackable>> {
                raw::load_trackable_handle(table.to_handle(), parent_key, child_key).to_weak()
            };
        },
        {
            link: "LoadDialogHandle"
            name: load_dialog_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32) -> Handle<j_dialog>;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32) -> Option<Weak<j_dialog>> {
                raw::load_dialog_handle(table.to_handle(), parent_key, child_key).to_weak()
            };
        },
        {
            link: "LoadButtonHandle"
            name: load_button_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32) -> Handle<j_button>;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32) -> Option<Weak<j_button>> {
                raw::load_button_handle(table.to_handle(), parent_key, child_key).to_weak()
            };
        },
        {
            link: "LoadTextTagHandle"
            name: load_text_tag_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32) -> Handle<j_texttag>;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32) -> Handle<j_texttag> {
                raw::load_text_tag_handle(table.to_handle(), parent_key, child_key)
            };
        },
        {
            link: "LoadLightningHandle"
            name: load_lightning_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32) -> Handle<j_lightning>;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32) -> Handle<j_lightning> {
                raw::load_lightning_handle(table.to_handle(), parent_key, child_key)
            };
        },
        {
            link: "LoadImageHandle"
            name: load_image_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32) -> Handle<j_image>;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32) -> Handle<j_image> {
                raw::load_image_handle(table.to_handle(), parent_key, child_key)
            };
        },
        {
            link: "LoadUbersplatHandle"
            name: load_ubersplat_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32) -> Handle<j_ubersplat>;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32) -> Handle<j_ubersplat> {
                raw::load_ubersplat_handle(table.to_handle(), parent_key, child_key)
            };
        },
        {
            link: "LoadRegionHandle"
            name: load_region_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32) -> Handle<j_region>;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32) -> Option<Weak<j_region>> {
                raw::load_region_handle(table.to_handle(), parent_key, child_key).to_weak()
            };
        },
        {
            link: "LoadFogStateHandle"
            name: load_fog_state_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32) -> i32;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32) -> i32 {
                raw::load_fog_state_handle(table.to_handle(), parent_key, child_key)
            };
        },
        {
            link: "LoadFogModifierHandle"
            name: load_fog_modifier_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32) -> Handle<j_fogmodifier>;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32) -> Option<Weak<j_fogmodifier>> {
                raw::load_fog_modifier_handle(table.to_handle(), parent_key, child_key).to_weak()
            };
        },
        {
            link: "LoadHashtableHandle"
            name: load_hashtable_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32) -> Handle<j_hashtable>;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32) -> Option<Weak<j_hashtable>> {
                raw::load_hashtable_handle(table.to_handle(), parent_key, child_key).to_weak()
            };
        },
        {
            link: "LoadFrameHandle"
            name: load_frame_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32) -> Handle<j_framehandle>;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32) -> Handle<j_framehandle> {
                raw::load_frame_handle(table.to_handle(), parent_key, child_key)
            };
        },
        {
            link: "HaveSavedInteger"
            name: have_saved_integer
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32) -> bool;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32) -> bool {
                raw::have_saved_integer(table.to_handle(), parent_key, child_key)
            };
        },
        {
            link: "HaveSavedReal"
            name: have_saved_real
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32) -> bool;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32) -> bool {
                raw::have_saved_real(table.to_handle(), parent_key, child_key)
            };
        },
        {
            link: "HaveSavedBoolean"
            name: have_saved_boolean
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32) -> bool;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32) -> bool {
                raw::have_saved_boolean(table.to_handle(), parent_key, child_key)
            };
        },
        {
            link: "HaveSavedString"
            name: have_saved_string
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32) -> bool;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32) -> bool {
                raw::have_saved_string(table.to_handle(), parent_key, child_key)
            };
        },
        {
            link: "HaveSavedHandle"
            name: have_saved_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32) -> bool;
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32) -> bool {
                raw::have_saved_handle(table.to_handle(), parent_key, child_key)
            };
        },
        {
            link: "RemoveSavedInteger"
            name: remove_saved_integer
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32);
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32) {
                raw::remove_saved_integer(table.to_handle(), parent_key, child_key)
            };
        },
        {
            link: "RemoveSavedReal"
            name: remove_saved_real
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32);
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32) {
                raw::remove_saved_real(table.to_handle(), parent_key, child_key)
            };
        },
        {
            link: "RemoveSavedBoolean"
            name: remove_saved_boolean
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32);
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32) {
                raw::remove_saved_boolean(table.to_handle(), parent_key, child_key)
            };
        },
        {
            link: "RemoveSavedString"
            name: remove_saved_string
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32);
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32) {
                raw::remove_saved_string(table.to_handle(), parent_key, child_key)
            };
        },
        {
            link: "RemoveSavedHandle"
            name: remove_saved_handle
            raw: (table: Handle<j_hashtable>, parent_key: i32, child_key: i32);
            user: (table: &Agent<j_hashtable>, parent_key: i32, child_key: i32) {
                raw::remove_saved_handle(table.to_handle(), parent_key, child_key)
            };
        },
        {
            link: "FlushParentHashtable"
            name: flush_parent_hashtable
            raw: (table: Handle<j_hashtable>);
            user: (table: &Agent<j_hashtable>) {
                raw::flush_parent_hashtable(table.to_handle())
            };
        },
        {
            link: "FlushChildHashtable"
            name: flush_child_hashtable
            raw: (table: Handle<j_hashtable>, parent_key: i32);
            user: (table: &Agent<j_hashtable>, parent_key: i32) {
                raw::flush_child_hashtable(table.to_handle(), parent_key)
            };
        },
        {
            link: "GetRandomInt"
            name: get_random_int
            raw: (low_bound: i32, high_bound: i32) -> i32;
            user: (low_bound: i32, high_bound: i32) -> i32 {
                raw::get_random_int(low_bound, high_bound)
            };
        },
        {
            link: "GetRandomReal"
            name: get_random_real
            raw: (low_bound: f32, high_bound: f32) -> f32;
            user: (low_bound: f32, high_bound: f32) -> f32 {
                raw::get_random_real(low_bound, high_bound)
            };
        },
        {
            link: "CreateUnitPool"
            name: create_unit_pool
            raw: () -> Handle<j_unitpool>;
            user: () -> Handle<j_unitpool> {
                raw::create_unit_pool()
            };
        },
        {
            link: "DestroyUnitPool"
            name: destroy_unit_pool
            raw: (which_pool: Handle<j_unitpool>);
            user: (which_pool: Handle<j_unitpool>) {
                raw::destroy_unit_pool(which_pool)
            };
        },
        {
            link: "UnitPoolAddUnitType"
            name: unit_pool_add_unit_type
            raw: (which_pool: Handle<j_unitpool>, unit_id: i32, weight: f32);
            user: (which_pool: Handle<j_unitpool>, unit_id: i32, weight: f32) {
                raw::unit_pool_add_unit_type(which_pool, unit_id, weight)
            };
        },
        {
            link: "UnitPoolRemoveUnitType"
            name: unit_pool_remove_unit_type
            raw: (which_pool: Handle<j_unitpool>, unit_id: i32);
            user: (which_pool: Handle<j_unitpool>, unit_id: i32) {
                raw::unit_pool_remove_unit_type(which_pool, unit_id)
            };
        },
        {
            link: "PlaceRandomUnit"
            name: place_random_unit
            raw: (which_pool: Handle<j_unitpool>, for_which_player: Handle<j_player>, x: f32, y: f32, facing: f32) -> Handle<j_unit>;
            user: (which_pool: Handle<j_unitpool>, for_which_player: &Agent<j_player>, x: f32, y: f32, facing: f32) -> Option<Weak<j_unit>> {
                raw::place_random_unit(which_pool, for_which_player.to_handle(), x, y, facing).to_weak()
            };
        },
        {
            link: "CreateItemPool"
            name: create_item_pool
            raw: () -> Handle<j_itempool>;
            user: () -> Handle<j_itempool> {
                raw::create_item_pool()
            };
        },
        {
            link: "DestroyItemPool"
            name: destroy_item_pool
            raw: (which_item_pool: Handle<j_itempool>);
            user: (which_item_pool: Handle<j_itempool>) {
                raw::destroy_item_pool(which_item_pool)
            };
        },
        {
            link: "ItemPoolAddItemType"
            name: item_pool_add_item_type
            raw: (which_item_pool: Handle<j_itempool>, item_id: i32, weight: f32);
            user: (which_item_pool: Handle<j_itempool>, item_id: i32, weight: f32) {
                raw::item_pool_add_item_type(which_item_pool, item_id, weight)
            };
        },
        {
            link: "ItemPoolRemoveItemType"
            name: item_pool_remove_item_type
            raw: (which_item_pool: Handle<j_itempool>, item_id: i32);
            user: (which_item_pool: Handle<j_itempool>, item_id: i32) {
                raw::item_pool_remove_item_type(which_item_pool, item_id)
            };
        },
        {
            link: "PlaceRandomItem"
            name: place_random_item
            raw: (which_item_pool: Handle<j_itempool>, x: f32, y: f32) -> Handle<j_item>;
            user: (which_item_pool: Handle<j_itempool>, x: f32, y: f32) -> Option<Weak<j_item>> {
                raw::place_random_item(which_item_pool, x, y).to_weak()
            };
        },
        {
            link: "ChooseRandomCreep"
            name: choose_random_creep
            raw: (level: i32) -> i32;
            user: (level: i32) -> i32 {
                raw::choose_random_creep(level)
            };
        },
        {
            link: "ChooseRandomNPBuilding"
            name: choose_random_n_p_building
            raw: () -> i32;
            user: () -> i32 {
                raw::choose_random_n_p_building()
            };
        },
        {
            link: "ChooseRandomItem"
            name: choose_random_item
            raw: (level: i32) -> i32;
            user: (level: i32) -> i32 {
                raw::choose_random_item(level)
            };
        },
        {
            link: "ChooseRandomItemEx"
            name: choose_random_item_ex
            raw: (which_type: i32, level: i32) -> i32;
            user: (which_type: i32, level: i32) -> i32 {
                raw::choose_random_item_ex(which_type, level)
            };
        },
        {
            link: "SetRandomSeed"
            name: set_random_seed
            raw: (seed: i32);
            user: (seed: i32) {
                raw::set_random_seed(seed)
            };
        },
        {
            link: "SetTerrainFog"
            name: set_terrain_fog
            raw: (a: f32, b: f32, c: f32, d: f32, e: f32);
            user: (a: f32, b: f32, c: f32, d: f32, e: f32) {
                raw::set_terrain_fog(a, b, c, d, e)
            };
        },
        {
            link: "ResetTerrainFog"
            name: reset_terrain_fog
            raw: ();
            user: () {
                raw::reset_terrain_fog()
            };
        },
        {
            link: "SetUnitFog"
            name: set_unit_fog
            raw: (a: f32, b: f32, c: f32, d: f32, e: f32);
            user: (a: f32, b: f32, c: f32, d: f32, e: f32) {
                raw::set_unit_fog(a, b, c, d, e)
            };
        },
        {
            link: "SetTerrainFogEx"
            name: set_terrain_fog_ex
            raw: (style: i32, zstart: f32, zend: f32, density: f32, red: f32, green: f32, blue: f32);
            user: (style: i32, zstart: f32, zend: f32, density: f32, red: f32, green: f32, blue: f32) {
                raw::set_terrain_fog_ex(style, zstart, zend, density, red, green, blue)
            };
        },
        {
            link: "DisplayTextToPlayer"
            name: display_text_to_player
            raw: (to_player: Handle<j_player>, x: f32, y: f32, message: *const i8);
            user: (to_player: &Agent<j_player>, x: f32, y: f32, message: &CStr) {
                raw::display_text_to_player(to_player.to_handle(), x, y, message.as_ptr())
            };
        },
        {
            link: "DisplayTimedTextToPlayer"
            name: display_timed_text_to_player
            raw: (to_player: Handle<j_player>, x: f32, y: f32, duration: f32, message: *const i8);
            user: (to_player: &Agent<j_player>, x: f32, y: f32, duration: f32, message: &CStr) {
                raw::display_timed_text_to_player(to_player.to_handle(), x, y, duration, message.as_ptr())
            };
        },
        {
            link: "DisplayTimedTextFromPlayer"
            name: display_timed_text_from_player
            raw: (to_player: Handle<j_player>, x: f32, y: f32, duration: f32, message: *const i8);
            user: (to_player: &Agent<j_player>, x: f32, y: f32, duration: f32, message: &CStr) {
                raw::display_timed_text_from_player(to_player.to_handle(), x, y, duration, message.as_ptr())
            };
        },
        {
            link: "ClearTextMessages"
            name: clear_text_messages
            raw: ();
            user: () {
                raw::clear_text_messages()
            };
        },
        {
            link: "SetDayNightModels"
            name: set_day_night_models
            raw: (terrain_d_n_c_file: *const i8, unit_d_n_c_file: *const i8);
            user: (terrain_d_n_c_file: &CStr, unit_d_n_c_file: &CStr) {
                raw::set_day_night_models(terrain_d_n_c_file.as_ptr(), unit_d_n_c_file.as_ptr())
            };
        },
        {
            link: "SetPortraitLight"
            name: set_portrait_light
            raw: (portrait_d_n_c_file: *const i8);
            user: (portrait_d_n_c_file: &CStr) {
                raw::set_portrait_light(portrait_d_n_c_file.as_ptr())
            };
        },
        {
            link: "SetSkyModel"
            name: set_sky_model
            raw: (sky_model_file: *const i8);
            user: (sky_model_file: &CStr) {
                raw::set_sky_model(sky_model_file.as_ptr())
            };
        },
        {
            link: "EnableUserControl"
            name: enable_user_control
            raw: (b: bool);
            user: (b: bool) {
                raw::enable_user_control(b)
            };
        },
        {
            link: "EnableUserUI"
            name: enable_user_u_i
            raw: (b: bool);
            user: (b: bool) {
                raw::enable_user_u_i(b)
            };
        },
        {
            link: "SuspendTimeOfDay"
            name: suspend_time_of_day
            raw: (b: bool);
            user: (b: bool) {
                raw::suspend_time_of_day(b)
            };
        },
        {
            link: "SetTimeOfDayScale"
            name: set_time_of_day_scale
            raw: (r: f32);
            user: (r: f32) {
                raw::set_time_of_day_scale(r)
            };
        },
        {
            link: "GetTimeOfDayScale"
            name: get_time_of_day_scale
            raw: () -> f32;
            user: () -> f32 {
                raw::get_time_of_day_scale()
            };
        },
        {
            link: "ShowInterface"
            name: show_interface
            raw: (flag: bool, fade_duration: f32);
            user: (flag: bool, fade_duration: f32) {
                raw::show_interface(flag, fade_duration)
            };
        },
        {
            link: "PauseGame"
            name: pause_game
            raw: (flag: bool);
            user: (flag: bool) {
                raw::pause_game(flag)
            };
        },
        {
            link: "UnitAddIndicator"
            name: unit_add_indicator
            raw: (which_unit: Handle<j_unit>, red: i32, green: i32, blue: i32, alpha: i32);
            user: (which_unit: &Agent<j_unit>, red: i32, green: i32, blue: i32, alpha: i32) {
                raw::unit_add_indicator(which_unit.to_handle(), red, green, blue, alpha)
            };
        },
        {
            link: "AddIndicator"
            name: add_indicator
            raw: (which_widget: Handle<j_widget>, red: i32, green: i32, blue: i32, alpha: i32);
            user: (which_widget: &Agent<j_widget>, red: i32, green: i32, blue: i32, alpha: i32) {
                raw::add_indicator(which_widget.to_handle(), red, green, blue, alpha)
            };
        },
        {
            link: "PingMinimap"
            name: ping_minimap
            raw: (x: f32, y: f32, duration: f32);
            user: (x: f32, y: f32, duration: f32) {
                raw::ping_minimap(x, y, duration)
            };
        },
        {
            link: "PingMinimapEx"
            name: ping_minimap_ex
            raw: (x: f32, y: f32, duration: f32, red: i32, green: i32, blue: i32, extra_effects: bool);
            user: (x: f32, y: f32, duration: f32, red: i32, green: i32, blue: i32, extra_effects: bool) {
                raw::ping_minimap_ex(x, y, duration, red, green, blue, extra_effects)
            };
        },
        {
            link: "CreateMinimapIconOnUnit"
            name: create_minimap_icon_on_unit
            raw: (which_unit: Handle<j_unit>, red: i32, green: i32, blue: i32, ping_path: *const i8, fog_visibility: i32) -> Handle<j_minimapicon>;
            user: (which_unit: &Agent<j_unit>, red: i32, green: i32, blue: i32, ping_path: &CStr, fog_visibility: i32) -> Handle<j_minimapicon> {
                raw::create_minimap_icon_on_unit(which_unit.to_handle(), red, green, blue, ping_path.as_ptr(), fog_visibility)
            };
        },
        {
            link: "CreateMinimapIconAtLoc"
            name: create_minimap_icon_at_loc
            raw: (_where: Handle<j_location>, red: i32, green: i32, blue: i32, ping_path: *const i8, fog_visibility: i32) -> Handle<j_minimapicon>;
            user: (_where: &Agent<j_location>, red: i32, green: i32, blue: i32, ping_path: &CStr, fog_visibility: i32) -> Handle<j_minimapicon> {
                raw::create_minimap_icon_at_loc(_where.to_handle(), red, green, blue, ping_path.as_ptr(), fog_visibility)
            };
        },
        {
            link: "CreateMinimapIcon"
            name: create_minimap_icon
            raw: (x: f32, y: f32, red: i32, green: i32, blue: i32, ping_path: *const i8, fog_visibility: i32) -> Handle<j_minimapicon>;
            user: (x: f32, y: f32, red: i32, green: i32, blue: i32, ping_path: &CStr, fog_visibility: i32) -> Handle<j_minimapicon> {
                raw::create_minimap_icon(x, y, red, green, blue, ping_path.as_ptr(), fog_visibility)
            };
        },
        {
            link: "SkinManagerGetLocalPath"
            name: skin_manager_get_local_path
            raw: (key: *const i8, out: *mut i8, out_len: *mut usize);
            user: (key: &CStr) -> String {
                raw::skin_manager_get_local_path(key.as_ptr(), stringret::ptr(), stringret::reset_len()); stringret::read_string()
            };
        },
        {
            link: "DestroyMinimapIcon"
            name: destroy_minimap_icon
            raw: (ping_id: Handle<j_minimapicon>);
            user: (ping_id: Handle<j_minimapicon>) {
                raw::destroy_minimap_icon(ping_id)
            };
        },
        {
            link: "SetMinimapIconVisible"
            name: set_minimap_icon_visible
            raw: (which_minimap_icon: Handle<j_minimapicon>, visible: bool);
            user: (which_minimap_icon: Handle<j_minimapicon>, visible: bool) {
                raw::set_minimap_icon_visible(which_minimap_icon, visible)
            };
        },
        {
            link: "SetMinimapIconOrphanDestroy"
            name: set_minimap_icon_orphan_destroy
            raw: (which_minimap_icon: Handle<j_minimapicon>, do_destroy: bool);
            user: (which_minimap_icon: Handle<j_minimapicon>, do_destroy: bool) {
                raw::set_minimap_icon_orphan_destroy(which_minimap_icon, do_destroy)
            };
        },
        {
            link: "EnableOcclusion"
            name: enable_occlusion
            raw: (flag: bool);
            user: (flag: bool) {
                raw::enable_occlusion(flag)
            };
        },
        {
            link: "SetIntroShotText"
            name: set_intro_shot_text
            raw: (intro_text: *const i8);
            user: (intro_text: &CStr) {
                raw::set_intro_shot_text(intro_text.as_ptr())
            };
        },
        {
            link: "SetIntroShotModel"
            name: set_intro_shot_model
            raw: (intro_model_path: *const i8);
            user: (intro_model_path: &CStr) {
                raw::set_intro_shot_model(intro_model_path.as_ptr())
            };
        },
        {
            link: "EnableWorldFogBoundary"
            name: enable_world_fog_boundary
            raw: (b: bool);
            user: (b: bool) {
                raw::enable_world_fog_boundary(b)
            };
        },
        {
            link: "PlayModelCinematic"
            name: play_model_cinematic
            raw: (model_name: *const i8);
            user: (model_name: &CStr) {
                raw::play_model_cinematic(model_name.as_ptr())
            };
        },
        {
            link: "PlayCinematic"
            name: play_cinematic
            raw: (movie_name: *const i8);
            user: (movie_name: &CStr) {
                raw::play_cinematic(movie_name.as_ptr())
            };
        },
        {
            link: "ForceUIKey"
            name: force_u_i_key
            raw: (key: *const i8);
            user: (key: &CStr) {
                raw::force_u_i_key(key.as_ptr())
            };
        },
        {
            link: "ForceUICancel"
            name: force_u_i_cancel
            raw: ();
            user: () {
                raw::force_u_i_cancel()
            };
        },
        {
            link: "DisplayLoadDialog"
            name: display_load_dialog
            raw: ();
            user: () {
                raw::display_load_dialog()
            };
        },
        {
            link: "SetAltMinimapIcon"
            name: set_alt_minimap_icon
            raw: (icon_path: *const i8);
            user: (icon_path: &CStr) {
                raw::set_alt_minimap_icon(icon_path.as_ptr())
            };
        },
        {
            link: "DisableRestartMission"
            name: disable_restart_mission
            raw: (flag: bool);
            user: (flag: bool) {
                raw::disable_restart_mission(flag)
            };
        },
        {
            link: "CreateTextTag"
            name: create_text_tag
            raw: () -> Handle<j_texttag>;
            user: () -> Handle<j_texttag> {
                raw::create_text_tag()
            };
        },
        {
            link: "DestroyTextTag"
            name: destroy_text_tag
            raw: (t: Handle<j_texttag>);
            user: (t: Handle<j_texttag>) {
                raw::destroy_text_tag(t)
            };
        },
        {
            link: "SetTextTagText"
            name: set_text_tag_text
            raw: (t: Handle<j_texttag>, s: *const i8, height: f32);
            user: (t: Handle<j_texttag>, s: &CStr, height: f32) {
                raw::set_text_tag_text(t, s.as_ptr(), height)
            };
        },
        {
            link: "SetTextTagPos"
            name: set_text_tag_pos
            raw: (t: Handle<j_texttag>, x: f32, y: f32, height_offset: f32);
            user: (t: Handle<j_texttag>, x: f32, y: f32, height_offset: f32) {
                raw::set_text_tag_pos(t, x, y, height_offset)
            };
        },
        {
            link: "SetTextTagPosUnit"
            name: set_text_tag_pos_unit
            raw: (t: Handle<j_texttag>, which_unit: Handle<j_unit>, height_offset: f32);
            user: (t: Handle<j_texttag>, which_unit: &Agent<j_unit>, height_offset: f32) {
                raw::set_text_tag_pos_unit(t, which_unit.to_handle(), height_offset)
            };
        },
        {
            link: "SetTextTagColor"
            name: set_text_tag_color
            raw: (t: Handle<j_texttag>, red: i32, green: i32, blue: i32, alpha: i32);
            user: (t: Handle<j_texttag>, red: i32, green: i32, blue: i32, alpha: i32) {
                raw::set_text_tag_color(t, red, green, blue, alpha)
            };
        },
        {
            link: "SetTextTagVelocity"
            name: set_text_tag_velocity
            raw: (t: Handle<j_texttag>, xvel: f32, yvel: f32);
            user: (t: Handle<j_texttag>, xvel: f32, yvel: f32) {
                raw::set_text_tag_velocity(t, xvel, yvel)
            };
        },
        {
            link: "SetTextTagVisibility"
            name: set_text_tag_visibility
            raw: (t: Handle<j_texttag>, flag: bool);
            user: (t: Handle<j_texttag>, flag: bool) {
                raw::set_text_tag_visibility(t, flag)
            };
        },
        {
            link: "SetTextTagSuspended"
            name: set_text_tag_suspended
            raw: (t: Handle<j_texttag>, flag: bool);
            user: (t: Handle<j_texttag>, flag: bool) {
                raw::set_text_tag_suspended(t, flag)
            };
        },
        {
            link: "SetTextTagPermanent"
            name: set_text_tag_permanent
            raw: (t: Handle<j_texttag>, flag: bool);
            user: (t: Handle<j_texttag>, flag: bool) {
                raw::set_text_tag_permanent(t, flag)
            };
        },
        {
            link: "SetTextTagAge"
            name: set_text_tag_age
            raw: (t: Handle<j_texttag>, age: f32);
            user: (t: Handle<j_texttag>, age: f32) {
                raw::set_text_tag_age(t, age)
            };
        },
        {
            link: "SetTextTagLifespan"
            name: set_text_tag_lifespan
            raw: (t: Handle<j_texttag>, lifespan: f32);
            user: (t: Handle<j_texttag>, lifespan: f32) {
                raw::set_text_tag_lifespan(t, lifespan)
            };
        },
        {
            link: "SetTextTagFadepoint"
            name: set_text_tag_fadepoint
            raw: (t: Handle<j_texttag>, fadepoint: f32);
            user: (t: Handle<j_texttag>, fadepoint: f32) {
                raw::set_text_tag_fadepoint(t, fadepoint)
            };
        },
        {
            link: "SetReservedLocalHeroButtons"
            name: set_reserved_local_hero_buttons
            raw: (reserved: i32);
            user: (reserved: i32) {
                raw::set_reserved_local_hero_buttons(reserved)
            };
        },
        {
            link: "GetAllyColorFilterState"
            name: get_ally_color_filter_state
            raw: () -> i32;
            user: () -> i32 {
                raw::get_ally_color_filter_state()
            };
        },
        {
            link: "SetAllyColorFilterState"
            name: set_ally_color_filter_state
            raw: (state: i32);
            user: (state: i32) {
                raw::set_ally_color_filter_state(state)
            };
        },
        {
            link: "GetCreepCampFilterState"
            name: get_creep_camp_filter_state
            raw: () -> bool;
            user: () -> bool {
                raw::get_creep_camp_filter_state()
            };
        },
        {
            link: "SetCreepCampFilterState"
            name: set_creep_camp_filter_state
            raw: (state: bool);
            user: (state: bool) {
                raw::set_creep_camp_filter_state(state)
            };
        },
        {
            link: "EnableMinimapFilterButtons"
            name: enable_minimap_filter_buttons
            raw: (enable_ally: bool, enable_creep: bool);
            user: (enable_ally: bool, enable_creep: bool) {
                raw::enable_minimap_filter_buttons(enable_ally, enable_creep)
            };
        },
        {
            link: "EnableDragSelect"
            name: enable_drag_select
            raw: (state: bool, ui: bool);
            user: (state: bool, ui: bool) {
                raw::enable_drag_select(state, ui)
            };
        },
        {
            link: "EnablePreSelect"
            name: enable_pre_select
            raw: (state: bool, ui: bool);
            user: (state: bool, ui: bool) {
                raw::enable_pre_select(state, ui)
            };
        },
        {
            link: "EnableSelect"
            name: enable_select
            raw: (state: bool, ui: bool);
            user: (state: bool, ui: bool) {
                raw::enable_select(state, ui)
            };
        },
        {
            link: "CreateTrackable"
            name: create_trackable
            raw: (trackable_model_path: *const i8, x: f32, y: f32, facing: f32) -> Handle<j_trackable>;
            user: (trackable_model_path: &CStr, x: f32, y: f32, facing: f32) -> Option<Weak<j_trackable>> {
                raw::create_trackable(trackable_model_path.as_ptr(), x, y, facing).to_weak()
            };
        },
        {
            link: "CreateQuest"
            name: create_quest
            raw: () -> Handle<j_quest>;
            user: () -> Option<Weak<j_quest>> {
                raw::create_quest().to_weak()
            };
        },
        {
            link: "DestroyQuest"
            name: destroy_quest
            raw: (which_quest: Handle<j_quest>);
            user: (which_quest: &Agent<j_quest>) {
                raw::destroy_quest(which_quest.to_handle())
            };
        },
        {
            link: "QuestSetTitle"
            name: quest_set_title
            raw: (which_quest: Handle<j_quest>, title: *const i8);
            user: (which_quest: &Agent<j_quest>, title: &CStr) {
                raw::quest_set_title(which_quest.to_handle(), title.as_ptr())
            };
        },
        {
            link: "QuestSetDescription"
            name: quest_set_description
            raw: (which_quest: Handle<j_quest>, description: *const i8);
            user: (which_quest: &Agent<j_quest>, description: &CStr) {
                raw::quest_set_description(which_quest.to_handle(), description.as_ptr())
            };
        },
        {
            link: "QuestSetIconPath"
            name: quest_set_icon_path
            raw: (which_quest: Handle<j_quest>, icon_path: *const i8);
            user: (which_quest: &Agent<j_quest>, icon_path: &CStr) {
                raw::quest_set_icon_path(which_quest.to_handle(), icon_path.as_ptr())
            };
        },
        {
            link: "QuestSetRequired"
            name: quest_set_required
            raw: (which_quest: Handle<j_quest>, required: bool);
            user: (which_quest: &Agent<j_quest>, required: bool) {
                raw::quest_set_required(which_quest.to_handle(), required)
            };
        },
        {
            link: "QuestSetCompleted"
            name: quest_set_completed
            raw: (which_quest: Handle<j_quest>, completed: bool);
            user: (which_quest: &Agent<j_quest>, completed: bool) {
                raw::quest_set_completed(which_quest.to_handle(), completed)
            };
        },
        {
            link: "QuestSetDiscovered"
            name: quest_set_discovered
            raw: (which_quest: Handle<j_quest>, discovered: bool);
            user: (which_quest: &Agent<j_quest>, discovered: bool) {
                raw::quest_set_discovered(which_quest.to_handle(), discovered)
            };
        },
        {
            link: "QuestSetFailed"
            name: quest_set_failed
            raw: (which_quest: Handle<j_quest>, failed: bool);
            user: (which_quest: &Agent<j_quest>, failed: bool) {
                raw::quest_set_failed(which_quest.to_handle(), failed)
            };
        },
        {
            link: "QuestSetEnabled"
            name: quest_set_enabled
            raw: (which_quest: Handle<j_quest>, enabled: bool);
            user: (which_quest: &Agent<j_quest>, enabled: bool) {
                raw::quest_set_enabled(which_quest.to_handle(), enabled)
            };
        },
        {
            link: "IsQuestRequired"
            name: is_quest_required
            raw: (which_quest: Handle<j_quest>) -> bool;
            user: (which_quest: &Agent<j_quest>) -> bool {
                raw::is_quest_required(which_quest.to_handle())
            };
        },
        {
            link: "IsQuestCompleted"
            name: is_quest_completed
            raw: (which_quest: Handle<j_quest>) -> bool;
            user: (which_quest: &Agent<j_quest>) -> bool {
                raw::is_quest_completed(which_quest.to_handle())
            };
        },
        {
            link: "IsQuestDiscovered"
            name: is_quest_discovered
            raw: (which_quest: Handle<j_quest>) -> bool;
            user: (which_quest: &Agent<j_quest>) -> bool {
                raw::is_quest_discovered(which_quest.to_handle())
            };
        },
        {
            link: "IsQuestFailed"
            name: is_quest_failed
            raw: (which_quest: Handle<j_quest>) -> bool;
            user: (which_quest: &Agent<j_quest>) -> bool {
                raw::is_quest_failed(which_quest.to_handle())
            };
        },
        {
            link: "IsQuestEnabled"
            name: is_quest_enabled
            raw: (which_quest: Handle<j_quest>) -> bool;
            user: (which_quest: &Agent<j_quest>) -> bool {
                raw::is_quest_enabled(which_quest.to_handle())
            };
        },
        {
            link: "QuestCreateItem"
            name: quest_create_item
            raw: (which_quest: Handle<j_quest>) -> Handle<j_questitem>;
            user: (which_quest: &Agent<j_quest>) -> Option<Weak<j_questitem>> {
                raw::quest_create_item(which_quest.to_handle()).to_weak()
            };
        },
        {
            link: "QuestItemSetDescription"
            name: quest_item_set_description
            raw: (which_quest_item: Handle<j_questitem>, description: *const i8);
            user: (which_quest_item: &Agent<j_questitem>, description: &CStr) {
                raw::quest_item_set_description(which_quest_item.to_handle(), description.as_ptr())
            };
        },
        {
            link: "QuestItemSetCompleted"
            name: quest_item_set_completed
            raw: (which_quest_item: Handle<j_questitem>, completed: bool);
            user: (which_quest_item: &Agent<j_questitem>, completed: bool) {
                raw::quest_item_set_completed(which_quest_item.to_handle(), completed)
            };
        },
        {
            link: "IsQuestItemCompleted"
            name: is_quest_item_completed
            raw: (which_quest_item: Handle<j_questitem>) -> bool;
            user: (which_quest_item: &Agent<j_questitem>) -> bool {
                raw::is_quest_item_completed(which_quest_item.to_handle())
            };
        },
        {
            link: "CreateDefeatCondition"
            name: create_defeat_condition
            raw: () -> Handle<j_defeatcondition>;
            user: () -> Option<Weak<j_defeatcondition>> {
                raw::create_defeat_condition().to_weak()
            };
        },
        {
            link: "DestroyDefeatCondition"
            name: destroy_defeat_condition
            raw: (which_condition: Handle<j_defeatcondition>);
            user: (which_condition: &Agent<j_defeatcondition>) {
                raw::destroy_defeat_condition(which_condition.to_handle())
            };
        },
        {
            link: "DefeatConditionSetDescription"
            name: defeat_condition_set_description
            raw: (which_condition: Handle<j_defeatcondition>, description: *const i8);
            user: (which_condition: &Agent<j_defeatcondition>, description: &CStr) {
                raw::defeat_condition_set_description(which_condition.to_handle(), description.as_ptr())
            };
        },
        {
            link: "FlashQuestDialogButton"
            name: flash_quest_dialog_button
            raw: ();
            user: () {
                raw::flash_quest_dialog_button()
            };
        },
        {
            link: "ForceQuestDialogUpdate"
            name: force_quest_dialog_update
            raw: ();
            user: () {
                raw::force_quest_dialog_update()
            };
        },
        {
            link: "CreateTimerDialog"
            name: create_timer_dialog
            raw: (t: Handle<j_timer>) -> Handle<j_timerdialog>;
            user: (t: &Agent<j_timer>) -> Option<Weak<j_timerdialog>> {
                raw::create_timer_dialog(t.to_handle()).to_weak()
            };
        },
        {
            link: "DestroyTimerDialog"
            name: destroy_timer_dialog
            raw: (which_dialog: Handle<j_timerdialog>);
            user: (which_dialog: &Agent<j_timerdialog>) {
                raw::destroy_timer_dialog(which_dialog.to_handle())
            };
        },
        {
            link: "TimerDialogSetTitle"
            name: timer_dialog_set_title
            raw: (which_dialog: Handle<j_timerdialog>, title: *const i8);
            user: (which_dialog: &Agent<j_timerdialog>, title: &CStr) {
                raw::timer_dialog_set_title(which_dialog.to_handle(), title.as_ptr())
            };
        },
        {
            link: "TimerDialogSetTitleColor"
            name: timer_dialog_set_title_color
            raw: (which_dialog: Handle<j_timerdialog>, red: i32, green: i32, blue: i32, alpha: i32);
            user: (which_dialog: &Agent<j_timerdialog>, red: i32, green: i32, blue: i32, alpha: i32) {
                raw::timer_dialog_set_title_color(which_dialog.to_handle(), red, green, blue, alpha)
            };
        },
        {
            link: "TimerDialogSetTimeColor"
            name: timer_dialog_set_time_color
            raw: (which_dialog: Handle<j_timerdialog>, red: i32, green: i32, blue: i32, alpha: i32);
            user: (which_dialog: &Agent<j_timerdialog>, red: i32, green: i32, blue: i32, alpha: i32) {
                raw::timer_dialog_set_time_color(which_dialog.to_handle(), red, green, blue, alpha)
            };
        },
        {
            link: "TimerDialogSetSpeed"
            name: timer_dialog_set_speed
            raw: (which_dialog: Handle<j_timerdialog>, speed_mult_factor: f32);
            user: (which_dialog: &Agent<j_timerdialog>, speed_mult_factor: f32) {
                raw::timer_dialog_set_speed(which_dialog.to_handle(), speed_mult_factor)
            };
        },
        {
            link: "TimerDialogDisplay"
            name: timer_dialog_display
            raw: (which_dialog: Handle<j_timerdialog>, display: bool);
            user: (which_dialog: &Agent<j_timerdialog>, display: bool) {
                raw::timer_dialog_display(which_dialog.to_handle(), display)
            };
        },
        {
            link: "IsTimerDialogDisplayed"
            name: is_timer_dialog_displayed
            raw: (which_dialog: Handle<j_timerdialog>) -> bool;
            user: (which_dialog: &Agent<j_timerdialog>) -> bool {
                raw::is_timer_dialog_displayed(which_dialog.to_handle())
            };
        },
        {
            link: "TimerDialogSetRealTimeRemaining"
            name: timer_dialog_set_real_time_remaining
            raw: (which_dialog: Handle<j_timerdialog>, time_remaining: f32);
            user: (which_dialog: &Agent<j_timerdialog>, time_remaining: f32) {
                raw::timer_dialog_set_real_time_remaining(which_dialog.to_handle(), time_remaining)
            };
        },
        {
            link: "CreateLeaderboard"
            name: create_leaderboard
            raw: () -> Handle<j_leaderboard>;
            user: () -> Option<Weak<j_leaderboard>> {
                raw::create_leaderboard().to_weak()
            };
        },
        {
            link: "DestroyLeaderboard"
            name: destroy_leaderboard
            raw: (lb: Handle<j_leaderboard>);
            user: (lb: &Agent<j_leaderboard>) {
                raw::destroy_leaderboard(lb.to_handle())
            };
        },
        {
            link: "LeaderboardDisplay"
            name: leaderboard_display
            raw: (lb: Handle<j_leaderboard>, show: bool);
            user: (lb: &Agent<j_leaderboard>, show: bool) {
                raw::leaderboard_display(lb.to_handle(), show)
            };
        },
        {
            link: "IsLeaderboardDisplayed"
            name: is_leaderboard_displayed
            raw: (lb: Handle<j_leaderboard>) -> bool;
            user: (lb: &Agent<j_leaderboard>) -> bool {
                raw::is_leaderboard_displayed(lb.to_handle())
            };
        },
        {
            link: "LeaderboardGetItemCount"
            name: leaderboard_get_item_count
            raw: (lb: Handle<j_leaderboard>) -> i32;
            user: (lb: &Agent<j_leaderboard>) -> i32 {
                raw::leaderboard_get_item_count(lb.to_handle())
            };
        },
        {
            link: "LeaderboardSetSizeByItemCount"
            name: leaderboard_set_size_by_item_count
            raw: (lb: Handle<j_leaderboard>, count: i32);
            user: (lb: &Agent<j_leaderboard>, count: i32) {
                raw::leaderboard_set_size_by_item_count(lb.to_handle(), count)
            };
        },
        {
            link: "LeaderboardAddItem"
            name: leaderboard_add_item
            raw: (lb: Handle<j_leaderboard>, label: *const i8, value: i32, p: Handle<j_player>);
            user: (lb: &Agent<j_leaderboard>, label: &CStr, value: i32, p: &Agent<j_player>) {
                raw::leaderboard_add_item(lb.to_handle(), label.as_ptr(), value, p.to_handle())
            };
        },
        {
            link: "LeaderboardRemoveItem"
            name: leaderboard_remove_item
            raw: (lb: Handle<j_leaderboard>, index: i32);
            user: (lb: &Agent<j_leaderboard>, index: i32) {
                raw::leaderboard_remove_item(lb.to_handle(), index)
            };
        },
        {
            link: "LeaderboardRemovePlayerItem"
            name: leaderboard_remove_player_item
            raw: (lb: Handle<j_leaderboard>, p: Handle<j_player>);
            user: (lb: &Agent<j_leaderboard>, p: &Agent<j_player>) {
                raw::leaderboard_remove_player_item(lb.to_handle(), p.to_handle())
            };
        },
        {
            link: "LeaderboardClear"
            name: leaderboard_clear
            raw: (lb: Handle<j_leaderboard>);
            user: (lb: &Agent<j_leaderboard>) {
                raw::leaderboard_clear(lb.to_handle())
            };
        },
        {
            link: "LeaderboardSortItemsByValue"
            name: leaderboard_sort_items_by_value
            raw: (lb: Handle<j_leaderboard>, ascending: bool);
            user: (lb: &Agent<j_leaderboard>, ascending: bool) {
                raw::leaderboard_sort_items_by_value(lb.to_handle(), ascending)
            };
        },
        {
            link: "LeaderboardSortItemsByPlayer"
            name: leaderboard_sort_items_by_player
            raw: (lb: Handle<j_leaderboard>, ascending: bool);
            user: (lb: &Agent<j_leaderboard>, ascending: bool) {
                raw::leaderboard_sort_items_by_player(lb.to_handle(), ascending)
            };
        },
        {
            link: "LeaderboardSortItemsByLabel"
            name: leaderboard_sort_items_by_label
            raw: (lb: Handle<j_leaderboard>, ascending: bool);
            user: (lb: &Agent<j_leaderboard>, ascending: bool) {
                raw::leaderboard_sort_items_by_label(lb.to_handle(), ascending)
            };
        },
        {
            link: "LeaderboardHasPlayerItem"
            name: leaderboard_has_player_item
            raw: (lb: Handle<j_leaderboard>, p: Handle<j_player>) -> bool;
            user: (lb: &Agent<j_leaderboard>, p: &Agent<j_player>) -> bool {
                raw::leaderboard_has_player_item(lb.to_handle(), p.to_handle())
            };
        },
        {
            link: "LeaderboardGetPlayerIndex"
            name: leaderboard_get_player_index
            raw: (lb: Handle<j_leaderboard>, p: Handle<j_player>) -> i32;
            user: (lb: &Agent<j_leaderboard>, p: &Agent<j_player>) -> i32 {
                raw::leaderboard_get_player_index(lb.to_handle(), p.to_handle())
            };
        },
        {
            link: "LeaderboardSetLabel"
            name: leaderboard_set_label
            raw: (lb: Handle<j_leaderboard>, label: *const i8);
            user: (lb: &Agent<j_leaderboard>, label: &CStr) {
                raw::leaderboard_set_label(lb.to_handle(), label.as_ptr())
            };
        },
        {
            link: "LeaderboardGetLabelText"
            name: leaderboard_get_label_text
            raw: (lb: Handle<j_leaderboard>, out: *mut i8, out_len: *mut usize);
            user: (lb: &Agent<j_leaderboard>) -> String {
                raw::leaderboard_get_label_text(lb.to_handle(), stringret::ptr(), stringret::reset_len()); stringret::read_string()
            };
        },
        {
            link: "PlayerSetLeaderboard"
            name: player_set_leaderboard
            raw: (to_player: Handle<j_player>, lb: Handle<j_leaderboard>);
            user: (to_player: &Agent<j_player>, lb: &Agent<j_leaderboard>) {
                raw::player_set_leaderboard(to_player.to_handle(), lb.to_handle())
            };
        },
        {
            link: "PlayerGetLeaderboard"
            name: player_get_leaderboard
            raw: (to_player: Handle<j_player>) -> Handle<j_leaderboard>;
            user: (to_player: &Agent<j_player>) -> Option<Weak<j_leaderboard>> {
                raw::player_get_leaderboard(to_player.to_handle()).to_weak()
            };
        },
        {
            link: "LeaderboardSetLabelColor"
            name: leaderboard_set_label_color
            raw: (lb: Handle<j_leaderboard>, red: i32, green: i32, blue: i32, alpha: i32);
            user: (lb: &Agent<j_leaderboard>, red: i32, green: i32, blue: i32, alpha: i32) {
                raw::leaderboard_set_label_color(lb.to_handle(), red, green, blue, alpha)
            };
        },
        {
            link: "LeaderboardSetValueColor"
            name: leaderboard_set_value_color
            raw: (lb: Handle<j_leaderboard>, red: i32, green: i32, blue: i32, alpha: i32);
            user: (lb: &Agent<j_leaderboard>, red: i32, green: i32, blue: i32, alpha: i32) {
                raw::leaderboard_set_value_color(lb.to_handle(), red, green, blue, alpha)
            };
        },
        {
            link: "LeaderboardSetStyle"
            name: leaderboard_set_style
            raw: (lb: Handle<j_leaderboard>, show_label: bool, show_names: bool, show_values: bool, show_icons: bool);
            user: (lb: &Agent<j_leaderboard>, show_label: bool, show_names: bool, show_values: bool, show_icons: bool) {
                raw::leaderboard_set_style(lb.to_handle(), show_label, show_names, show_values, show_icons)
            };
        },
        {
            link: "LeaderboardSetItemValue"
            name: leaderboard_set_item_value
            raw: (lb: Handle<j_leaderboard>, which_item: i32, val: i32);
            user: (lb: &Agent<j_leaderboard>, which_item: i32, val: i32) {
                raw::leaderboard_set_item_value(lb.to_handle(), which_item, val)
            };
        },
        {
            link: "LeaderboardSetItemLabel"
            name: leaderboard_set_item_label
            raw: (lb: Handle<j_leaderboard>, which_item: i32, val: *const i8);
            user: (lb: &Agent<j_leaderboard>, which_item: i32, val: &CStr) {
                raw::leaderboard_set_item_label(lb.to_handle(), which_item, val.as_ptr())
            };
        },
        {
            link: "LeaderboardSetItemStyle"
            name: leaderboard_set_item_style
            raw: (lb: Handle<j_leaderboard>, which_item: i32, show_label: bool, show_value: bool, show_icon: bool);
            user: (lb: &Agent<j_leaderboard>, which_item: i32, show_label: bool, show_value: bool, show_icon: bool) {
                raw::leaderboard_set_item_style(lb.to_handle(), which_item, show_label, show_value, show_icon)
            };
        },
        {
            link: "LeaderboardSetItemLabelColor"
            name: leaderboard_set_item_label_color
            raw: (lb: Handle<j_leaderboard>, which_item: i32, red: i32, green: i32, blue: i32, alpha: i32);
            user: (lb: &Agent<j_leaderboard>, which_item: i32, red: i32, green: i32, blue: i32, alpha: i32) {
                raw::leaderboard_set_item_label_color(lb.to_handle(), which_item, red, green, blue, alpha)
            };
        },
        {
            link: "LeaderboardSetItemValueColor"
            name: leaderboard_set_item_value_color
            raw: (lb: Handle<j_leaderboard>, which_item: i32, red: i32, green: i32, blue: i32, alpha: i32);
            user: (lb: &Agent<j_leaderboard>, which_item: i32, red: i32, green: i32, blue: i32, alpha: i32) {
                raw::leaderboard_set_item_value_color(lb.to_handle(), which_item, red, green, blue, alpha)
            };
        },
        {
            link: "CreateMultiboard"
            name: create_multiboard
            raw: () -> Handle<j_multiboard>;
            user: () -> Option<Weak<j_multiboard>> {
                raw::create_multiboard().to_weak()
            };
        },
        {
            link: "DestroyMultiboard"
            name: destroy_multiboard
            raw: (lb: Handle<j_multiboard>);
            user: (lb: &Agent<j_multiboard>) {
                raw::destroy_multiboard(lb.to_handle())
            };
        },
        {
            link: "MultiboardDisplay"
            name: multiboard_display
            raw: (lb: Handle<j_multiboard>, show: bool);
            user: (lb: &Agent<j_multiboard>, show: bool) {
                raw::multiboard_display(lb.to_handle(), show)
            };
        },
        {
            link: "IsMultiboardDisplayed"
            name: is_multiboard_displayed
            raw: (lb: Handle<j_multiboard>) -> bool;
            user: (lb: &Agent<j_multiboard>) -> bool {
                raw::is_multiboard_displayed(lb.to_handle())
            };
        },
        {
            link: "MultiboardMinimize"
            name: multiboard_minimize
            raw: (lb: Handle<j_multiboard>, minimize: bool);
            user: (lb: &Agent<j_multiboard>, minimize: bool) {
                raw::multiboard_minimize(lb.to_handle(), minimize)
            };
        },
        {
            link: "IsMultiboardMinimized"
            name: is_multiboard_minimized
            raw: (lb: Handle<j_multiboard>) -> bool;
            user: (lb: &Agent<j_multiboard>) -> bool {
                raw::is_multiboard_minimized(lb.to_handle())
            };
        },
        {
            link: "MultiboardClear"
            name: multiboard_clear
            raw: (lb: Handle<j_multiboard>);
            user: (lb: &Agent<j_multiboard>) {
                raw::multiboard_clear(lb.to_handle())
            };
        },
        {
            link: "MultiboardSetTitleText"
            name: multiboard_set_title_text
            raw: (lb: Handle<j_multiboard>, label: *const i8);
            user: (lb: &Agent<j_multiboard>, label: &CStr) {
                raw::multiboard_set_title_text(lb.to_handle(), label.as_ptr())
            };
        },
        {
            link: "MultiboardGetTitleText"
            name: multiboard_get_title_text
            raw: (lb: Handle<j_multiboard>, out: *mut i8, out_len: *mut usize);
            user: (lb: &Agent<j_multiboard>) -> String {
                raw::multiboard_get_title_text(lb.to_handle(), stringret::ptr(), stringret::reset_len()); stringret::read_string()
            };
        },
        {
            link: "MultiboardSetTitleTextColor"
            name: multiboard_set_title_text_color
            raw: (lb: Handle<j_multiboard>, red: i32, green: i32, blue: i32, alpha: i32);
            user: (lb: &Agent<j_multiboard>, red: i32, green: i32, blue: i32, alpha: i32) {
                raw::multiboard_set_title_text_color(lb.to_handle(), red, green, blue, alpha)
            };
        },
        {
            link: "MultiboardGetRowCount"
            name: multiboard_get_row_count
            raw: (lb: Handle<j_multiboard>) -> i32;
            user: (lb: &Agent<j_multiboard>) -> i32 {
                raw::multiboard_get_row_count(lb.to_handle())
            };
        },
        {
            link: "MultiboardGetColumnCount"
            name: multiboard_get_column_count
            raw: (lb: Handle<j_multiboard>) -> i32;
            user: (lb: &Agent<j_multiboard>) -> i32 {
                raw::multiboard_get_column_count(lb.to_handle())
            };
        },
        {
            link: "MultiboardSetColumnCount"
            name: multiboard_set_column_count
            raw: (lb: Handle<j_multiboard>, count: i32);
            user: (lb: &Agent<j_multiboard>, count: i32) {
                raw::multiboard_set_column_count(lb.to_handle(), count)
            };
        },
        {
            link: "MultiboardSetRowCount"
            name: multiboard_set_row_count
            raw: (lb: Handle<j_multiboard>, count: i32);
            user: (lb: &Agent<j_multiboard>, count: i32) {
                raw::multiboard_set_row_count(lb.to_handle(), count)
            };
        },
        {
            link: "MultiboardSetItemsStyle"
            name: multiboard_set_items_style
            raw: (lb: Handle<j_multiboard>, show_values: bool, show_icons: bool);
            user: (lb: &Agent<j_multiboard>, show_values: bool, show_icons: bool) {
                raw::multiboard_set_items_style(lb.to_handle(), show_values, show_icons)
            };
        },
        {
            link: "MultiboardSetItemsValue"
            name: multiboard_set_items_value
            raw: (lb: Handle<j_multiboard>, value: *const i8);
            user: (lb: &Agent<j_multiboard>, value: &CStr) {
                raw::multiboard_set_items_value(lb.to_handle(), value.as_ptr())
            };
        },
        {
            link: "MultiboardSetItemsValueColor"
            name: multiboard_set_items_value_color
            raw: (lb: Handle<j_multiboard>, red: i32, green: i32, blue: i32, alpha: i32);
            user: (lb: &Agent<j_multiboard>, red: i32, green: i32, blue: i32, alpha: i32) {
                raw::multiboard_set_items_value_color(lb.to_handle(), red, green, blue, alpha)
            };
        },
        {
            link: "MultiboardSetItemsWidth"
            name: multiboard_set_items_width
            raw: (lb: Handle<j_multiboard>, width: f32);
            user: (lb: &Agent<j_multiboard>, width: f32) {
                raw::multiboard_set_items_width(lb.to_handle(), width)
            };
        },
        {
            link: "MultiboardSetItemsIcon"
            name: multiboard_set_items_icon
            raw: (lb: Handle<j_multiboard>, icon_path: *const i8);
            user: (lb: &Agent<j_multiboard>, icon_path: &CStr) {
                raw::multiboard_set_items_icon(lb.to_handle(), icon_path.as_ptr())
            };
        },
        {
            link: "MultiboardGetItem"
            name: multiboard_get_item
            raw: (lb: Handle<j_multiboard>, row: i32, column: i32) -> Handle<j_multiboarditem>;
            user: (lb: &Agent<j_multiboard>, row: i32, column: i32) -> Option<Weak<j_multiboarditem>> {
                raw::multiboard_get_item(lb.to_handle(), row, column).to_weak()
            };
        },
        {
            link: "MultiboardReleaseItem"
            name: multiboard_release_item
            raw: (mbi: Handle<j_multiboarditem>);
            user: (mbi: &Agent<j_multiboarditem>) {
                raw::multiboard_release_item(mbi.to_handle())
            };
        },
        {
            link: "MultiboardSetItemStyle"
            name: multiboard_set_item_style
            raw: (mbi: Handle<j_multiboarditem>, show_value: bool, show_icon: bool);
            user: (mbi: &Agent<j_multiboarditem>, show_value: bool, show_icon: bool) {
                raw::multiboard_set_item_style(mbi.to_handle(), show_value, show_icon)
            };
        },
        {
            link: "MultiboardSetItemValue"
            name: multiboard_set_item_value
            raw: (mbi: Handle<j_multiboarditem>, val: *const i8);
            user: (mbi: &Agent<j_multiboarditem>, val: &CStr) {
                raw::multiboard_set_item_value(mbi.to_handle(), val.as_ptr())
            };
        },
        {
            link: "MultiboardSetItemValueColor"
            name: multiboard_set_item_value_color
            raw: (mbi: Handle<j_multiboarditem>, red: i32, green: i32, blue: i32, alpha: i32);
            user: (mbi: &Agent<j_multiboarditem>, red: i32, green: i32, blue: i32, alpha: i32) {
                raw::multiboard_set_item_value_color(mbi.to_handle(), red, green, blue, alpha)
            };
        },
        {
            link: "MultiboardSetItemWidth"
            name: multiboard_set_item_width
            raw: (mbi: Handle<j_multiboarditem>, width: f32);
            user: (mbi: &Agent<j_multiboarditem>, width: f32) {
                raw::multiboard_set_item_width(mbi.to_handle(), width)
            };
        },
        {
            link: "MultiboardSetItemIcon"
            name: multiboard_set_item_icon
            raw: (mbi: Handle<j_multiboarditem>, icon_file_name: *const i8);
            user: (mbi: &Agent<j_multiboarditem>, icon_file_name: &CStr) {
                raw::multiboard_set_item_icon(mbi.to_handle(), icon_file_name.as_ptr())
            };
        },
        {
            link: "MultiboardSuppressDisplay"
            name: multiboard_suppress_display
            raw: (flag: bool);
            user: (flag: bool) {
                raw::multiboard_suppress_display(flag)
            };
        },
        {
            link: "SetCameraPosition"
            name: set_camera_position
            raw: (x: f32, y: f32);
            user: (x: f32, y: f32) {
                raw::set_camera_position(x, y)
            };
        },
        {
            link: "SetCameraQuickPosition"
            name: set_camera_quick_position
            raw: (x: f32, y: f32);
            user: (x: f32, y: f32) {
                raw::set_camera_quick_position(x, y)
            };
        },
        {
            link: "SetCameraBounds"
            name: set_camera_bounds
            raw: (x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, x4: f32, y4: f32);
            user: (x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, x4: f32, y4: f32) {
                raw::set_camera_bounds(x1, y1, x2, y2, x3, y3, x4, y4)
            };
        },
        {
            link: "StopCamera"
            name: stop_camera
            raw: ();
            user: () {
                raw::stop_camera()
            };
        },
        {
            link: "ResetToGameCamera"
            name: reset_to_game_camera
            raw: (duration: f32);
            user: (duration: f32) {
                raw::reset_to_game_camera(duration)
            };
        },
        {
            link: "PanCameraTo"
            name: pan_camera_to
            raw: (x: f32, y: f32);
            user: (x: f32, y: f32) {
                raw::pan_camera_to(x, y)
            };
        },
        {
            link: "PanCameraToTimed"
            name: pan_camera_to_timed
            raw: (x: f32, y: f32, duration: f32);
            user: (x: f32, y: f32, duration: f32) {
                raw::pan_camera_to_timed(x, y, duration)
            };
        },
        {
            link: "PanCameraToWithZ"
            name: pan_camera_to_with_z
            raw: (x: f32, y: f32, z_offset_dest: f32);
            user: (x: f32, y: f32, z_offset_dest: f32) {
                raw::pan_camera_to_with_z(x, y, z_offset_dest)
            };
        },
        {
            link: "PanCameraToTimedWithZ"
            name: pan_camera_to_timed_with_z
            raw: (x: f32, y: f32, z_offset_dest: f32, duration: f32);
            user: (x: f32, y: f32, z_offset_dest: f32, duration: f32) {
                raw::pan_camera_to_timed_with_z(x, y, z_offset_dest, duration)
            };
        },
        {
            link: "SetCinematicCamera"
            name: set_cinematic_camera
            raw: (camera_model_file: *const i8);
            user: (camera_model_file: &CStr) {
                raw::set_cinematic_camera(camera_model_file.as_ptr())
            };
        },
        {
            link: "SetCameraRotateMode"
            name: set_camera_rotate_mode
            raw: (x: f32, y: f32, radians_to_sweep: f32, duration: f32);
            user: (x: f32, y: f32, radians_to_sweep: f32, duration: f32) {
                raw::set_camera_rotate_mode(x, y, radians_to_sweep, duration)
            };
        },
        {
            link: "SetCameraField"
            name: set_camera_field
            raw: (which_field: i32, value: f32, duration: f32);
            user: (which_field: i32, value: f32, duration: f32) {
                raw::set_camera_field(which_field, value, duration)
            };
        },
        {
            link: "AdjustCameraField"
            name: adjust_camera_field
            raw: (which_field: i32, offset: f32, duration: f32);
            user: (which_field: i32, offset: f32, duration: f32) {
                raw::adjust_camera_field(which_field, offset, duration)
            };
        },
        {
            link: "SetCameraTargetController"
            name: set_camera_target_controller
            raw: (which_unit: Handle<j_unit>, xoffset: f32, yoffset: f32, inherit_orientation: bool);
            user: (which_unit: &Agent<j_unit>, xoffset: f32, yoffset: f32, inherit_orientation: bool) {
                raw::set_camera_target_controller(which_unit.to_handle(), xoffset, yoffset, inherit_orientation)
            };
        },
        {
            link: "SetCameraOrientController"
            name: set_camera_orient_controller
            raw: (which_unit: Handle<j_unit>, xoffset: f32, yoffset: f32);
            user: (which_unit: &Agent<j_unit>, xoffset: f32, yoffset: f32) {
                raw::set_camera_orient_controller(which_unit.to_handle(), xoffset, yoffset)
            };
        },
        {
            link: "CreateCameraSetup"
            name: create_camera_setup
            raw: () -> Handle<j_camerasetup>;
            user: () -> Handle<j_camerasetup> {
                raw::create_camera_setup()
            };
        },
        {
            link: "CameraSetupSetField"
            name: camera_setup_set_field
            raw: (which_setup: Handle<j_camerasetup>, which_field: i32, value: f32, duration: f32);
            user: (which_setup: Handle<j_camerasetup>, which_field: i32, value: f32, duration: f32) {
                raw::camera_setup_set_field(which_setup, which_field, value, duration)
            };
        },
        {
            link: "CameraSetupGetField"
            name: camera_setup_get_field
            raw: (which_setup: Handle<j_camerasetup>, which_field: i32) -> f32;
            user: (which_setup: Handle<j_camerasetup>, which_field: i32) -> f32 {
                raw::camera_setup_get_field(which_setup, which_field)
            };
        },
        {
            link: "CameraSetupSetDestPosition"
            name: camera_setup_set_dest_position
            raw: (which_setup: Handle<j_camerasetup>, x: f32, y: f32, duration: f32);
            user: (which_setup: Handle<j_camerasetup>, x: f32, y: f32, duration: f32) {
                raw::camera_setup_set_dest_position(which_setup, x, y, duration)
            };
        },
        {
            link: "CameraSetupGetDestPositionLoc"
            name: camera_setup_get_dest_position_loc
            raw: (which_setup: Handle<j_camerasetup>) -> Handle<j_location>;
            user: (which_setup: Handle<j_camerasetup>) -> Option<Weak<j_location>> {
                raw::camera_setup_get_dest_position_loc(which_setup).to_weak()
            };
        },
        {
            link: "CameraSetupGetDestPositionX"
            name: camera_setup_get_dest_position_x
            raw: (which_setup: Handle<j_camerasetup>) -> f32;
            user: (which_setup: Handle<j_camerasetup>) -> f32 {
                raw::camera_setup_get_dest_position_x(which_setup)
            };
        },
        {
            link: "CameraSetupGetDestPositionY"
            name: camera_setup_get_dest_position_y
            raw: (which_setup: Handle<j_camerasetup>) -> f32;
            user: (which_setup: Handle<j_camerasetup>) -> f32 {
                raw::camera_setup_get_dest_position_y(which_setup)
            };
        },
        {
            link: "CameraSetupApply"
            name: camera_setup_apply
            raw: (which_setup: Handle<j_camerasetup>, do_pan: bool, pan_timed: bool);
            user: (which_setup: Handle<j_camerasetup>, do_pan: bool, pan_timed: bool) {
                raw::camera_setup_apply(which_setup, do_pan, pan_timed)
            };
        },
        {
            link: "CameraSetupApplyWithZ"
            name: camera_setup_apply_with_z
            raw: (which_setup: Handle<j_camerasetup>, z_dest_offset: f32);
            user: (which_setup: Handle<j_camerasetup>, z_dest_offset: f32) {
                raw::camera_setup_apply_with_z(which_setup, z_dest_offset)
            };
        },
        {
            link: "CameraSetupApplyForceDuration"
            name: camera_setup_apply_force_duration
            raw: (which_setup: Handle<j_camerasetup>, do_pan: bool, force_duration: f32);
            user: (which_setup: Handle<j_camerasetup>, do_pan: bool, force_duration: f32) {
                raw::camera_setup_apply_force_duration(which_setup, do_pan, force_duration)
            };
        },
        {
            link: "CameraSetupApplyForceDurationWithZ"
            name: camera_setup_apply_force_duration_with_z
            raw: (which_setup: Handle<j_camerasetup>, z_dest_offset: f32, force_duration: f32);
            user: (which_setup: Handle<j_camerasetup>, z_dest_offset: f32, force_duration: f32) {
                raw::camera_setup_apply_force_duration_with_z(which_setup, z_dest_offset, force_duration)
            };
        },
        {
            link: "BlzCameraSetupSetLabel"
            name: blz_camera_setup_set_label
            raw: (which_setup: Handle<j_camerasetup>, label: *const i8);
            user: (which_setup: Handle<j_camerasetup>, label: &CStr) {
                raw::blz_camera_setup_set_label(which_setup, label.as_ptr())
            };
        },
        {
            link: "BlzCameraSetupGetLabel"
            name: blz_camera_setup_get_label
            raw: (which_setup: Handle<j_camerasetup>, out: *mut i8, out_len: *mut usize);
            user: (which_setup: Handle<j_camerasetup>) -> String {
                raw::blz_camera_setup_get_label(which_setup, stringret::ptr(), stringret::reset_len()); stringret::read_string()
            };
        },
        {
            link: "CameraSetTargetNoise"
            name: camera_set_target_noise
            raw: (mag: f32, velocity: f32);
            user: (mag: f32, velocity: f32) {
                raw::camera_set_target_noise(mag, velocity)
            };
        },
        {
            link: "CameraSetSourceNoise"
            name: camera_set_source_noise
            raw: (mag: f32, velocity: f32);
            user: (mag: f32, velocity: f32) {
                raw::camera_set_source_noise(mag, velocity)
            };
        },
        {
            link: "CameraSetTargetNoiseEx"
            name: camera_set_target_noise_ex
            raw: (mag: f32, velocity: f32, vert_only: bool);
            user: (mag: f32, velocity: f32, vert_only: bool) {
                raw::camera_set_target_noise_ex(mag, velocity, vert_only)
            };
        },
        {
            link: "CameraSetSourceNoiseEx"
            name: camera_set_source_noise_ex
            raw: (mag: f32, velocity: f32, vert_only: bool);
            user: (mag: f32, velocity: f32, vert_only: bool) {
                raw::camera_set_source_noise_ex(mag, velocity, vert_only)
            };
        },
        {
            link: "CameraSetSmoothingFactor"
            name: camera_set_smoothing_factor
            raw: (factor: f32);
            user: (factor: f32) {
                raw::camera_set_smoothing_factor(factor)
            };
        },
        {
            link: "CameraSetFocalDistance"
            name: camera_set_focal_distance
            raw: (distance: f32);
            user: (distance: f32) {
                raw::camera_set_focal_distance(distance)
            };
        },
        {
            link: "CameraSetDepthOfFieldScale"
            name: camera_set_depth_of_field_scale
            raw: (scale: f32);
            user: (scale: f32) {
                raw::camera_set_depth_of_field_scale(scale)
            };
        },
        {
            link: "SetCineFilterTexture"
            name: set_cine_filter_texture
            raw: (filename: *const i8);
            user: (filename: &CStr) {
                raw::set_cine_filter_texture(filename.as_ptr())
            };
        },
        {
            link: "SetCineFilterBlendMode"
            name: set_cine_filter_blend_mode
            raw: (which_mode: i32);
            user: (which_mode: i32) {
                raw::set_cine_filter_blend_mode(which_mode)
            };
        },
        {
            link: "SetCineFilterTexMapFlags"
            name: set_cine_filter_tex_map_flags
            raw: (which_flags: i32);
            user: (which_flags: i32) {
                raw::set_cine_filter_tex_map_flags(which_flags)
            };
        },
        {
            link: "SetCineFilterStartUV"
            name: set_cine_filter_start_u_v
            raw: (minu: f32, minv: f32, maxu: f32, maxv: f32);
            user: (minu: f32, minv: f32, maxu: f32, maxv: f32) {
                raw::set_cine_filter_start_u_v(minu, minv, maxu, maxv)
            };
        },
        {
            link: "SetCineFilterEndUV"
            name: set_cine_filter_end_u_v
            raw: (minu: f32, minv: f32, maxu: f32, maxv: f32);
            user: (minu: f32, minv: f32, maxu: f32, maxv: f32) {
                raw::set_cine_filter_end_u_v(minu, minv, maxu, maxv)
            };
        },
        {
            link: "SetCineFilterStartColor"
            name: set_cine_filter_start_color
            raw: (red: i32, green: i32, blue: i32, alpha: i32);
            user: (red: i32, green: i32, blue: i32, alpha: i32) {
                raw::set_cine_filter_start_color(red, green, blue, alpha)
            };
        },
        {
            link: "SetCineFilterEndColor"
            name: set_cine_filter_end_color
            raw: (red: i32, green: i32, blue: i32, alpha: i32);
            user: (red: i32, green: i32, blue: i32, alpha: i32) {
                raw::set_cine_filter_end_color(red, green, blue, alpha)
            };
        },
        {
            link: "SetCineFilterDuration"
            name: set_cine_filter_duration
            raw: (duration: f32);
            user: (duration: f32) {
                raw::set_cine_filter_duration(duration)
            };
        },
        {
            link: "DisplayCineFilter"
            name: display_cine_filter
            raw: (flag: bool);
            user: (flag: bool) {
                raw::display_cine_filter(flag)
            };
        },
        {
            link: "IsCineFilterDisplayed"
            name: is_cine_filter_displayed
            raw: () -> bool;
            user: () -> bool {
                raw::is_cine_filter_displayed()
            };
        },
        {
            link: "SetCinematicScene"
            name: set_cinematic_scene
            raw: (portrait_unit_id: i32, color: i32, speaker_title: *const i8, text: *const i8, scene_duration: f32, voiceover_duration: f32);
            user: (portrait_unit_id: i32, color: i32, speaker_title: &CStr, text: &CStr, scene_duration: f32, voiceover_duration: f32) {
                raw::set_cinematic_scene(portrait_unit_id, color, speaker_title.as_ptr(), text.as_ptr(), scene_duration, voiceover_duration)
            };
        },
        {
            link: "EndCinematicScene"
            name: end_cinematic_scene
            raw: ();
            user: () {
                raw::end_cinematic_scene()
            };
        },
        {
            link: "ForceCinematicSubtitles"
            name: force_cinematic_subtitles
            raw: (flag: bool);
            user: (flag: bool) {
                raw::force_cinematic_subtitles(flag)
            };
        },
        {
            link: "SetCinematicAudio"
            name: set_cinematic_audio
            raw: (cinematic_audio: bool);
            user: (cinematic_audio: bool) {
                raw::set_cinematic_audio(cinematic_audio)
            };
        },
        {
            link: "GetCameraMargin"
            name: get_camera_margin
            raw: (which_margin: i32) -> f32;
            user: (which_margin: i32) -> f32 {
                raw::get_camera_margin(which_margin)
            };
        },
        {
            link: "GetCameraBoundMinX"
            name: get_camera_bound_min_x
            raw: () -> f32;
            user: () -> f32 {
                raw::get_camera_bound_min_x()
            };
        },
        {
            link: "GetCameraBoundMinY"
            name: get_camera_bound_min_y
            raw: () -> f32;
            user: () -> f32 {
                raw::get_camera_bound_min_y()
            };
        },
        {
            link: "GetCameraBoundMaxX"
            name: get_camera_bound_max_x
            raw: () -> f32;
            user: () -> f32 {
                raw::get_camera_bound_max_x()
            };
        },
        {
            link: "GetCameraBoundMaxY"
            name: get_camera_bound_max_y
            raw: () -> f32;
            user: () -> f32 {
                raw::get_camera_bound_max_y()
            };
        },
        {
            link: "GetCameraField"
            name: get_camera_field
            raw: (which_field: i32) -> f32;
            user: (which_field: i32) -> f32 {
                raw::get_camera_field(which_field)
            };
        },
        {
            link: "GetCameraTargetPositionX"
            name: get_camera_target_position_x
            raw: () -> f32;
            user: () -> f32 {
                raw::get_camera_target_position_x()
            };
        },
        {
            link: "GetCameraTargetPositionY"
            name: get_camera_target_position_y
            raw: () -> f32;
            user: () -> f32 {
                raw::get_camera_target_position_y()
            };
        },
        {
            link: "GetCameraTargetPositionZ"
            name: get_camera_target_position_z
            raw: () -> f32;
            user: () -> f32 {
                raw::get_camera_target_position_z()
            };
        },
        {
            link: "GetCameraTargetPositionLoc"
            name: get_camera_target_position_loc
            raw: () -> Handle<j_location>;
            user: () -> Option<Weak<j_location>> {
                raw::get_camera_target_position_loc().to_weak()
            };
        },
        {
            link: "GetCameraEyePositionX"
            name: get_camera_eye_position_x
            raw: () -> f32;
            user: () -> f32 {
                raw::get_camera_eye_position_x()
            };
        },
        {
            link: "GetCameraEyePositionY"
            name: get_camera_eye_position_y
            raw: () -> f32;
            user: () -> f32 {
                raw::get_camera_eye_position_y()
            };
        },
        {
            link: "GetCameraEyePositionZ"
            name: get_camera_eye_position_z
            raw: () -> f32;
            user: () -> f32 {
                raw::get_camera_eye_position_z()
            };
        },
        {
            link: "GetCameraEyePositionLoc"
            name: get_camera_eye_position_loc
            raw: () -> Handle<j_location>;
            user: () -> Option<Weak<j_location>> {
                raw::get_camera_eye_position_loc().to_weak()
            };
        },
        {
            link: "NewSoundEnvironment"
            name: new_sound_environment
            raw: (environment_name: *const i8);
            user: (environment_name: &CStr) {
                raw::new_sound_environment(environment_name.as_ptr())
            };
        },
        {
            link: "CreateSound"
            name: create_sound
            raw: (file_name: *const i8, looping: bool, is3_d: bool, stopwhenoutofrange: bool, fade_in_rate: i32, fade_out_rate: i32, eax_setting: *const i8) -> Handle<j_sound>;
            user: (file_name: &CStr, looping: bool, is3_d: bool, stopwhenoutofrange: bool, fade_in_rate: i32, fade_out_rate: i32, eax_setting: &CStr) -> Option<Weak<j_sound>> {
                raw::create_sound(file_name.as_ptr(), looping, is3_d, stopwhenoutofrange, fade_in_rate, fade_out_rate, eax_setting.as_ptr()).to_weak()
            };
        },
        {
            link: "CreateSoundFilenameWithLabel"
            name: create_sound_filename_with_label
            raw: (file_name: *const i8, looping: bool, is3_d: bool, stopwhenoutofrange: bool, fade_in_rate: i32, fade_out_rate: i32, s_l_k_entry_name: *const i8) -> Handle<j_sound>;
            user: (file_name: &CStr, looping: bool, is3_d: bool, stopwhenoutofrange: bool, fade_in_rate: i32, fade_out_rate: i32, s_l_k_entry_name: &CStr) -> Option<Weak<j_sound>> {
                raw::create_sound_filename_with_label(file_name.as_ptr(), looping, is3_d, stopwhenoutofrange, fade_in_rate, fade_out_rate, s_l_k_entry_name.as_ptr()).to_weak()
            };
        },
        {
            link: "CreateSoundFromLabel"
            name: create_sound_from_label
            raw: (sound_label: *const i8, looping: bool, is3_d: bool, stopwhenoutofrange: bool, fade_in_rate: i32, fade_out_rate: i32) -> Handle<j_sound>;
            user: (sound_label: &CStr, looping: bool, is3_d: bool, stopwhenoutofrange: bool, fade_in_rate: i32, fade_out_rate: i32) -> Option<Weak<j_sound>> {
                raw::create_sound_from_label(sound_label.as_ptr(), looping, is3_d, stopwhenoutofrange, fade_in_rate, fade_out_rate).to_weak()
            };
        },
        {
            link: "CreateMIDISound"
            name: create_m_i_d_i_sound
            raw: (sound_label: *const i8, fade_in_rate: i32, fade_out_rate: i32) -> Handle<j_sound>;
            user: (sound_label: &CStr, fade_in_rate: i32, fade_out_rate: i32) -> Option<Weak<j_sound>> {
                raw::create_m_i_d_i_sound(sound_label.as_ptr(), fade_in_rate, fade_out_rate).to_weak()
            };
        },
        {
            link: "SetSoundParamsFromLabel"
            name: set_sound_params_from_label
            raw: (sound_handle: Handle<j_sound>, sound_label: *const i8);
            user: (sound_handle: &Agent<j_sound>, sound_label: &CStr) {
                raw::set_sound_params_from_label(sound_handle.to_handle(), sound_label.as_ptr())
            };
        },
        {
            link: "SetSoundDistanceCutoff"
            name: set_sound_distance_cutoff
            raw: (sound_handle: Handle<j_sound>, cutoff: f32);
            user: (sound_handle: &Agent<j_sound>, cutoff: f32) {
                raw::set_sound_distance_cutoff(sound_handle.to_handle(), cutoff)
            };
        },
        {
            link: "SetSoundChannel"
            name: set_sound_channel
            raw: (sound_handle: Handle<j_sound>, channel: i32);
            user: (sound_handle: &Agent<j_sound>, channel: i32) {
                raw::set_sound_channel(sound_handle.to_handle(), channel)
            };
        },
        {
            link: "SetSoundVolume"
            name: set_sound_volume
            raw: (sound_handle: Handle<j_sound>, volume: i32);
            user: (sound_handle: &Agent<j_sound>, volume: i32) {
                raw::set_sound_volume(sound_handle.to_handle(), volume)
            };
        },
        {
            link: "SetSoundPitch"
            name: set_sound_pitch
            raw: (sound_handle: Handle<j_sound>, pitch: f32);
            user: (sound_handle: &Agent<j_sound>, pitch: f32) {
                raw::set_sound_pitch(sound_handle.to_handle(), pitch)
            };
        },
        {
            link: "SetSoundPlayPosition"
            name: set_sound_play_position
            raw: (sound_handle: Handle<j_sound>, millisecs: i32);
            user: (sound_handle: &Agent<j_sound>, millisecs: i32) {
                raw::set_sound_play_position(sound_handle.to_handle(), millisecs)
            };
        },
        {
            link: "SetSoundDistances"
            name: set_sound_distances
            raw: (sound_handle: Handle<j_sound>, min_dist: f32, max_dist: f32);
            user: (sound_handle: &Agent<j_sound>, min_dist: f32, max_dist: f32) {
                raw::set_sound_distances(sound_handle.to_handle(), min_dist, max_dist)
            };
        },
        {
            link: "SetSoundConeAngles"
            name: set_sound_cone_angles
            raw: (sound_handle: Handle<j_sound>, inside: f32, outside: f32, outside_volume: i32);
            user: (sound_handle: &Agent<j_sound>, inside: f32, outside: f32, outside_volume: i32) {
                raw::set_sound_cone_angles(sound_handle.to_handle(), inside, outside, outside_volume)
            };
        },
        {
            link: "SetSoundConeOrientation"
            name: set_sound_cone_orientation
            raw: (sound_handle: Handle<j_sound>, x: f32, y: f32, z: f32);
            user: (sound_handle: &Agent<j_sound>, x: f32, y: f32, z: f32) {
                raw::set_sound_cone_orientation(sound_handle.to_handle(), x, y, z)
            };
        },
        {
            link: "SetSoundPosition"
            name: set_sound_position
            raw: (sound_handle: Handle<j_sound>, x: f32, y: f32, z: f32);
            user: (sound_handle: &Agent<j_sound>, x: f32, y: f32, z: f32) {
                raw::set_sound_position(sound_handle.to_handle(), x, y, z)
            };
        },
        {
            link: "SetSoundVelocity"
            name: set_sound_velocity
            raw: (sound_handle: Handle<j_sound>, x: f32, y: f32, z: f32);
            user: (sound_handle: &Agent<j_sound>, x: f32, y: f32, z: f32) {
                raw::set_sound_velocity(sound_handle.to_handle(), x, y, z)
            };
        },
        {
            link: "AttachSoundToUnit"
            name: attach_sound_to_unit
            raw: (sound_handle: Handle<j_sound>, which_unit: Handle<j_unit>);
            user: (sound_handle: &Agent<j_sound>, which_unit: &Agent<j_unit>) {
                raw::attach_sound_to_unit(sound_handle.to_handle(), which_unit.to_handle())
            };
        },
        {
            link: "StartSound"
            name: start_sound
            raw: (sound_handle: Handle<j_sound>);
            user: (sound_handle: &Agent<j_sound>) {
                raw::start_sound(sound_handle.to_handle())
            };
        },
        {
            link: "StartSoundEx"
            name: start_sound_ex
            raw: (sound_handle: Handle<j_sound>, fade_in: bool);
            user: (sound_handle: &Agent<j_sound>, fade_in: bool) {
                raw::start_sound_ex(sound_handle.to_handle(), fade_in)
            };
        },
        {
            link: "StopSound"
            name: stop_sound
            raw: (sound_handle: Handle<j_sound>, kill_when_done: bool, fade_out: bool);
            user: (sound_handle: &Agent<j_sound>, kill_when_done: bool, fade_out: bool) {
                raw::stop_sound(sound_handle.to_handle(), kill_when_done, fade_out)
            };
        },
        {
            link: "KillSoundWhenDone"
            name: kill_sound_when_done
            raw: (sound_handle: Handle<j_sound>);
            user: (sound_handle: &Agent<j_sound>) {
                raw::kill_sound_when_done(sound_handle.to_handle())
            };
        },
        {
            link: "SetMapMusic"
            name: set_map_music
            raw: (music_name: *const i8, random: bool, index: i32);
            user: (music_name: &CStr, random: bool, index: i32) {
                raw::set_map_music(music_name.as_ptr(), random, index)
            };
        },
        {
            link: "ClearMapMusic"
            name: clear_map_music
            raw: ();
            user: () {
                raw::clear_map_music()
            };
        },
        {
            link: "PlayMusic"
            name: play_music
            raw: (music_name: *const i8);
            user: (music_name: &CStr) {
                raw::play_music(music_name.as_ptr())
            };
        },
        {
            link: "PlayMusicEx"
            name: play_music_ex
            raw: (music_name: *const i8, frommsecs: i32, fadeinmsecs: i32);
            user: (music_name: &CStr, frommsecs: i32, fadeinmsecs: i32) {
                raw::play_music_ex(music_name.as_ptr(), frommsecs, fadeinmsecs)
            };
        },
        {
            link: "StopMusic"
            name: stop_music
            raw: (fade_out: bool);
            user: (fade_out: bool) {
                raw::stop_music(fade_out)
            };
        },
        {
            link: "ResumeMusic"
            name: resume_music
            raw: ();
            user: () {
                raw::resume_music()
            };
        },
        {
            link: "PlayThematicMusic"
            name: play_thematic_music
            raw: (music_file_name: *const i8);
            user: (music_file_name: &CStr) {
                raw::play_thematic_music(music_file_name.as_ptr())
            };
        },
        {
            link: "PlayThematicMusicEx"
            name: play_thematic_music_ex
            raw: (music_file_name: *const i8, frommsecs: i32);
            user: (music_file_name: &CStr, frommsecs: i32) {
                raw::play_thematic_music_ex(music_file_name.as_ptr(), frommsecs)
            };
        },
        {
            link: "EndThematicMusic"
            name: end_thematic_music
            raw: ();
            user: () {
                raw::end_thematic_music()
            };
        },
        {
            link: "SetMusicVolume"
            name: set_music_volume
            raw: (volume: i32);
            user: (volume: i32) {
                raw::set_music_volume(volume)
            };
        },
        {
            link: "SetMusicPlayPosition"
            name: set_music_play_position
            raw: (millisecs: i32);
            user: (millisecs: i32) {
                raw::set_music_play_position(millisecs)
            };
        },
        {
            link: "SetThematicMusicVolume"
            name: set_thematic_music_volume
            raw: (volume: i32);
            user: (volume: i32) {
                raw::set_thematic_music_volume(volume)
            };
        },
        {
            link: "SetThematicMusicPlayPosition"
            name: set_thematic_music_play_position
            raw: (millisecs: i32);
            user: (millisecs: i32) {
                raw::set_thematic_music_play_position(millisecs)
            };
        },
        {
            link: "SetSoundDuration"
            name: set_sound_duration
            raw: (sound_handle: Handle<j_sound>, duration: i32);
            user: (sound_handle: &Agent<j_sound>, duration: i32) {
                raw::set_sound_duration(sound_handle.to_handle(), duration)
            };
        },
        {
            link: "GetSoundDuration"
            name: get_sound_duration
            raw: (sound_handle: Handle<j_sound>) -> i32;
            user: (sound_handle: &Agent<j_sound>) -> i32 {
                raw::get_sound_duration(sound_handle.to_handle())
            };
        },
        {
            link: "GetSoundFileDuration"
            name: get_sound_file_duration
            raw: (music_file_name: *const i8) -> i32;
            user: (music_file_name: &CStr) -> i32 {
                raw::get_sound_file_duration(music_file_name.as_ptr())
            };
        },
        {
            link: "VolumeGroupSetVolume"
            name: volume_group_set_volume
            raw: (vgroup: i32, scale: f32);
            user: (vgroup: i32, scale: f32) {
                raw::volume_group_set_volume(vgroup, scale)
            };
        },
        {
            link: "VolumeGroupReset"
            name: volume_group_reset
            raw: ();
            user: () {
                raw::volume_group_reset()
            };
        },
        {
            link: "GetSoundIsPlaying"
            name: get_sound_is_playing
            raw: (sound_handle: Handle<j_sound>) -> bool;
            user: (sound_handle: &Agent<j_sound>) -> bool {
                raw::get_sound_is_playing(sound_handle.to_handle())
            };
        },
        {
            link: "GetSoundIsLoading"
            name: get_sound_is_loading
            raw: (sound_handle: Handle<j_sound>) -> bool;
            user: (sound_handle: &Agent<j_sound>) -> bool {
                raw::get_sound_is_loading(sound_handle.to_handle())
            };
        },
        {
            link: "RegisterStackedSound"
            name: register_stacked_sound
            raw: (sound_handle: Handle<j_sound>, by_position: bool, rectwidth: f32, rectheight: f32);
            user: (sound_handle: &Agent<j_sound>, by_position: bool, rectwidth: f32, rectheight: f32) {
                raw::register_stacked_sound(sound_handle.to_handle(), by_position, rectwidth, rectheight)
            };
        },
        {
            link: "UnregisterStackedSound"
            name: unregister_stacked_sound
            raw: (sound_handle: Handle<j_sound>, by_position: bool, rectwidth: f32, rectheight: f32);
            user: (sound_handle: &Agent<j_sound>, by_position: bool, rectwidth: f32, rectheight: f32) {
                raw::unregister_stacked_sound(sound_handle.to_handle(), by_position, rectwidth, rectheight)
            };
        },
        {
            link: "SetSoundFacialAnimationLabel"
            name: set_sound_facial_animation_label
            raw: (sound_handle: Handle<j_sound>, animation_label: *const i8) -> bool;
            user: (sound_handle: &Agent<j_sound>, animation_label: &CStr) -> bool {
                raw::set_sound_facial_animation_label(sound_handle.to_handle(), animation_label.as_ptr())
            };
        },
        {
            link: "SetSoundFacialAnimationGroupLabel"
            name: set_sound_facial_animation_group_label
            raw: (sound_handle: Handle<j_sound>, group_label: *const i8) -> bool;
            user: (sound_handle: &Agent<j_sound>, group_label: &CStr) -> bool {
                raw::set_sound_facial_animation_group_label(sound_handle.to_handle(), group_label.as_ptr())
            };
        },
        {
            link: "SetSoundFacialAnimationSetFilepath"
            name: set_sound_facial_animation_set_filepath
            raw: (sound_handle: Handle<j_sound>, animation_set_filepath: *const i8) -> bool;
            user: (sound_handle: &Agent<j_sound>, animation_set_filepath: &CStr) -> bool {
                raw::set_sound_facial_animation_set_filepath(sound_handle.to_handle(), animation_set_filepath.as_ptr())
            };
        },
        {
            link: "SetDialogueSpeakerNameKey"
            name: set_dialogue_speaker_name_key
            raw: (sound_handle: Handle<j_sound>, speaker_name: *const i8) -> bool;
            user: (sound_handle: &Agent<j_sound>, speaker_name: &CStr) -> bool {
                raw::set_dialogue_speaker_name_key(sound_handle.to_handle(), speaker_name.as_ptr())
            };
        },
        {
            link: "GetDialogueSpeakerNameKey"
            name: get_dialogue_speaker_name_key
            raw: (sound_handle: Handle<j_sound>, out: *mut i8, out_len: *mut usize);
            user: (sound_handle: &Agent<j_sound>) -> String {
                raw::get_dialogue_speaker_name_key(sound_handle.to_handle(), stringret::ptr(), stringret::reset_len()); stringret::read_string()
            };
        },
        {
            link: "SetDialogueTextKey"
            name: set_dialogue_text_key
            raw: (sound_handle: Handle<j_sound>, dialogue_text: *const i8) -> bool;
            user: (sound_handle: &Agent<j_sound>, dialogue_text: &CStr) -> bool {
                raw::set_dialogue_text_key(sound_handle.to_handle(), dialogue_text.as_ptr())
            };
        },
        {
            link: "GetDialogueTextKey"
            name: get_dialogue_text_key
            raw: (sound_handle: Handle<j_sound>, out: *mut i8, out_len: *mut usize);
            user: (sound_handle: &Agent<j_sound>) -> String {
                raw::get_dialogue_text_key(sound_handle.to_handle(), stringret::ptr(), stringret::reset_len()); stringret::read_string()
            };
        },
        {
            link: "AddWeatherEffect"
            name: add_weather_effect
            raw: (_where: Handle<j_rect>, effect_i_d: i32) -> Handle<j_weathereffect>;
            user: (_where: &Agent<j_rect>, effect_i_d: i32) -> Handle<j_weathereffect> {
                raw::add_weather_effect(_where.to_handle(), effect_i_d)
            };
        },
        {
            link: "RemoveWeatherEffect"
            name: remove_weather_effect
            raw: (which_effect: Handle<j_weathereffect>);
            user: (which_effect: Handle<j_weathereffect>) {
                raw::remove_weather_effect(which_effect)
            };
        },
        {
            link: "EnableWeatherEffect"
            name: enable_weather_effect
            raw: (which_effect: Handle<j_weathereffect>, enable: bool);
            user: (which_effect: Handle<j_weathereffect>, enable: bool) {
                raw::enable_weather_effect(which_effect, enable)
            };
        },
        {
            link: "TerrainDeformCrater"
            name: terrain_deform_crater
            raw: (x: f32, y: f32, radius: f32, depth: f32, duration: i32, permanent: bool) -> Handle<j_terraindeformation>;
            user: (x: f32, y: f32, radius: f32, depth: f32, duration: i32, permanent: bool) -> Handle<j_terraindeformation> {
                raw::terrain_deform_crater(x, y, radius, depth, duration, permanent)
            };
        },
        {
            link: "TerrainDeformRipple"
            name: terrain_deform_ripple
            raw: (x: f32, y: f32, radius: f32, depth: f32, duration: i32, count: i32, space_waves: f32, time_waves: f32, radius_start_pct: f32, limit_neg: bool) -> Handle<j_terraindeformation>;
            user: (x: f32, y: f32, radius: f32, depth: f32, duration: i32, count: i32, space_waves: f32, time_waves: f32, radius_start_pct: f32, limit_neg: bool) -> Handle<j_terraindeformation> {
                raw::terrain_deform_ripple(x, y, radius, depth, duration, count, space_waves, time_waves, radius_start_pct, limit_neg)
            };
        },
        {
            link: "TerrainDeformWave"
            name: terrain_deform_wave
            raw: (x: f32, y: f32, dir_x: f32, dir_y: f32, distance: f32, speed: f32, radius: f32, depth: f32, trail_time: i32, count: i32) -> Handle<j_terraindeformation>;
            user: (x: f32, y: f32, dir_x: f32, dir_y: f32, distance: f32, speed: f32, radius: f32, depth: f32, trail_time: i32, count: i32) -> Handle<j_terraindeformation> {
                raw::terrain_deform_wave(x, y, dir_x, dir_y, distance, speed, radius, depth, trail_time, count)
            };
        },
        {
            link: "TerrainDeformRandom"
            name: terrain_deform_random
            raw: (x: f32, y: f32, radius: f32, min_delta: f32, max_delta: f32, duration: i32, update_interval: i32) -> Handle<j_terraindeformation>;
            user: (x: f32, y: f32, radius: f32, min_delta: f32, max_delta: f32, duration: i32, update_interval: i32) -> Handle<j_terraindeformation> {
                raw::terrain_deform_random(x, y, radius, min_delta, max_delta, duration, update_interval)
            };
        },
        {
            link: "TerrainDeformStop"
            name: terrain_deform_stop
            raw: (deformation: Handle<j_terraindeformation>, duration: i32);
            user: (deformation: Handle<j_terraindeformation>, duration: i32) {
                raw::terrain_deform_stop(deformation, duration)
            };
        },
        {
            link: "TerrainDeformStopAll"
            name: terrain_deform_stop_all
            raw: ();
            user: () {
                raw::terrain_deform_stop_all()
            };
        },
        {
            link: "AddSpecialEffect"
            name: add_special_effect
            raw: (model_name: *const i8, x: f32, y: f32) -> Handle<j_effect>;
            user: (model_name: &CStr, x: f32, y: f32) -> Option<Weak<j_effect>> {
                raw::add_special_effect(model_name.as_ptr(), x, y).to_weak()
            };
        },
        {
            link: "AddSpecialEffectLoc"
            name: add_special_effect_loc
            raw: (model_name: *const i8, _where: Handle<j_location>) -> Handle<j_effect>;
            user: (model_name: &CStr, _where: &Agent<j_location>) -> Option<Weak<j_effect>> {
                raw::add_special_effect_loc(model_name.as_ptr(), _where.to_handle()).to_weak()
            };
        },
        {
            link: "AddSpecialEffectTarget"
            name: add_special_effect_target
            raw: (model_name: *const i8, target_widget: Handle<j_widget>, attach_point_name: *const i8) -> Handle<j_effect>;
            user: (model_name: &CStr, target_widget: &Agent<j_widget>, attach_point_name: &CStr) -> Option<Weak<j_effect>> {
                raw::add_special_effect_target(model_name.as_ptr(), target_widget.to_handle(), attach_point_name.as_ptr()).to_weak()
            };
        },
        {
            link: "DestroyEffect"
            name: destroy_effect
            raw: (which_effect: Handle<j_effect>);
            user: (which_effect: &Agent<j_effect>) {
                raw::destroy_effect(which_effect.to_handle())
            };
        },
        {
            link: "AddSpellEffect"
            name: add_spell_effect
            raw: (ability_string: *const i8, t: i32, x: f32, y: f32) -> Handle<j_effect>;
            user: (ability_string: &CStr, t: i32, x: f32, y: f32) -> Option<Weak<j_effect>> {
                raw::add_spell_effect(ability_string.as_ptr(), t, x, y).to_weak()
            };
        },
        {
            link: "AddSpellEffectLoc"
            name: add_spell_effect_loc
            raw: (ability_string: *const i8, t: i32, _where: Handle<j_location>) -> Handle<j_effect>;
            user: (ability_string: &CStr, t: i32, _where: &Agent<j_location>) -> Option<Weak<j_effect>> {
                raw::add_spell_effect_loc(ability_string.as_ptr(), t, _where.to_handle()).to_weak()
            };
        },
        {
            link: "AddSpellEffectById"
            name: add_spell_effect_by_id
            raw: (ability_id: i32, t: i32, x: f32, y: f32) -> Handle<j_effect>;
            user: (ability_id: i32, t: i32, x: f32, y: f32) -> Option<Weak<j_effect>> {
                raw::add_spell_effect_by_id(ability_id, t, x, y).to_weak()
            };
        },
        {
            link: "AddSpellEffectByIdLoc"
            name: add_spell_effect_by_id_loc
            raw: (ability_id: i32, t: i32, _where: Handle<j_location>) -> Handle<j_effect>;
            user: (ability_id: i32, t: i32, _where: &Agent<j_location>) -> Option<Weak<j_effect>> {
                raw::add_spell_effect_by_id_loc(ability_id, t, _where.to_handle()).to_weak()
            };
        },
        {
            link: "AddSpellEffectTarget"
            name: add_spell_effect_target
            raw: (model_name: *const i8, t: i32, target_widget: Handle<j_widget>, attach_point: *const i8) -> Handle<j_effect>;
            user: (model_name: &CStr, t: i32, target_widget: &Agent<j_widget>, attach_point: &CStr) -> Option<Weak<j_effect>> {
                raw::add_spell_effect_target(model_name.as_ptr(), t, target_widget.to_handle(), attach_point.as_ptr()).to_weak()
            };
        },
        {
            link: "AddSpellEffectTargetById"
            name: add_spell_effect_target_by_id
            raw: (ability_id: i32, t: i32, target_widget: Handle<j_widget>, attach_point: *const i8) -> Handle<j_effect>;
            user: (ability_id: i32, t: i32, target_widget: &Agent<j_widget>, attach_point: &CStr) -> Option<Weak<j_effect>> {
                raw::add_spell_effect_target_by_id(ability_id, t, target_widget.to_handle(), attach_point.as_ptr()).to_weak()
            };
        },
        {
            link: "AddLightning"
            name: add_lightning
            raw: (code_name: *const i8, check_visibility: bool, x1: f32, y1: f32, x2: f32, y2: f32) -> Handle<j_lightning>;
            user: (code_name: &CStr, check_visibility: bool, x1: f32, y1: f32, x2: f32, y2: f32) -> Handle<j_lightning> {
                raw::add_lightning(code_name.as_ptr(), check_visibility, x1, y1, x2, y2)
            };
        },
        {
            link: "AddLightningEx"
            name: add_lightning_ex
            raw: (code_name: *const i8, check_visibility: bool, x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32) -> Handle<j_lightning>;
            user: (code_name: &CStr, check_visibility: bool, x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32) -> Handle<j_lightning> {
                raw::add_lightning_ex(code_name.as_ptr(), check_visibility, x1, y1, z1, x2, y2, z2)
            };
        },
        {
            link: "DestroyLightning"
            name: destroy_lightning
            raw: (which_bolt: Handle<j_lightning>) -> bool;
            user: (which_bolt: Handle<j_lightning>) -> bool {
                raw::destroy_lightning(which_bolt)
            };
        },
        {
            link: "MoveLightning"
            name: move_lightning
            raw: (which_bolt: Handle<j_lightning>, check_visibility: bool, x1: f32, y1: f32, x2: f32, y2: f32) -> bool;
            user: (which_bolt: Handle<j_lightning>, check_visibility: bool, x1: f32, y1: f32, x2: f32, y2: f32) -> bool {
                raw::move_lightning(which_bolt, check_visibility, x1, y1, x2, y2)
            };
        },
        {
            link: "MoveLightningEx"
            name: move_lightning_ex
            raw: (which_bolt: Handle<j_lightning>, check_visibility: bool, x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32) -> bool;
            user: (which_bolt: Handle<j_lightning>, check_visibility: bool, x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32) -> bool {
                raw::move_lightning_ex(which_bolt, check_visibility, x1, y1, z1, x2, y2, z2)
            };
        },
        {
            link: "GetLightningColorA"
            name: get_lightning_color_a
            raw: (which_bolt: Handle<j_lightning>) -> f32;
            user: (which_bolt: Handle<j_lightning>) -> f32 {
                raw::get_lightning_color_a(which_bolt)
            };
        },
        {
            link: "GetLightningColorR"
            name: get_lightning_color_r
            raw: (which_bolt: Handle<j_lightning>) -> f32;
            user: (which_bolt: Handle<j_lightning>) -> f32 {
                raw::get_lightning_color_r(which_bolt)
            };
        },
        {
            link: "GetLightningColorG"
            name: get_lightning_color_g
            raw: (which_bolt: Handle<j_lightning>) -> f32;
            user: (which_bolt: Handle<j_lightning>) -> f32 {
                raw::get_lightning_color_g(which_bolt)
            };
        },
        {
            link: "GetLightningColorB"
            name: get_lightning_color_b
            raw: (which_bolt: Handle<j_lightning>) -> f32;
            user: (which_bolt: Handle<j_lightning>) -> f32 {
                raw::get_lightning_color_b(which_bolt)
            };
        },
        {
            link: "SetLightningColor"
            name: set_lightning_color
            raw: (which_bolt: Handle<j_lightning>, r: f32, g: f32, b: f32, a: f32) -> bool;
            user: (which_bolt: Handle<j_lightning>, r: f32, g: f32, b: f32, a: f32) -> bool {
                raw::set_lightning_color(which_bolt, r, g, b, a)
            };
        },
        {
            link: "GetAbilityEffect"
            name: get_ability_effect
            raw: (ability_string: *const i8, t: i32, index: i32, out: *mut i8, out_len: *mut usize);
            user: (ability_string: &CStr, t: i32, index: i32) -> String {
                raw::get_ability_effect(ability_string.as_ptr(), t, index, stringret::ptr(), stringret::reset_len()); stringret::read_string()
            };
        },
        {
            link: "GetAbilityEffectById"
            name: get_ability_effect_by_id
            raw: (ability_id: i32, t: i32, index: i32, out: *mut i8, out_len: *mut usize);
            user: (ability_id: i32, t: i32, index: i32) -> String {
                raw::get_ability_effect_by_id(ability_id, t, index, stringret::ptr(), stringret::reset_len()); stringret::read_string()
            };
        },
        {
            link: "GetAbilitySound"
            name: get_ability_sound
            raw: (ability_string: *const i8, t: i32, out: *mut i8, out_len: *mut usize);
            user: (ability_string: &CStr, t: i32) -> String {
                raw::get_ability_sound(ability_string.as_ptr(), t, stringret::ptr(), stringret::reset_len()); stringret::read_string()
            };
        },
        {
            link: "GetAbilitySoundById"
            name: get_ability_sound_by_id
            raw: (ability_id: i32, t: i32, out: *mut i8, out_len: *mut usize);
            user: (ability_id: i32, t: i32) -> String {
                raw::get_ability_sound_by_id(ability_id, t, stringret::ptr(), stringret::reset_len()); stringret::read_string()
            };
        },
        {
            link: "GetTerrainCliffLevel"
            name: get_terrain_cliff_level
            raw: (x: f32, y: f32) -> i32;
            user: (x: f32, y: f32) -> i32 {
                raw::get_terrain_cliff_level(x, y)
            };
        },
        {
            link: "SetWaterBaseColor"
            name: set_water_base_color
            raw: (red: i32, green: i32, blue: i32, alpha: i32);
            user: (red: i32, green: i32, blue: i32, alpha: i32) {
                raw::set_water_base_color(red, green, blue, alpha)
            };
        },
        {
            link: "SetWaterDeforms"
            name: set_water_deforms
            raw: (val: bool);
            user: (val: bool) {
                raw::set_water_deforms(val)
            };
        },
        {
            link: "GetTerrainType"
            name: get_terrain_type
            raw: (x: f32, y: f32) -> i32;
            user: (x: f32, y: f32) -> i32 {
                raw::get_terrain_type(x, y)
            };
        },
        {
            link: "GetTerrainVariance"
            name: get_terrain_variance
            raw: (x: f32, y: f32) -> i32;
            user: (x: f32, y: f32) -> i32 {
                raw::get_terrain_variance(x, y)
            };
        },
        {
            link: "SetTerrainType"
            name: set_terrain_type
            raw: (x: f32, y: f32, terrain_type: i32, variation: i32, area: i32, shape: i32);
            user: (x: f32, y: f32, terrain_type: i32, variation: i32, area: i32, shape: i32) {
                raw::set_terrain_type(x, y, terrain_type, variation, area, shape)
            };
        },
        {
            link: "IsTerrainPathable"
            name: is_terrain_pathable
            raw: (x: f32, y: f32, t: i32) -> bool;
            user: (x: f32, y: f32, t: i32) -> bool {
                raw::is_terrain_pathable(x, y, t)
            };
        },
        {
            link: "SetTerrainPathable"
            name: set_terrain_pathable
            raw: (x: f32, y: f32, t: i32, flag: bool);
            user: (x: f32, y: f32, t: i32, flag: bool) {
                raw::set_terrain_pathable(x, y, t, flag)
            };
        },
        {
            link: "CreateImage"
            name: create_image
            raw: (file: *const i8, size_x: f32, size_y: f32, size_z: f32, pos_x: f32, pos_y: f32, pos_z: f32, origin_x: f32, origin_y: f32, origin_z: f32, image_type: i32) -> Handle<j_image>;
            user: (file: &CStr, size_x: f32, size_y: f32, size_z: f32, pos_x: f32, pos_y: f32, pos_z: f32, origin_x: f32, origin_y: f32, origin_z: f32, image_type: i32) -> Handle<j_image> {
                raw::create_image(file.as_ptr(), size_x, size_y, size_z, pos_x, pos_y, pos_z, origin_x, origin_y, origin_z, image_type)
            };
        },
        {
            link: "DestroyImage"
            name: destroy_image
            raw: (which_image: Handle<j_image>);
            user: (which_image: Handle<j_image>) {
                raw::destroy_image(which_image)
            };
        },
        {
            link: "ShowImage"
            name: show_image
            raw: (which_image: Handle<j_image>, flag: bool);
            user: (which_image: Handle<j_image>, flag: bool) {
                raw::show_image(which_image, flag)
            };
        },
        {
            link: "SetImageConstantHeight"
            name: set_image_constant_height
            raw: (which_image: Handle<j_image>, flag: bool, height: f32);
            user: (which_image: Handle<j_image>, flag: bool, height: f32) {
                raw::set_image_constant_height(which_image, flag, height)
            };
        },
        {
            link: "SetImagePosition"
            name: set_image_position
            raw: (which_image: Handle<j_image>, x: f32, y: f32, z: f32);
            user: (which_image: Handle<j_image>, x: f32, y: f32, z: f32) {
                raw::set_image_position(which_image, x, y, z)
            };
        },
        {
            link: "SetImageColor"
            name: set_image_color
            raw: (which_image: Handle<j_image>, red: i32, green: i32, blue: i32, alpha: i32);
            user: (which_image: Handle<j_image>, red: i32, green: i32, blue: i32, alpha: i32) {
                raw::set_image_color(which_image, red, green, blue, alpha)
            };
        },
        {
            link: "SetImageRender"
            name: set_image_render
            raw: (which_image: Handle<j_image>, flag: bool);
            user: (which_image: Handle<j_image>, flag: bool) {
                raw::set_image_render(which_image, flag)
            };
        },
        {
            link: "SetImageRenderAlways"
            name: set_image_render_always
            raw: (which_image: Handle<j_image>, flag: bool);
            user: (which_image: Handle<j_image>, flag: bool) {
                raw::set_image_render_always(which_image, flag)
            };
        },
        {
            link: "SetImageAboveWater"
            name: set_image_above_water
            raw: (which_image: Handle<j_image>, flag: bool, use_water_alpha: bool);
            user: (which_image: Handle<j_image>, flag: bool, use_water_alpha: bool) {
                raw::set_image_above_water(which_image, flag, use_water_alpha)
            };
        },
        {
            link: "SetImageType"
            name: set_image_type
            raw: (which_image: Handle<j_image>, image_type: i32);
            user: (which_image: Handle<j_image>, image_type: i32) {
                raw::set_image_type(which_image, image_type)
            };
        },
        {
            link: "CreateUbersplat"
            name: create_ubersplat
            raw: (x: f32, y: f32, name: *const i8, red: i32, green: i32, blue: i32, alpha: i32, force_paused: bool, no_birth_time: bool) -> Handle<j_ubersplat>;
            user: (x: f32, y: f32, name: &CStr, red: i32, green: i32, blue: i32, alpha: i32, force_paused: bool, no_birth_time: bool) -> Handle<j_ubersplat> {
                raw::create_ubersplat(x, y, name.as_ptr(), red, green, blue, alpha, force_paused, no_birth_time)
            };
        },
        {
            link: "DestroyUbersplat"
            name: destroy_ubersplat
            raw: (which_splat: Handle<j_ubersplat>);
            user: (which_splat: Handle<j_ubersplat>) {
                raw::destroy_ubersplat(which_splat)
            };
        },
        {
            link: "ResetUbersplat"
            name: reset_ubersplat
            raw: (which_splat: Handle<j_ubersplat>);
            user: (which_splat: Handle<j_ubersplat>) {
                raw::reset_ubersplat(which_splat)
            };
        },
        {
            link: "FinishUbersplat"
            name: finish_ubersplat
            raw: (which_splat: Handle<j_ubersplat>);
            user: (which_splat: Handle<j_ubersplat>) {
                raw::finish_ubersplat(which_splat)
            };
        },
        {
            link: "ShowUbersplat"
            name: show_ubersplat
            raw: (which_splat: Handle<j_ubersplat>, flag: bool);
            user: (which_splat: Handle<j_ubersplat>, flag: bool) {
                raw::show_ubersplat(which_splat, flag)
            };
        },
        {
            link: "SetUbersplatRender"
            name: set_ubersplat_render
            raw: (which_splat: Handle<j_ubersplat>, flag: bool);
            user: (which_splat: Handle<j_ubersplat>, flag: bool) {
                raw::set_ubersplat_render(which_splat, flag)
            };
        },
        {
            link: "SetUbersplatRenderAlways"
            name: set_ubersplat_render_always
            raw: (which_splat: Handle<j_ubersplat>, flag: bool);
            user: (which_splat: Handle<j_ubersplat>, flag: bool) {
                raw::set_ubersplat_render_always(which_splat, flag)
            };
        },
        {
            link: "SetBlight"
            name: set_blight
            raw: (which_player: Handle<j_player>, x: f32, y: f32, radius: f32, add_blight: bool);
            user: (which_player: &Agent<j_player>, x: f32, y: f32, radius: f32, add_blight: bool) {
                raw::set_blight(which_player.to_handle(), x, y, radius, add_blight)
            };
        },
        {
            link: "SetBlightRect"
            name: set_blight_rect
            raw: (which_player: Handle<j_player>, r: Handle<j_rect>, add_blight: bool);
            user: (which_player: &Agent<j_player>, r: &Agent<j_rect>, add_blight: bool) {
                raw::set_blight_rect(which_player.to_handle(), r.to_handle(), add_blight)
            };
        },
        {
            link: "SetBlightPoint"
            name: set_blight_point
            raw: (which_player: Handle<j_player>, x: f32, y: f32, add_blight: bool);
            user: (which_player: &Agent<j_player>, x: f32, y: f32, add_blight: bool) {
                raw::set_blight_point(which_player.to_handle(), x, y, add_blight)
            };
        },
        {
            link: "SetBlightLoc"
            name: set_blight_loc
            raw: (which_player: Handle<j_player>, which_location: Handle<j_location>, radius: f32, add_blight: bool);
            user: (which_player: &Agent<j_player>, which_location: &Agent<j_location>, radius: f32, add_blight: bool) {
                raw::set_blight_loc(which_player.to_handle(), which_location.to_handle(), radius, add_blight)
            };
        },
        {
            link: "CreateBlightedGoldmine"
            name: create_blighted_goldmine
            raw: (id: Handle<j_player>, x: f32, y: f32, face: f32) -> Handle<j_unit>;
            user: (id: &Agent<j_player>, x: f32, y: f32, face: f32) -> Option<Weak<j_unit>> {
                raw::create_blighted_goldmine(id.to_handle(), x, y, face).to_weak()
            };
        },
        {
            link: "IsPointBlighted"
            name: is_point_blighted
            raw: (x: f32, y: f32) -> bool;
            user: (x: f32, y: f32) -> bool {
                raw::is_point_blighted(x, y)
            };
        },
        {
            link: "SetDoodadAnimation"
            name: set_doodad_animation
            raw: (x: f32, y: f32, radius: f32, doodad_i_d: i32, nearest_only: bool, anim_name: *const i8, anim_random: bool);
            user: (x: f32, y: f32, radius: f32, doodad_i_d: i32, nearest_only: bool, anim_name: &CStr, anim_random: bool) {
                raw::set_doodad_animation(x, y, radius, doodad_i_d, nearest_only, anim_name.as_ptr(), anim_random)
            };
        },
        {
            link: "SetDoodadAnimationRect"
            name: set_doodad_animation_rect
            raw: (r: Handle<j_rect>, doodad_i_d: i32, anim_name: *const i8, anim_random: bool);
            user: (r: &Agent<j_rect>, doodad_i_d: i32, anim_name: &CStr, anim_random: bool) {
                raw::set_doodad_animation_rect(r.to_handle(), doodad_i_d, anim_name.as_ptr(), anim_random)
            };
        },
        {
            link: "StartMeleeAI"
            name: start_melee_a_i
            raw: (num: Handle<j_player>, script: *const i8);
            user: (num: &Agent<j_player>, script: &CStr) {
                raw::start_melee_a_i(num.to_handle(), script.as_ptr())
            };
        },
        {
            link: "StartCampaignAI"
            name: start_campaign_a_i
            raw: (num: Handle<j_player>, script: *const i8);
            user: (num: &Agent<j_player>, script: &CStr) {
                raw::start_campaign_a_i(num.to_handle(), script.as_ptr())
            };
        },
        {
            link: "CommandAI"
            name: command_a_i
            raw: (num: Handle<j_player>, command: i32, data: i32);
            user: (num: &Agent<j_player>, command: i32, data: i32) {
                raw::command_a_i(num.to_handle(), command, data)
            };
        },
        {
            link: "PauseCompAI"
            name: pause_comp_a_i
            raw: (p: Handle<j_player>, pause: bool);
            user: (p: &Agent<j_player>, pause: bool) {
                raw::pause_comp_a_i(p.to_handle(), pause)
            };
        },
        {
            link: "GetAIDifficulty"
            name: get_a_i_difficulty
            raw: (num: Handle<j_player>) -> i32;
            user: (num: &Agent<j_player>) -> i32 {
                raw::get_a_i_difficulty(num.to_handle())
            };
        },
        {
            link: "RemoveGuardPosition"
            name: remove_guard_position
            raw: (h_unit: Handle<j_unit>);
            user: (h_unit: &Agent<j_unit>) {
                raw::remove_guard_position(h_unit.to_handle())
            };
        },
        {
            link: "RecycleGuardPosition"
            name: recycle_guard_position
            raw: (h_unit: Handle<j_unit>);
            user: (h_unit: &Agent<j_unit>) {
                raw::recycle_guard_position(h_unit.to_handle())
            };
        },
        {
            link: "RemoveAllGuardPositions"
            name: remove_all_guard_positions
            raw: (num: Handle<j_player>);
            user: (num: &Agent<j_player>) {
                raw::remove_all_guard_positions(num.to_handle())
            };
        },
        {
            link: "Cheat"
            name: cheat
            raw: (cheat_str: *const i8);
            user: (cheat_str: &CStr) {
                raw::cheat(cheat_str.as_ptr())
            };
        },
        {
            link: "IsNoVictoryCheat"
            name: is_no_victory_cheat
            raw: () -> bool;
            user: () -> bool {
                raw::is_no_victory_cheat()
            };
        },
        {
            link: "IsNoDefeatCheat"
            name: is_no_defeat_cheat
            raw: () -> bool;
            user: () -> bool {
                raw::is_no_defeat_cheat()
            };
        },
        {
            link: "Preload"
            name: preload
            raw: (filename: *const i8);
            user: (filename: &CStr) {
                raw::preload(filename.as_ptr())
            };
        },
        {
            link: "PreloadEnd"
            name: preload_end
            raw: (timeout: f32);
            user: (timeout: f32) {
                raw::preload_end(timeout)
            };
        },
        {
            link: "PreloadStart"
            name: preload_start
            raw: ();
            user: () {
                raw::preload_start()
            };
        },
        {
            link: "PreloadRefresh"
            name: preload_refresh
            raw: ();
            user: () {
                raw::preload_refresh()
            };
        },
        {
            link: "PreloadEndEx"
            name: preload_end_ex
            raw: ();
            user: () {
                raw::preload_end_ex()
            };
        },
        {
            link: "PreloadGenClear"
            name: preload_gen_clear
            raw: ();
            user: () {
                raw::preload_gen_clear()
            };
        },
        {
            link: "PreloadGenStart"
            name: preload_gen_start
            raw: ();
            user: () {
                raw::preload_gen_start()
            };
        },
        {
            link: "PreloadGenEnd"
            name: preload_gen_end
            raw: (filename: *const i8);
            user: (filename: &CStr) {
                raw::preload_gen_end(filename.as_ptr())
            };
        },
        {
            link: "Preloader"
            name: preloader
            raw: (filename: *const i8);
            user: (filename: &CStr) {
                raw::preloader(filename.as_ptr())
            };
        },
        {
            link: "BlzHideCinematicPanels"
            name: blz_hide_cinematic_panels
            raw: (enable: bool);
            user: (enable: bool) {
                raw::blz_hide_cinematic_panels(enable)
            };
        },
        {
            link: "AutomationSetTestType"
            name: automation_set_test_type
            raw: (test_type: *const i8);
            user: (test_type: &CStr) {
                raw::automation_set_test_type(test_type.as_ptr())
            };
        },
        {
            link: "AutomationTestStart"
            name: automation_test_start
            raw: (test_name: *const i8);
            user: (test_name: &CStr) {
                raw::automation_test_start(test_name.as_ptr())
            };
        },
        {
            link: "AutomationTestEnd"
            name: automation_test_end
            raw: ();
            user: () {
                raw::automation_test_end()
            };
        },
        {
            link: "AutomationTestingFinished"
            name: automation_testing_finished
            raw: ();
            user: () {
                raw::automation_testing_finished()
            };
        },
        {
            link: "BlzGetTriggerPlayerMouseX"
            name: blz_get_trigger_player_mouse_x
            raw: () -> f32;
            user: () -> f32 {
                raw::blz_get_trigger_player_mouse_x()
            };
        },
        {
            link: "BlzGetTriggerPlayerMouseY"
            name: blz_get_trigger_player_mouse_y
            raw: () -> f32;
            user: () -> f32 {
                raw::blz_get_trigger_player_mouse_y()
            };
        },
        {
            link: "BlzGetTriggerPlayerMousePosition"
            name: blz_get_trigger_player_mouse_position
            raw: () -> Handle<j_location>;
            user: () -> Option<Weak<j_location>> {
                raw::blz_get_trigger_player_mouse_position().to_weak()
            };
        },
        {
            link: "BlzGetTriggerPlayerMouseButton"
            name: blz_get_trigger_player_mouse_button
            raw: () -> i32;
            user: () -> i32 {
                raw::blz_get_trigger_player_mouse_button()
            };
        },
        {
            link: "BlzSetAbilityTooltip"
            name: blz_set_ability_tooltip
            raw: (abil_code: i32, tooltip: *const i8, level: i32);
            user: (abil_code: i32, tooltip: &CStr, level: i32) {
                raw::blz_set_ability_tooltip(abil_code, tooltip.as_ptr(), level)
            };
        },
        {
            link: "BlzSetAbilityActivatedTooltip"
            name: blz_set_ability_activated_tooltip
            raw: (abil_code: i32, tooltip: *const i8, level: i32);
            user: (abil_code: i32, tooltip: &CStr, level: i32) {
                raw::blz_set_ability_activated_tooltip(abil_code, tooltip.as_ptr(), level)
            };
        },
        {
            link: "BlzSetAbilityExtendedTooltip"
            name: blz_set_ability_extended_tooltip
            raw: (abil_code: i32, extended_tooltip: *const i8, level: i32);
            user: (abil_code: i32, extended_tooltip: &CStr, level: i32) {
                raw::blz_set_ability_extended_tooltip(abil_code, extended_tooltip.as_ptr(), level)
            };
        },
        {
            link: "BlzSetAbilityActivatedExtendedTooltip"
            name: blz_set_ability_activated_extended_tooltip
            raw: (abil_code: i32, extended_tooltip: *const i8, level: i32);
            user: (abil_code: i32, extended_tooltip: &CStr, level: i32) {
                raw::blz_set_ability_activated_extended_tooltip(abil_code, extended_tooltip.as_ptr(), level)
            };
        },
        {
            link: "BlzSetAbilityResearchTooltip"
            name: blz_set_ability_research_tooltip
            raw: (abil_code: i32, research_tooltip: *const i8, level: i32);
            user: (abil_code: i32, research_tooltip: &CStr, level: i32) {
                raw::blz_set_ability_research_tooltip(abil_code, research_tooltip.as_ptr(), level)
            };
        },
        {
            link: "BlzSetAbilityResearchExtendedTooltip"
            name: blz_set_ability_research_extended_tooltip
            raw: (abil_code: i32, research_extended_tooltip: *const i8, level: i32);
            user: (abil_code: i32, research_extended_tooltip: &CStr, level: i32) {
                raw::blz_set_ability_research_extended_tooltip(abil_code, research_extended_tooltip.as_ptr(), level)
            };
        },
        {
            link: "BlzGetAbilityTooltip"
            name: blz_get_ability_tooltip
            raw: (abil_code: i32, level: i32, out: *mut i8, out_len: *mut usize);
            user: (abil_code: i32, level: i32) -> String {
                raw::blz_get_ability_tooltip(abil_code, level, stringret::ptr(), stringret::reset_len()); stringret::read_string()
            };
        },
        {
            link: "BlzGetAbilityActivatedTooltip"
            name: blz_get_ability_activated_tooltip
            raw: (abil_code: i32, level: i32, out: *mut i8, out_len: *mut usize);
            user: (abil_code: i32, level: i32) -> String {
                raw::blz_get_ability_activated_tooltip(abil_code, level, stringret::ptr(), stringret::reset_len()); stringret::read_string()
            };
        },
        {
            link: "BlzGetAbilityExtendedTooltip"
            name: blz_get_ability_extended_tooltip
            raw: (abil_code: i32, level: i32, out: *mut i8, out_len: *mut usize);
            user: (abil_code: i32, level: i32) -> String {
                raw::blz_get_ability_extended_tooltip(abil_code, level, stringret::ptr(), stringret::reset_len()); stringret::read_string()
            };
        },
        {
            link: "BlzGetAbilityActivatedExtendedTooltip"
            name: blz_get_ability_activated_extended_tooltip
            raw: (abil_code: i32, level: i32, out: *mut i8, out_len: *mut usize);
            user: (abil_code: i32, level: i32) -> String {
                raw::blz_get_ability_activated_extended_tooltip(abil_code, level, stringret::ptr(), stringret::reset_len()); stringret::read_string()
            };
        },
        {
            link: "BlzGetAbilityResearchTooltip"
            name: blz_get_ability_research_tooltip
            raw: (abil_code: i32, level: i32, out: *mut i8, out_len: *mut usize);
            user: (abil_code: i32, level: i32) -> String {
                raw::blz_get_ability_research_tooltip(abil_code, level, stringret::ptr(), stringret::reset_len()); stringret::read_string()
            };
        },
        {
            link: "BlzGetAbilityResearchExtendedTooltip"
            name: blz_get_ability_research_extended_tooltip
            raw: (abil_code: i32, level: i32, out: *mut i8, out_len: *mut usize);
            user: (abil_code: i32, level: i32) -> String {
                raw::blz_get_ability_research_extended_tooltip(abil_code, level, stringret::ptr(), stringret::reset_len()); stringret::read_string()
            };
        },
        {
            link: "BlzSetAbilityIcon"
            name: blz_set_ability_icon
            raw: (abil_code: i32, icon_path: *const i8);
            user: (abil_code: i32, icon_path: &CStr) {
                raw::blz_set_ability_icon(abil_code, icon_path.as_ptr())
            };
        },
        {
            link: "BlzGetAbilityIcon"
            name: blz_get_ability_icon
            raw: (abil_code: i32, out: *mut i8, out_len: *mut usize);
            user: (abil_code: i32) -> String {
                raw::blz_get_ability_icon(abil_code, stringret::ptr(), stringret::reset_len()); stringret::read_string()
            };
        },
        {
            link: "BlzSetAbilityActivatedIcon"
            name: blz_set_ability_activated_icon
            raw: (abil_code: i32, icon_path: *const i8);
            user: (abil_code: i32, icon_path: &CStr) {
                raw::blz_set_ability_activated_icon(abil_code, icon_path.as_ptr())
            };
        },
        {
            link: "BlzGetAbilityActivatedIcon"
            name: blz_get_ability_activated_icon
            raw: (abil_code: i32, out: *mut i8, out_len: *mut usize);
            user: (abil_code: i32) -> String {
                raw::blz_get_ability_activated_icon(abil_code, stringret::ptr(), stringret::reset_len()); stringret::read_string()
            };
        },
        {
            link: "BlzGetAbilityPosX"
            name: blz_get_ability_pos_x
            raw: (abil_code: i32) -> i32;
            user: (abil_code: i32) -> i32 {
                raw::blz_get_ability_pos_x(abil_code)
            };
        },
        {
            link: "BlzGetAbilityPosY"
            name: blz_get_ability_pos_y
            raw: (abil_code: i32) -> i32;
            user: (abil_code: i32) -> i32 {
                raw::blz_get_ability_pos_y(abil_code)
            };
        },
        {
            link: "BlzSetAbilityPosX"
            name: blz_set_ability_pos_x
            raw: (abil_code: i32, x: i32);
            user: (abil_code: i32, x: i32) {
                raw::blz_set_ability_pos_x(abil_code, x)
            };
        },
        {
            link: "BlzSetAbilityPosY"
            name: blz_set_ability_pos_y
            raw: (abil_code: i32, y: i32);
            user: (abil_code: i32, y: i32) {
                raw::blz_set_ability_pos_y(abil_code, y)
            };
        },
        {
            link: "BlzGetAbilityActivatedPosX"
            name: blz_get_ability_activated_pos_x
            raw: (abil_code: i32) -> i32;
            user: (abil_code: i32) -> i32 {
                raw::blz_get_ability_activated_pos_x(abil_code)
            };
        },
        {
            link: "BlzGetAbilityActivatedPosY"
            name: blz_get_ability_activated_pos_y
            raw: (abil_code: i32) -> i32;
            user: (abil_code: i32) -> i32 {
                raw::blz_get_ability_activated_pos_y(abil_code)
            };
        },
        {
            link: "BlzSetAbilityActivatedPosX"
            name: blz_set_ability_activated_pos_x
            raw: (abil_code: i32, x: i32);
            user: (abil_code: i32, x: i32) {
                raw::blz_set_ability_activated_pos_x(abil_code, x)
            };
        },
        {
            link: "BlzSetAbilityActivatedPosY"
            name: blz_set_ability_activated_pos_y
            raw: (abil_code: i32, y: i32);
            user: (abil_code: i32, y: i32) {
                raw::blz_set_ability_activated_pos_y(abil_code, y)
            };
        },
        {
            link: "BlzGetUnitMaxHP"
            name: blz_get_unit_max_h_p
            raw: (which_unit: Handle<j_unit>) -> i32;
            user: (which_unit: &Agent<j_unit>) -> i32 {
                raw::blz_get_unit_max_h_p(which_unit.to_handle())
            };
        },
        {
            link: "BlzSetUnitMaxHP"
            name: blz_set_unit_max_h_p
            raw: (which_unit: Handle<j_unit>, hp: i32);
            user: (which_unit: &Agent<j_unit>, hp: i32) {
                raw::blz_set_unit_max_h_p(which_unit.to_handle(), hp)
            };
        },
        {
            link: "BlzGetUnitMaxMana"
            name: blz_get_unit_max_mana
            raw: (which_unit: Handle<j_unit>) -> i32;
            user: (which_unit: &Agent<j_unit>) -> i32 {
                raw::blz_get_unit_max_mana(which_unit.to_handle())
            };
        },
        {
            link: "BlzSetUnitMaxMana"
            name: blz_set_unit_max_mana
            raw: (which_unit: Handle<j_unit>, mana: i32);
            user: (which_unit: &Agent<j_unit>, mana: i32) {
                raw::blz_set_unit_max_mana(which_unit.to_handle(), mana)
            };
        },
        {
            link: "BlzSetItemName"
            name: blz_set_item_name
            raw: (which_item: Handle<j_item>, name: *const i8);
            user: (which_item: &Agent<j_item>, name: &CStr) {
                raw::blz_set_item_name(which_item.to_handle(), name.as_ptr())
            };
        },
        {
            link: "BlzSetItemDescription"
            name: blz_set_item_description
            raw: (which_item: Handle<j_item>, description: *const i8);
            user: (which_item: &Agent<j_item>, description: &CStr) {
                raw::blz_set_item_description(which_item.to_handle(), description.as_ptr())
            };
        },
        {
            link: "BlzGetItemDescription"
            name: blz_get_item_description
            raw: (which_item: Handle<j_item>, out: *mut i8, out_len: *mut usize);
            user: (which_item: &Agent<j_item>) -> String {
                raw::blz_get_item_description(which_item.to_handle(), stringret::ptr(), stringret::reset_len()); stringret::read_string()
            };
        },
        {
            link: "BlzSetItemTooltip"
            name: blz_set_item_tooltip
            raw: (which_item: Handle<j_item>, tooltip: *const i8);
            user: (which_item: &Agent<j_item>, tooltip: &CStr) {
                raw::blz_set_item_tooltip(which_item.to_handle(), tooltip.as_ptr())
            };
        },
        {
            link: "BlzGetItemTooltip"
            name: blz_get_item_tooltip
            raw: (which_item: Handle<j_item>, out: *mut i8, out_len: *mut usize);
            user: (which_item: &Agent<j_item>) -> String {
                raw::blz_get_item_tooltip(which_item.to_handle(), stringret::ptr(), stringret::reset_len()); stringret::read_string()
            };
        },
        {
            link: "BlzSetItemExtendedTooltip"
            name: blz_set_item_extended_tooltip
            raw: (which_item: Handle<j_item>, extended_tooltip: *const i8);
            user: (which_item: &Agent<j_item>, extended_tooltip: &CStr) {
                raw::blz_set_item_extended_tooltip(which_item.to_handle(), extended_tooltip.as_ptr())
            };
        },
        {
            link: "BlzGetItemExtendedTooltip"
            name: blz_get_item_extended_tooltip
            raw: (which_item: Handle<j_item>, out: *mut i8, out_len: *mut usize);
            user: (which_item: &Agent<j_item>) -> String {
                raw::blz_get_item_extended_tooltip(which_item.to_handle(), stringret::ptr(), stringret::reset_len()); stringret::read_string()
            };
        },
        {
            link: "BlzSetItemIconPath"
            name: blz_set_item_icon_path
            raw: (which_item: Handle<j_item>, icon_path: *const i8);
            user: (which_item: &Agent<j_item>, icon_path: &CStr) {
                raw::blz_set_item_icon_path(which_item.to_handle(), icon_path.as_ptr())
            };
        },
        {
            link: "BlzGetItemIconPath"
            name: blz_get_item_icon_path
            raw: (which_item: Handle<j_item>, out: *mut i8, out_len: *mut usize);
            user: (which_item: &Agent<j_item>) -> String {
                raw::blz_get_item_icon_path(which_item.to_handle(), stringret::ptr(), stringret::reset_len()); stringret::read_string()
            };
        },
        {
            link: "BlzSetUnitName"
            name: blz_set_unit_name
            raw: (which_unit: Handle<j_unit>, name: *const i8);
            user: (which_unit: &Agent<j_unit>, name: &CStr) {
                raw::blz_set_unit_name(which_unit.to_handle(), name.as_ptr())
            };
        },
        {
            link: "BlzSetHeroProperName"
            name: blz_set_hero_proper_name
            raw: (which_unit: Handle<j_unit>, hero_proper_name: *const i8);
            user: (which_unit: &Agent<j_unit>, hero_proper_name: &CStr) {
                raw::blz_set_hero_proper_name(which_unit.to_handle(), hero_proper_name.as_ptr())
            };
        },
        {
            link: "BlzGetUnitBaseDamage"
            name: blz_get_unit_base_damage
            raw: (which_unit: Handle<j_unit>, weapon_index: i32) -> i32;
            user: (which_unit: &Agent<j_unit>, weapon_index: i32) -> i32 {
                raw::blz_get_unit_base_damage(which_unit.to_handle(), weapon_index)
            };
        },
        {
            link: "BlzSetUnitBaseDamage"
            name: blz_set_unit_base_damage
            raw: (which_unit: Handle<j_unit>, base_damage: i32, weapon_index: i32);
            user: (which_unit: &Agent<j_unit>, base_damage: i32, weapon_index: i32) {
                raw::blz_set_unit_base_damage(which_unit.to_handle(), base_damage, weapon_index)
            };
        },
        {
            link: "BlzGetUnitDiceNumber"
            name: blz_get_unit_dice_number
            raw: (which_unit: Handle<j_unit>, weapon_index: i32) -> i32;
            user: (which_unit: &Agent<j_unit>, weapon_index: i32) -> i32 {
                raw::blz_get_unit_dice_number(which_unit.to_handle(), weapon_index)
            };
        },
        {
            link: "BlzSetUnitDiceNumber"
            name: blz_set_unit_dice_number
            raw: (which_unit: Handle<j_unit>, dice_number: i32, weapon_index: i32);
            user: (which_unit: &Agent<j_unit>, dice_number: i32, weapon_index: i32) {
                raw::blz_set_unit_dice_number(which_unit.to_handle(), dice_number, weapon_index)
            };
        },
        {
            link: "BlzGetUnitDiceSides"
            name: blz_get_unit_dice_sides
            raw: (which_unit: Handle<j_unit>, weapon_index: i32) -> i32;
            user: (which_unit: &Agent<j_unit>, weapon_index: i32) -> i32 {
                raw::blz_get_unit_dice_sides(which_unit.to_handle(), weapon_index)
            };
        },
        {
            link: "BlzSetUnitDiceSides"
            name: blz_set_unit_dice_sides
            raw: (which_unit: Handle<j_unit>, dice_sides: i32, weapon_index: i32);
            user: (which_unit: &Agent<j_unit>, dice_sides: i32, weapon_index: i32) {
                raw::blz_set_unit_dice_sides(which_unit.to_handle(), dice_sides, weapon_index)
            };
        },
        {
            link: "BlzGetUnitAttackCooldown"
            name: blz_get_unit_attack_cooldown
            raw: (which_unit: Handle<j_unit>, weapon_index: i32) -> f32;
            user: (which_unit: &Agent<j_unit>, weapon_index: i32) -> f32 {
                raw::blz_get_unit_attack_cooldown(which_unit.to_handle(), weapon_index)
            };
        },
        {
            link: "BlzSetUnitAttackCooldown"
            name: blz_set_unit_attack_cooldown
            raw: (which_unit: Handle<j_unit>, cooldown: f32, weapon_index: i32);
            user: (which_unit: &Agent<j_unit>, cooldown: f32, weapon_index: i32) {
                raw::blz_set_unit_attack_cooldown(which_unit.to_handle(), cooldown, weapon_index)
            };
        },
        {
            link: "BlzSetSpecialEffectColorByPlayer"
            name: blz_set_special_effect_color_by_player
            raw: (which_effect: Handle<j_effect>, which_player: Handle<j_player>);
            user: (which_effect: &Agent<j_effect>, which_player: &Agent<j_player>) {
                raw::blz_set_special_effect_color_by_player(which_effect.to_handle(), which_player.to_handle())
            };
        },
        {
            link: "BlzSetSpecialEffectColor"
            name: blz_set_special_effect_color
            raw: (which_effect: Handle<j_effect>, r: i32, g: i32, b: i32);
            user: (which_effect: &Agent<j_effect>, r: i32, g: i32, b: i32) {
                raw::blz_set_special_effect_color(which_effect.to_handle(), r, g, b)
            };
        },
        {
            link: "BlzSetSpecialEffectAlpha"
            name: blz_set_special_effect_alpha
            raw: (which_effect: Handle<j_effect>, alpha: i32);
            user: (which_effect: &Agent<j_effect>, alpha: i32) {
                raw::blz_set_special_effect_alpha(which_effect.to_handle(), alpha)
            };
        },
        {
            link: "BlzSetSpecialEffectScale"
            name: blz_set_special_effect_scale
            raw: (which_effect: Handle<j_effect>, scale: f32);
            user: (which_effect: &Agent<j_effect>, scale: f32) {
                raw::blz_set_special_effect_scale(which_effect.to_handle(), scale)
            };
        },
        {
            link: "BlzSetSpecialEffectPosition"
            name: blz_set_special_effect_position
            raw: (which_effect: Handle<j_effect>, x: f32, y: f32, z: f32);
            user: (which_effect: &Agent<j_effect>, x: f32, y: f32, z: f32) {
                raw::blz_set_special_effect_position(which_effect.to_handle(), x, y, z)
            };
        },
        {
            link: "BlzSetSpecialEffectHeight"
            name: blz_set_special_effect_height
            raw: (which_effect: Handle<j_effect>, height: f32);
            user: (which_effect: &Agent<j_effect>, height: f32) {
                raw::blz_set_special_effect_height(which_effect.to_handle(), height)
            };
        },
        {
            link: "BlzSetSpecialEffectTimeScale"
            name: blz_set_special_effect_time_scale
            raw: (which_effect: Handle<j_effect>, time_scale: f32);
            user: (which_effect: &Agent<j_effect>, time_scale: f32) {
                raw::blz_set_special_effect_time_scale(which_effect.to_handle(), time_scale)
            };
        },
        {
            link: "BlzSetSpecialEffectTime"
            name: blz_set_special_effect_time
            raw: (which_effect: Handle<j_effect>, time: f32);
            user: (which_effect: &Agent<j_effect>, time: f32) {
                raw::blz_set_special_effect_time(which_effect.to_handle(), time)
            };
        },
        {
            link: "BlzSetSpecialEffectOrientation"
            name: blz_set_special_effect_orientation
            raw: (which_effect: Handle<j_effect>, yaw: f32, pitch: f32, roll: f32);
            user: (which_effect: &Agent<j_effect>, yaw: f32, pitch: f32, roll: f32) {
                raw::blz_set_special_effect_orientation(which_effect.to_handle(), yaw, pitch, roll)
            };
        },
        {
            link: "BlzSetSpecialEffectYaw"
            name: blz_set_special_effect_yaw
            raw: (which_effect: Handle<j_effect>, yaw: f32);
            user: (which_effect: &Agent<j_effect>, yaw: f32) {
                raw::blz_set_special_effect_yaw(which_effect.to_handle(), yaw)
            };
        },
        {
            link: "BlzSetSpecialEffectPitch"
            name: blz_set_special_effect_pitch
            raw: (which_effect: Handle<j_effect>, pitch: f32);
            user: (which_effect: &Agent<j_effect>, pitch: f32) {
                raw::blz_set_special_effect_pitch(which_effect.to_handle(), pitch)
            };
        },
        {
            link: "BlzSetSpecialEffectRoll"
            name: blz_set_special_effect_roll
            raw: (which_effect: Handle<j_effect>, roll: f32);
            user: (which_effect: &Agent<j_effect>, roll: f32) {
                raw::blz_set_special_effect_roll(which_effect.to_handle(), roll)
            };
        },
        {
            link: "BlzSetSpecialEffectX"
            name: blz_set_special_effect_x
            raw: (which_effect: Handle<j_effect>, x: f32);
            user: (which_effect: &Agent<j_effect>, x: f32) {
                raw::blz_set_special_effect_x(which_effect.to_handle(), x)
            };
        },
        {
            link: "BlzSetSpecialEffectY"
            name: blz_set_special_effect_y
            raw: (which_effect: Handle<j_effect>, y: f32);
            user: (which_effect: &Agent<j_effect>, y: f32) {
                raw::blz_set_special_effect_y(which_effect.to_handle(), y)
            };
        },
        {
            link: "BlzSetSpecialEffectZ"
            name: blz_set_special_effect_z
            raw: (which_effect: Handle<j_effect>, z: f32);
            user: (which_effect: &Agent<j_effect>, z: f32) {
                raw::blz_set_special_effect_z(which_effect.to_handle(), z)
            };
        },
        {
            link: "BlzSetSpecialEffectPositionLoc"
            name: blz_set_special_effect_position_loc
            raw: (which_effect: Handle<j_effect>, loc: Handle<j_location>);
            user: (which_effect: &Agent<j_effect>, loc: &Agent<j_location>) {
                raw::blz_set_special_effect_position_loc(which_effect.to_handle(), loc.to_handle())
            };
        },
        {
            link: "BlzGetLocalSpecialEffectX"
            name: blz_get_local_special_effect_x
            raw: (which_effect: Handle<j_effect>) -> f32;
            user: (which_effect: &Agent<j_effect>) -> f32 {
                raw::blz_get_local_special_effect_x(which_effect.to_handle())
            };
        },
        {
            link: "BlzGetLocalSpecialEffectY"
            name: blz_get_local_special_effect_y
            raw: (which_effect: Handle<j_effect>) -> f32;
            user: (which_effect: &Agent<j_effect>) -> f32 {
                raw::blz_get_local_special_effect_y(which_effect.to_handle())
            };
        },
        {
            link: "BlzGetLocalSpecialEffectZ"
            name: blz_get_local_special_effect_z
            raw: (which_effect: Handle<j_effect>) -> f32;
            user: (which_effect: &Agent<j_effect>) -> f32 {
                raw::blz_get_local_special_effect_z(which_effect.to_handle())
            };
        },
        {
            link: "BlzSpecialEffectClearSubAnimations"
            name: blz_special_effect_clear_sub_animations
            raw: (which_effect: Handle<j_effect>);
            user: (which_effect: &Agent<j_effect>) {
                raw::blz_special_effect_clear_sub_animations(which_effect.to_handle())
            };
        },
        {
            link: "BlzSpecialEffectRemoveSubAnimation"
            name: blz_special_effect_remove_sub_animation
            raw: (which_effect: Handle<j_effect>, which_sub_anim: i32);
            user: (which_effect: &Agent<j_effect>, which_sub_anim: i32) {
                raw::blz_special_effect_remove_sub_animation(which_effect.to_handle(), which_sub_anim)
            };
        },
        {
            link: "BlzSpecialEffectAddSubAnimation"
            name: blz_special_effect_add_sub_animation
            raw: (which_effect: Handle<j_effect>, which_sub_anim: i32);
            user: (which_effect: &Agent<j_effect>, which_sub_anim: i32) {
                raw::blz_special_effect_add_sub_animation(which_effect.to_handle(), which_sub_anim)
            };
        },
        {
            link: "BlzPlaySpecialEffect"
            name: blz_play_special_effect
            raw: (which_effect: Handle<j_effect>, which_anim: i32);
            user: (which_effect: &Agent<j_effect>, which_anim: i32) {
                raw::blz_play_special_effect(which_effect.to_handle(), which_anim)
            };
        },
        {
            link: "BlzPlaySpecialEffectWithTimeScale"
            name: blz_play_special_effect_with_time_scale
            raw: (which_effect: Handle<j_effect>, which_anim: i32, time_scale: f32);
            user: (which_effect: &Agent<j_effect>, which_anim: i32, time_scale: f32) {
                raw::blz_play_special_effect_with_time_scale(which_effect.to_handle(), which_anim, time_scale)
            };
        },
        {
            link: "BlzGetAnimName"
            name: blz_get_anim_name
            raw: (which_anim: i32, out: *mut i8, out_len: *mut usize);
            user: (which_anim: i32) -> String {
                raw::blz_get_anim_name(which_anim, stringret::ptr(), stringret::reset_len()); stringret::read_string()
            };
        },
        {
            link: "BlzGetUnitArmor"
            name: blz_get_unit_armor
            raw: (which_unit: Handle<j_unit>) -> f32;
            user: (which_unit: &Agent<j_unit>) -> f32 {
                raw::blz_get_unit_armor(which_unit.to_handle())
            };
        },
        {
            link: "BlzSetUnitArmor"
            name: blz_set_unit_armor
            raw: (which_unit: Handle<j_unit>, armor_amount: f32);
            user: (which_unit: &Agent<j_unit>, armor_amount: f32) {
                raw::blz_set_unit_armor(which_unit.to_handle(), armor_amount)
            };
        },
        {
            link: "BlzUnitHideAbility"
            name: blz_unit_hide_ability
            raw: (which_unit: Handle<j_unit>, abil_id: i32, flag: bool);
            user: (which_unit: &Agent<j_unit>, abil_id: i32, flag: bool) {
                raw::blz_unit_hide_ability(which_unit.to_handle(), abil_id, flag)
            };
        },
        {
            link: "BlzUnitDisableAbility"
            name: blz_unit_disable_ability
            raw: (which_unit: Handle<j_unit>, abil_id: i32, flag: bool, hide_u_i: bool);
            user: (which_unit: &Agent<j_unit>, abil_id: i32, flag: bool, hide_u_i: bool) {
                raw::blz_unit_disable_ability(which_unit.to_handle(), abil_id, flag, hide_u_i)
            };
        },
        {
            link: "BlzUnitCancelTimedLife"
            name: blz_unit_cancel_timed_life
            raw: (which_unit: Handle<j_unit>);
            user: (which_unit: &Agent<j_unit>) {
                raw::blz_unit_cancel_timed_life(which_unit.to_handle())
            };
        },
        {
            link: "BlzIsUnitSelectable"
            name: blz_is_unit_selectable
            raw: (which_unit: Handle<j_unit>) -> bool;
            user: (which_unit: &Agent<j_unit>) -> bool {
                raw::blz_is_unit_selectable(which_unit.to_handle())
            };
        },
        {
            link: "BlzIsUnitInvulnerable"
            name: blz_is_unit_invulnerable
            raw: (which_unit: Handle<j_unit>) -> bool;
            user: (which_unit: &Agent<j_unit>) -> bool {
                raw::blz_is_unit_invulnerable(which_unit.to_handle())
            };
        },
        {
            link: "BlzUnitInterruptAttack"
            name: blz_unit_interrupt_attack
            raw: (which_unit: Handle<j_unit>);
            user: (which_unit: &Agent<j_unit>) {
                raw::blz_unit_interrupt_attack(which_unit.to_handle())
            };
        },
        {
            link: "BlzGetUnitCollisionSize"
            name: blz_get_unit_collision_size
            raw: (which_unit: Handle<j_unit>) -> f32;
            user: (which_unit: &Agent<j_unit>) -> f32 {
                raw::blz_get_unit_collision_size(which_unit.to_handle())
            };
        },
        {
            link: "BlzGetAbilityManaCost"
            name: blz_get_ability_mana_cost
            raw: (abil_id: i32, level: i32) -> i32;
            user: (abil_id: i32, level: i32) -> i32 {
                raw::blz_get_ability_mana_cost(abil_id, level)
            };
        },
        {
            link: "BlzGetAbilityCooldown"
            name: blz_get_ability_cooldown
            raw: (abil_id: i32, level: i32) -> f32;
            user: (abil_id: i32, level: i32) -> f32 {
                raw::blz_get_ability_cooldown(abil_id, level)
            };
        },
        {
            link: "BlzSetUnitAbilityCooldown"
            name: blz_set_unit_ability_cooldown
            raw: (which_unit: Handle<j_unit>, abil_id: i32, level: i32, cooldown: f32);
            user: (which_unit: &Agent<j_unit>, abil_id: i32, level: i32, cooldown: f32) {
                raw::blz_set_unit_ability_cooldown(which_unit.to_handle(), abil_id, level, cooldown)
            };
        },
        {
            link: "BlzGetUnitAbilityCooldown"
            name: blz_get_unit_ability_cooldown
            raw: (which_unit: Handle<j_unit>, abil_id: i32, level: i32) -> f32;
            user: (which_unit: &Agent<j_unit>, abil_id: i32, level: i32) -> f32 {
                raw::blz_get_unit_ability_cooldown(which_unit.to_handle(), abil_id, level)
            };
        },
        {
            link: "BlzGetUnitAbilityCooldownRemaining"
            name: blz_get_unit_ability_cooldown_remaining
            raw: (which_unit: Handle<j_unit>, abil_id: i32) -> f32;
            user: (which_unit: &Agent<j_unit>, abil_id: i32) -> f32 {
                raw::blz_get_unit_ability_cooldown_remaining(which_unit.to_handle(), abil_id)
            };
        },
        {
            link: "BlzEndUnitAbilityCooldown"
            name: blz_end_unit_ability_cooldown
            raw: (which_unit: Handle<j_unit>, abil_code: i32);
            user: (which_unit: &Agent<j_unit>, abil_code: i32) {
                raw::blz_end_unit_ability_cooldown(which_unit.to_handle(), abil_code)
            };
        },
        {
            link: "BlzStartUnitAbilityCooldown"
            name: blz_start_unit_ability_cooldown
            raw: (which_unit: Handle<j_unit>, abil_code: i32, cooldown: f32);
            user: (which_unit: &Agent<j_unit>, abil_code: i32, cooldown: f32) {
                raw::blz_start_unit_ability_cooldown(which_unit.to_handle(), abil_code, cooldown)
            };
        },
        {
            link: "BlzGetUnitAbilityManaCost"
            name: blz_get_unit_ability_mana_cost
            raw: (which_unit: Handle<j_unit>, abil_id: i32, level: i32) -> i32;
            user: (which_unit: &Agent<j_unit>, abil_id: i32, level: i32) -> i32 {
                raw::blz_get_unit_ability_mana_cost(which_unit.to_handle(), abil_id, level)
            };
        },
        {
            link: "BlzSetUnitAbilityManaCost"
            name: blz_set_unit_ability_mana_cost
            raw: (which_unit: Handle<j_unit>, abil_id: i32, level: i32, mana_cost: i32);
            user: (which_unit: &Agent<j_unit>, abil_id: i32, level: i32, mana_cost: i32) {
                raw::blz_set_unit_ability_mana_cost(which_unit.to_handle(), abil_id, level, mana_cost)
            };
        },
        {
            link: "BlzGetLocalUnitZ"
            name: blz_get_local_unit_z
            raw: (which_unit: Handle<j_unit>) -> f32;
            user: (which_unit: &Agent<j_unit>) -> f32 {
                raw::blz_get_local_unit_z(which_unit.to_handle())
            };
        },
        {
            link: "BlzDecPlayerTechResearched"
            name: blz_dec_player_tech_researched
            raw: (which_player: Handle<j_player>, techid: i32, levels: i32);
            user: (which_player: &Agent<j_player>, techid: i32, levels: i32) {
                raw::blz_dec_player_tech_researched(which_player.to_handle(), techid, levels)
            };
        },
        {
            link: "BlzSetEventDamage"
            name: blz_set_event_damage
            raw: (damage: f32);
            user: (damage: f32) {
                raw::blz_set_event_damage(damage)
            };
        },
        {
            link: "BlzGetEventDamageTarget"
            name: blz_get_event_damage_target
            raw: () -> Handle<j_unit>;
            user: () -> Option<Weak<j_unit>> {
                raw::blz_get_event_damage_target().to_weak()
            };
        },
        {
            link: "BlzGetEventAttackType"
            name: blz_get_event_attack_type
            raw: () -> i32;
            user: () -> i32 {
                raw::blz_get_event_attack_type()
            };
        },
        {
            link: "BlzGetEventDamageType"
            name: blz_get_event_damage_type
            raw: () -> i32;
            user: () -> i32 {
                raw::blz_get_event_damage_type()
            };
        },
        {
            link: "BlzGetEventWeaponType"
            name: blz_get_event_weapon_type
            raw: () -> i32;
            user: () -> i32 {
                raw::blz_get_event_weapon_type()
            };
        },
        {
            link: "BlzSetEventAttackType"
            name: blz_set_event_attack_type
            raw: (attack_type: i32) -> bool;
            user: (attack_type: i32) -> bool {
                raw::blz_set_event_attack_type(attack_type)
            };
        },
        {
            link: "BlzSetEventDamageType"
            name: blz_set_event_damage_type
            raw: (damage_type: i32) -> bool;
            user: (damage_type: i32) -> bool {
                raw::blz_set_event_damage_type(damage_type)
            };
        },
        {
            link: "BlzSetEventWeaponType"
            name: blz_set_event_weapon_type
            raw: (weapon_type: i32) -> bool;
            user: (weapon_type: i32) -> bool {
                raw::blz_set_event_weapon_type(weapon_type)
            };
        },
        {
            link: "BlzGetEventIsAttack"
            name: blz_get_event_is_attack
            raw: () -> bool;
            user: () -> bool {
                raw::blz_get_event_is_attack()
            };
        },
        {
            link: "BlzGetUnitZ"
            name: blz_get_unit_z
            raw: (which_unit: Handle<j_unit>) -> f32;
            user: (which_unit: &Agent<j_unit>) -> f32 {
                raw::blz_get_unit_z(which_unit.to_handle())
            };
        },
        {
            link: "BlzEnableSelections"
            name: blz_enable_selections
            raw: (enable_selection: bool, enable_selection_circle: bool);
            user: (enable_selection: bool, enable_selection_circle: bool) {
                raw::blz_enable_selections(enable_selection, enable_selection_circle)
            };
        },
        {
            link: "BlzIsSelectionEnabled"
            name: blz_is_selection_enabled
            raw: () -> bool;
            user: () -> bool {
                raw::blz_is_selection_enabled()
            };
        },
        {
            link: "BlzIsSelectionCircleEnabled"
            name: blz_is_selection_circle_enabled
            raw: () -> bool;
            user: () -> bool {
                raw::blz_is_selection_circle_enabled()
            };
        },
        {
            link: "BlzCameraSetupApplyForceDurationSmooth"
            name: blz_camera_setup_apply_force_duration_smooth
            raw: (which_setup: Handle<j_camerasetup>, do_pan: bool, forced_duration: f32, ease_in_duration: f32, ease_out_duration: f32, smooth_factor: f32);
            user: (which_setup: Handle<j_camerasetup>, do_pan: bool, forced_duration: f32, ease_in_duration: f32, ease_out_duration: f32, smooth_factor: f32) {
                raw::blz_camera_setup_apply_force_duration_smooth(which_setup, do_pan, forced_duration, ease_in_duration, ease_out_duration, smooth_factor)
            };
        },
        {
            link: "BlzEnableTargetIndicator"
            name: blz_enable_target_indicator
            raw: (enable: bool);
            user: (enable: bool) {
                raw::blz_enable_target_indicator(enable)
            };
        },
        {
            link: "BlzIsTargetIndicatorEnabled"
            name: blz_is_target_indicator_enabled
            raw: () -> bool;
            user: () -> bool {
                raw::blz_is_target_indicator_enabled()
            };
        },
        {
            link: "BlzShowTerrain"
            name: blz_show_terrain
            raw: (show: bool);
            user: (show: bool) {
                raw::blz_show_terrain(show)
            };
        },
        {
            link: "BlzShowSkyBox"
            name: blz_show_sky_box
            raw: (show: bool);
            user: (show: bool) {
                raw::blz_show_sky_box(show)
            };
        },
        {
            link: "BlzStartRecording"
            name: blz_start_recording
            raw: (fps: i32);
            user: (fps: i32) {
                raw::blz_start_recording(fps)
            };
        },
        {
            link: "BlzEndRecording"
            name: blz_end_recording
            raw: ();
            user: () {
                raw::blz_end_recording()
            };
        },
        {
            link: "BlzShowUnitTeamGlow"
            name: blz_show_unit_team_glow
            raw: (which_unit: Handle<j_unit>, show: bool);
            user: (which_unit: &Agent<j_unit>, show: bool) {
                raw::blz_show_unit_team_glow(which_unit.to_handle(), show)
            };
        },
        {
            link: "BlzGetOriginFrame"
            name: blz_get_origin_frame
            raw: (frame_type: i32, index: i32) -> Handle<j_framehandle>;
            user: (frame_type: i32, index: i32) -> Handle<j_framehandle> {
                raw::blz_get_origin_frame(frame_type, index)
            };
        },
        {
            link: "BlzEnableUIAutoPosition"
            name: blz_enable_u_i_auto_position
            raw: (enable: bool);
            user: (enable: bool) {
                raw::blz_enable_u_i_auto_position(enable)
            };
        },
        {
            link: "BlzHideOriginFrames"
            name: blz_hide_origin_frames
            raw: (enable: bool);
            user: (enable: bool) {
                raw::blz_hide_origin_frames(enable)
            };
        },
        {
            link: "BlzConvertColor"
            name: blz_convert_color
            raw: (a: i32, r: i32, g: i32, b: i32) -> i32;
            user: (a: i32, r: i32, g: i32, b: i32) -> i32 {
                raw::blz_convert_color(a, r, g, b)
            };
        },
        {
            link: "BlzLoadTOCFile"
            name: blz_load_t_o_c_file
            raw: (t_o_c_file: *const i8) -> bool;
            user: (t_o_c_file: &CStr) -> bool {
                raw::blz_load_t_o_c_file(t_o_c_file.as_ptr())
            };
        },
        {
            link: "BlzCreateFrame"
            name: blz_create_frame
            raw: (name: *const i8, owner: Handle<j_framehandle>, priority: i32, create_context: i32) -> Handle<j_framehandle>;
            user: (name: &CStr, owner: Handle<j_framehandle>, priority: i32, create_context: i32) -> Handle<j_framehandle> {
                raw::blz_create_frame(name.as_ptr(), owner, priority, create_context)
            };
        },
        {
            link: "BlzCreateSimpleFrame"
            name: blz_create_simple_frame
            raw: (name: *const i8, owner: Handle<j_framehandle>, create_context: i32) -> Handle<j_framehandle>;
            user: (name: &CStr, owner: Handle<j_framehandle>, create_context: i32) -> Handle<j_framehandle> {
                raw::blz_create_simple_frame(name.as_ptr(), owner, create_context)
            };
        },
        {
            link: "BlzCreateFrameByType"
            name: blz_create_frame_by_type
            raw: (type_name: *const i8, name: *const i8, owner: Handle<j_framehandle>, inherits: *const i8, create_context: i32) -> Handle<j_framehandle>;
            user: (type_name: &CStr, name: &CStr, owner: Handle<j_framehandle>, inherits: &CStr, create_context: i32) -> Handle<j_framehandle> {
                raw::blz_create_frame_by_type(type_name.as_ptr(), name.as_ptr(), owner, inherits.as_ptr(), create_context)
            };
        },
        {
            link: "BlzDestroyFrame"
            name: blz_destroy_frame
            raw: (frame: Handle<j_framehandle>);
            user: (frame: Handle<j_framehandle>) {
                raw::blz_destroy_frame(frame)
            };
        },
        {
            link: "BlzFrameSetPoint"
            name: blz_frame_set_point
            raw: (frame: Handle<j_framehandle>, point: i32, relative: Handle<j_framehandle>, relative_point: i32, x: f32, y: f32);
            user: (frame: Handle<j_framehandle>, point: i32, relative: Handle<j_framehandle>, relative_point: i32, x: f32, y: f32) {
                raw::blz_frame_set_point(frame, point, relative, relative_point, x, y)
            };
        },
        {
            link: "BlzFrameSetAbsPoint"
            name: blz_frame_set_abs_point
            raw: (frame: Handle<j_framehandle>, point: i32, x: f32, y: f32);
            user: (frame: Handle<j_framehandle>, point: i32, x: f32, y: f32) {
                raw::blz_frame_set_abs_point(frame, point, x, y)
            };
        },
        {
            link: "BlzFrameClearAllPoints"
            name: blz_frame_clear_all_points
            raw: (frame: Handle<j_framehandle>);
            user: (frame: Handle<j_framehandle>) {
                raw::blz_frame_clear_all_points(frame)
            };
        },
        {
            link: "BlzFrameSetAllPoints"
            name: blz_frame_set_all_points
            raw: (frame: Handle<j_framehandle>, relative: Handle<j_framehandle>);
            user: (frame: Handle<j_framehandle>, relative: Handle<j_framehandle>) {
                raw::blz_frame_set_all_points(frame, relative)
            };
        },
        {
            link: "BlzFrameSetVisible"
            name: blz_frame_set_visible
            raw: (frame: Handle<j_framehandle>, visible: bool);
            user: (frame: Handle<j_framehandle>, visible: bool) {
                raw::blz_frame_set_visible(frame, visible)
            };
        },
        {
            link: "BlzFrameIsVisible"
            name: blz_frame_is_visible
            raw: (frame: Handle<j_framehandle>) -> bool;
            user: (frame: Handle<j_framehandle>) -> bool {
                raw::blz_frame_is_visible(frame)
            };
        },
        {
            link: "BlzGetFrameByName"
            name: blz_get_frame_by_name
            raw: (name: *const i8, create_context: i32) -> Handle<j_framehandle>;
            user: (name: &CStr, create_context: i32) -> Handle<j_framehandle> {
                raw::blz_get_frame_by_name(name.as_ptr(), create_context)
            };
        },
        {
            link: "BlzFrameGetName"
            name: blz_frame_get_name
            raw: (frame: Handle<j_framehandle>, out: *mut i8, out_len: *mut usize);
            user: (frame: Handle<j_framehandle>) -> String {
                raw::blz_frame_get_name(frame, stringret::ptr(), stringret::reset_len()); stringret::read_string()
            };
        },
        {
            link: "BlzFrameClick"
            name: blz_frame_click
            raw: (frame: Handle<j_framehandle>);
            user: (frame: Handle<j_framehandle>) {
                raw::blz_frame_click(frame)
            };
        },
        {
            link: "BlzFrameSetText"
            name: blz_frame_set_text
            raw: (frame: Handle<j_framehandle>, text: *const i8);
            user: (frame: Handle<j_framehandle>, text: &CStr) {
                raw::blz_frame_set_text(frame, text.as_ptr())
            };
        },
        {
            link: "BlzFrameGetText"
            name: blz_frame_get_text
            raw: (frame: Handle<j_framehandle>, out: *mut i8, out_len: *mut usize);
            user: (frame: Handle<j_framehandle>) -> String {
                raw::blz_frame_get_text(frame, stringret::ptr(), stringret::reset_len()); stringret::read_string()
            };
        },
        {
            link: "BlzFrameAddText"
            name: blz_frame_add_text
            raw: (frame: Handle<j_framehandle>, text: *const i8);
            user: (frame: Handle<j_framehandle>, text: &CStr) {
                raw::blz_frame_add_text(frame, text.as_ptr())
            };
        },
        {
            link: "BlzFrameSetTextSizeLimit"
            name: blz_frame_set_text_size_limit
            raw: (frame: Handle<j_framehandle>, size: i32);
            user: (frame: Handle<j_framehandle>, size: i32) {
                raw::blz_frame_set_text_size_limit(frame, size)
            };
        },
        {
            link: "BlzFrameGetTextSizeLimit"
            name: blz_frame_get_text_size_limit
            raw: (frame: Handle<j_framehandle>) -> i32;
            user: (frame: Handle<j_framehandle>) -> i32 {
                raw::blz_frame_get_text_size_limit(frame)
            };
        },
        {
            link: "BlzFrameSetTextColor"
            name: blz_frame_set_text_color
            raw: (frame: Handle<j_framehandle>, color: i32);
            user: (frame: Handle<j_framehandle>, color: i32) {
                raw::blz_frame_set_text_color(frame, color)
            };
        },
        {
            link: "BlzFrameSetFocus"
            name: blz_frame_set_focus
            raw: (frame: Handle<j_framehandle>, flag: bool);
            user: (frame: Handle<j_framehandle>, flag: bool) {
                raw::blz_frame_set_focus(frame, flag)
            };
        },
        {
            link: "BlzFrameSetModel"
            name: blz_frame_set_model
            raw: (frame: Handle<j_framehandle>, model_file: *const i8, camera_index: i32);
            user: (frame: Handle<j_framehandle>, model_file: &CStr, camera_index: i32) {
                raw::blz_frame_set_model(frame, model_file.as_ptr(), camera_index)
            };
        },
        {
            link: "BlzFrameSetEnable"
            name: blz_frame_set_enable
            raw: (frame: Handle<j_framehandle>, enabled: bool);
            user: (frame: Handle<j_framehandle>, enabled: bool) {
                raw::blz_frame_set_enable(frame, enabled)
            };
        },
        {
            link: "BlzFrameGetEnable"
            name: blz_frame_get_enable
            raw: (frame: Handle<j_framehandle>) -> bool;
            user: (frame: Handle<j_framehandle>) -> bool {
                raw::blz_frame_get_enable(frame)
            };
        },
        {
            link: "BlzFrameSetAlpha"
            name: blz_frame_set_alpha
            raw: (frame: Handle<j_framehandle>, alpha: i32);
            user: (frame: Handle<j_framehandle>, alpha: i32) {
                raw::blz_frame_set_alpha(frame, alpha)
            };
        },
        {
            link: "BlzFrameGetAlpha"
            name: blz_frame_get_alpha
            raw: (frame: Handle<j_framehandle>) -> i32;
            user: (frame: Handle<j_framehandle>) -> i32 {
                raw::blz_frame_get_alpha(frame)
            };
        },
        {
            link: "BlzFrameSetSpriteAnimate"
            name: blz_frame_set_sprite_animate
            raw: (frame: Handle<j_framehandle>, primary_prop: i32, flags: i32);
            user: (frame: Handle<j_framehandle>, primary_prop: i32, flags: i32) {
                raw::blz_frame_set_sprite_animate(frame, primary_prop, flags)
            };
        },
        {
            link: "BlzFrameSetTexture"
            name: blz_frame_set_texture
            raw: (frame: Handle<j_framehandle>, tex_file: *const i8, flag: i32, blend: bool);
            user: (frame: Handle<j_framehandle>, tex_file: &CStr, flag: i32, blend: bool) {
                raw::blz_frame_set_texture(frame, tex_file.as_ptr(), flag, blend)
            };
        },
        {
            link: "BlzFrameSetScale"
            name: blz_frame_set_scale
            raw: (frame: Handle<j_framehandle>, scale: f32);
            user: (frame: Handle<j_framehandle>, scale: f32) {
                raw::blz_frame_set_scale(frame, scale)
            };
        },
        {
            link: "BlzFrameSetTooltip"
            name: blz_frame_set_tooltip
            raw: (frame: Handle<j_framehandle>, tooltip: Handle<j_framehandle>);
            user: (frame: Handle<j_framehandle>, tooltip: Handle<j_framehandle>) {
                raw::blz_frame_set_tooltip(frame, tooltip)
            };
        },
        {
            link: "BlzFrameCageMouse"
            name: blz_frame_cage_mouse
            raw: (frame: Handle<j_framehandle>, enable: bool);
            user: (frame: Handle<j_framehandle>, enable: bool) {
                raw::blz_frame_cage_mouse(frame, enable)
            };
        },
        {
            link: "BlzFrameSetValue"
            name: blz_frame_set_value
            raw: (frame: Handle<j_framehandle>, value: f32);
            user: (frame: Handle<j_framehandle>, value: f32) {
                raw::blz_frame_set_value(frame, value)
            };
        },
        {
            link: "BlzFrameGetValue"
            name: blz_frame_get_value
            raw: (frame: Handle<j_framehandle>) -> f32;
            user: (frame: Handle<j_framehandle>) -> f32 {
                raw::blz_frame_get_value(frame)
            };
        },
        {
            link: "BlzFrameSetMinMaxValue"
            name: blz_frame_set_min_max_value
            raw: (frame: Handle<j_framehandle>, min_value: f32, max_value: f32);
            user: (frame: Handle<j_framehandle>, min_value: f32, max_value: f32) {
                raw::blz_frame_set_min_max_value(frame, min_value, max_value)
            };
        },
        {
            link: "BlzFrameSetStepSize"
            name: blz_frame_set_step_size
            raw: (frame: Handle<j_framehandle>, step_size: f32);
            user: (frame: Handle<j_framehandle>, step_size: f32) {
                raw::blz_frame_set_step_size(frame, step_size)
            };
        },
        {
            link: "BlzFrameSetSize"
            name: blz_frame_set_size
            raw: (frame: Handle<j_framehandle>, width: f32, height: f32);
            user: (frame: Handle<j_framehandle>, width: f32, height: f32) {
                raw::blz_frame_set_size(frame, width, height)
            };
        },
        {
            link: "BlzFrameSetVertexColor"
            name: blz_frame_set_vertex_color
            raw: (frame: Handle<j_framehandle>, color: i32);
            user: (frame: Handle<j_framehandle>, color: i32) {
                raw::blz_frame_set_vertex_color(frame, color)
            };
        },
        {
            link: "BlzFrameSetLevel"
            name: blz_frame_set_level
            raw: (frame: Handle<j_framehandle>, level: i32);
            user: (frame: Handle<j_framehandle>, level: i32) {
                raw::blz_frame_set_level(frame, level)
            };
        },
        {
            link: "BlzFrameSetParent"
            name: blz_frame_set_parent
            raw: (frame: Handle<j_framehandle>, parent: Handle<j_framehandle>);
            user: (frame: Handle<j_framehandle>, parent: Handle<j_framehandle>) {
                raw::blz_frame_set_parent(frame, parent)
            };
        },
        {
            link: "BlzFrameGetParent"
            name: blz_frame_get_parent
            raw: (frame: Handle<j_framehandle>) -> Handle<j_framehandle>;
            user: (frame: Handle<j_framehandle>) -> Handle<j_framehandle> {
                raw::blz_frame_get_parent(frame)
            };
        },
        {
            link: "BlzFrameGetHeight"
            name: blz_frame_get_height
            raw: (frame: Handle<j_framehandle>) -> f32;
            user: (frame: Handle<j_framehandle>) -> f32 {
                raw::blz_frame_get_height(frame)
            };
        },
        {
            link: "BlzFrameGetWidth"
            name: blz_frame_get_width
            raw: (frame: Handle<j_framehandle>) -> f32;
            user: (frame: Handle<j_framehandle>) -> f32 {
                raw::blz_frame_get_width(frame)
            };
        },
        {
            link: "BlzFrameSetFont"
            name: blz_frame_set_font
            raw: (frame: Handle<j_framehandle>, file_name: *const i8, height: f32, flags: i32);
            user: (frame: Handle<j_framehandle>, file_name: &CStr, height: f32, flags: i32) {
                raw::blz_frame_set_font(frame, file_name.as_ptr(), height, flags)
            };
        },
        {
            link: "BlzFrameSetTextAlignment"
            name: blz_frame_set_text_alignment
            raw: (frame: Handle<j_framehandle>, vert: i32, horz: i32);
            user: (frame: Handle<j_framehandle>, vert: i32, horz: i32) {
                raw::blz_frame_set_text_alignment(frame, vert, horz)
            };
        },
        {
            link: "BlzFrameGetChildrenCount"
            name: blz_frame_get_children_count
            raw: (frame: Handle<j_framehandle>) -> i32;
            user: (frame: Handle<j_framehandle>) -> i32 {
                raw::blz_frame_get_children_count(frame)
            };
        },
        {
            link: "BlzFrameGetChild"
            name: blz_frame_get_child
            raw: (frame: Handle<j_framehandle>, index: i32) -> Handle<j_framehandle>;
            user: (frame: Handle<j_framehandle>, index: i32) -> Handle<j_framehandle> {
                raw::blz_frame_get_child(frame, index)
            };
        },
        {
            link: "BlzTriggerRegisterFrameEvent"
            name: blz_trigger_register_frame_event
            raw: (which_trigger: Handle<j_trigger>, frame: Handle<j_framehandle>, event_id: i32) -> Handle<j_event>;
            user: (which_trigger: &Agent<j_trigger>, frame: Handle<j_framehandle>, event_id: i32) -> Option<Weak<j_event>> {
                raw::blz_trigger_register_frame_event(which_trigger.to_handle(), frame, event_id).to_weak()
            };
        },
        {
            link: "BlzGetTriggerFrame"
            name: blz_get_trigger_frame
            raw: () -> Handle<j_framehandle>;
            user: () -> Handle<j_framehandle> {
                raw::blz_get_trigger_frame()
            };
        },
        {
            link: "BlzGetTriggerFrameEvent"
            name: blz_get_trigger_frame_event
            raw: () -> i32;
            user: () -> i32 {
                raw::blz_get_trigger_frame_event()
            };
        },
        {
            link: "BlzGetTriggerFrameValue"
            name: blz_get_trigger_frame_value
            raw: () -> f32;
            user: () -> f32 {
                raw::blz_get_trigger_frame_value()
            };
        },
        {
            link: "BlzGetTriggerFrameText"
            name: blz_get_trigger_frame_text
            raw: (out: *mut i8, out_len: *mut usize);
            user: () -> String {
                raw::blz_get_trigger_frame_text(stringret::ptr(), stringret::reset_len()); stringret::read_string()
            };
        },
        {
            link: "BlzTriggerRegisterPlayerSyncEvent"
            name: blz_trigger_register_player_sync_event
            raw: (which_trigger: Handle<j_trigger>, which_player: Handle<j_player>, prefix: *const i8, from_server: bool) -> Handle<j_event>;
            user: (which_trigger: &Agent<j_trigger>, which_player: &Agent<j_player>, prefix: &CStr, from_server: bool) -> Option<Weak<j_event>> {
                raw::blz_trigger_register_player_sync_event(which_trigger.to_handle(), which_player.to_handle(), prefix.as_ptr(), from_server).to_weak()
            };
        },
        {
            link: "BlzSendSyncData"
            name: blz_send_sync_data
            raw: (prefix: *const i8, data: *const i8) -> bool;
            user: (prefix: &CStr, data: &CStr) -> bool {
                raw::blz_send_sync_data(prefix.as_ptr(), data.as_ptr())
            };
        },
        {
            link: "BlzGetTriggerSyncPrefix"
            name: blz_get_trigger_sync_prefix
            raw: (out: *mut i8, out_len: *mut usize);
            user: () -> String {
                raw::blz_get_trigger_sync_prefix(stringret::ptr(), stringret::reset_len()); stringret::read_string()
            };
        },
        {
            link: "BlzGetTriggerSyncData"
            name: blz_get_trigger_sync_data
            raw: (out: *mut i8, out_len: *mut usize);
            user: () -> String {
                raw::blz_get_trigger_sync_data(stringret::ptr(), stringret::reset_len()); stringret::read_string()
            };
        },
        {
            link: "BlzTriggerRegisterPlayerKeyEvent"
            name: blz_trigger_register_player_key_event
            raw: (which_trigger: Handle<j_trigger>, which_player: Handle<j_player>, key: i32, meta_key: i32, key_down: bool) -> Handle<j_event>;
            user: (which_trigger: &Agent<j_trigger>, which_player: &Agent<j_player>, key: i32, meta_key: i32, key_down: bool) -> Option<Weak<j_event>> {
                raw::blz_trigger_register_player_key_event(which_trigger.to_handle(), which_player.to_handle(), key, meta_key, key_down).to_weak()
            };
        },
        {
            link: "BlzGetTriggerPlayerKey"
            name: blz_get_trigger_player_key
            raw: () -> i32;
            user: () -> i32 {
                raw::blz_get_trigger_player_key()
            };
        },
        {
            link: "BlzGetTriggerPlayerMetaKey"
            name: blz_get_trigger_player_meta_key
            raw: () -> i32;
            user: () -> i32 {
                raw::blz_get_trigger_player_meta_key()
            };
        },
        {
            link: "BlzGetTriggerPlayerIsKeyDown"
            name: blz_get_trigger_player_is_key_down
            raw: () -> bool;
            user: () -> bool {
                raw::blz_get_trigger_player_is_key_down()
            };
        },
        {
            link: "BlzEnableCursor"
            name: blz_enable_cursor
            raw: (enable: bool);
            user: (enable: bool) {
                raw::blz_enable_cursor(enable)
            };
        },
        {
            link: "BlzSetMousePos"
            name: blz_set_mouse_pos
            raw: (x: i32, y: i32);
            user: (x: i32, y: i32) {
                raw::blz_set_mouse_pos(x, y)
            };
        },
        {
            link: "BlzGetLocalClientWidth"
            name: blz_get_local_client_width
            raw: () -> i32;
            user: () -> i32 {
                raw::blz_get_local_client_width()
            };
        },
        {
            link: "BlzGetLocalClientHeight"
            name: blz_get_local_client_height
            raw: () -> i32;
            user: () -> i32 {
                raw::blz_get_local_client_height()
            };
        },
        {
            link: "BlzIsLocalClientActive"
            name: blz_is_local_client_active
            raw: () -> bool;
            user: () -> bool {
                raw::blz_is_local_client_active()
            };
        },
        {
            link: "BlzGetMouseFocusUnit"
            name: blz_get_mouse_focus_unit
            raw: () -> Handle<j_unit>;
            user: () -> Option<Weak<j_unit>> {
                raw::blz_get_mouse_focus_unit().to_weak()
            };
        },
        {
            link: "BlzChangeMinimapTerrainTex"
            name: blz_change_minimap_terrain_tex
            raw: (tex_file: *const i8) -> bool;
            user: (tex_file: &CStr) -> bool {
                raw::blz_change_minimap_terrain_tex(tex_file.as_ptr())
            };
        },
        {
            link: "BlzGetLocale"
            name: blz_get_locale
            raw: (out: *mut i8, out_len: *mut usize);
            user: () -> String {
                raw::blz_get_locale(stringret::ptr(), stringret::reset_len()); stringret::read_string()
            };
        },
        {
            link: "BlzGetSpecialEffectScale"
            name: blz_get_special_effect_scale
            raw: (which_effect: Handle<j_effect>) -> f32;
            user: (which_effect: &Agent<j_effect>) -> f32 {
                raw::blz_get_special_effect_scale(which_effect.to_handle())
            };
        },
        {
            link: "BlzSetSpecialEffectMatrixScale"
            name: blz_set_special_effect_matrix_scale
            raw: (which_effect: Handle<j_effect>, x: f32, y: f32, z: f32);
            user: (which_effect: &Agent<j_effect>, x: f32, y: f32, z: f32) {
                raw::blz_set_special_effect_matrix_scale(which_effect.to_handle(), x, y, z)
            };
        },
        {
            link: "BlzResetSpecialEffectMatrix"
            name: blz_reset_special_effect_matrix
            raw: (which_effect: Handle<j_effect>);
            user: (which_effect: &Agent<j_effect>) {
                raw::blz_reset_special_effect_matrix(which_effect.to_handle())
            };
        },
        {
            link: "BlzGetUnitAbility"
            name: blz_get_unit_ability
            raw: (which_unit: Handle<j_unit>, abil_id: i32) -> Handle<j_ability>;
            user: (which_unit: &Agent<j_unit>, abil_id: i32) -> Option<Weak<j_ability>> {
                raw::blz_get_unit_ability(which_unit.to_handle(), abil_id).to_weak()
            };
        },
        {
            link: "BlzGetUnitAbilityByIndex"
            name: blz_get_unit_ability_by_index
            raw: (which_unit: Handle<j_unit>, index: i32) -> Handle<j_ability>;
            user: (which_unit: &Agent<j_unit>, index: i32) -> Option<Weak<j_ability>> {
                raw::blz_get_unit_ability_by_index(which_unit.to_handle(), index).to_weak()
            };
        },
        {
            link: "BlzGetAbilityId"
            name: blz_get_ability_id
            raw: (which_ability: Handle<j_ability>) -> i32;
            user: (which_ability: &Agent<j_ability>) -> i32 {
                raw::blz_get_ability_id(which_ability.to_handle())
            };
        },
        {
            link: "BlzDisplayChatMessage"
            name: blz_display_chat_message
            raw: (which_player: Handle<j_player>, recipient: i32, message: *const i8);
            user: (which_player: &Agent<j_player>, recipient: i32, message: &CStr) {
                raw::blz_display_chat_message(which_player.to_handle(), recipient, message.as_ptr())
            };
        },
        {
            link: "BlzPauseUnitEx"
            name: blz_pause_unit_ex
            raw: (which_unit: Handle<j_unit>, flag: bool);
            user: (which_unit: &Agent<j_unit>, flag: bool) {
                raw::blz_pause_unit_ex(which_unit.to_handle(), flag)
            };
        },
        {
            link: "BlzSetUnitFacingEx"
            name: blz_set_unit_facing_ex
            raw: (which_unit: Handle<j_unit>, facing_angle: f32);
            user: (which_unit: &Agent<j_unit>, facing_angle: f32) {
                raw::blz_set_unit_facing_ex(which_unit.to_handle(), facing_angle)
            };
        },
        {
            link: "CreateCommandButtonEffect"
            name: create_command_button_effect
            raw: (ability_id: i32, order: *const i8) -> Handle<j_commandbuttoneffect>;
            user: (ability_id: i32, order: &CStr) -> Handle<j_commandbuttoneffect> {
                raw::create_command_button_effect(ability_id, order.as_ptr())
            };
        },
        {
            link: "CreateUpgradeCommandButtonEffect"
            name: create_upgrade_command_button_effect
            raw: (which_uprgade: i32) -> Handle<j_commandbuttoneffect>;
            user: (which_uprgade: i32) -> Handle<j_commandbuttoneffect> {
                raw::create_upgrade_command_button_effect(which_uprgade)
            };
        },
        {
            link: "CreateLearnCommandButtonEffect"
            name: create_learn_command_button_effect
            raw: (ability_id: i32) -> Handle<j_commandbuttoneffect>;
            user: (ability_id: i32) -> Handle<j_commandbuttoneffect> {
                raw::create_learn_command_button_effect(ability_id)
            };
        },
        {
            link: "DestroyCommandButtonEffect"
            name: destroy_command_button_effect
            raw: (which_effect: Handle<j_commandbuttoneffect>);
            user: (which_effect: Handle<j_commandbuttoneffect>) {
                raw::destroy_command_button_effect(which_effect)
            };
        },
        {
            link: "BlzBitOr"
            name: blz_bit_or
            raw: (x: i32, y: i32) -> i32;
            user: (x: i32, y: i32) -> i32 {
                raw::blz_bit_or(x, y)
            };
        },
        {
            link: "BlzBitAnd"
            name: blz_bit_and
            raw: (x: i32, y: i32) -> i32;
            user: (x: i32, y: i32) -> i32 {
                raw::blz_bit_and(x, y)
            };
        },
        {
            link: "BlzBitXor"
            name: blz_bit_xor
            raw: (x: i32, y: i32) -> i32;
            user: (x: i32, y: i32) -> i32 {
                raw::blz_bit_xor(x, y)
            };
        },
        {
            link: "BlzGetAbilityBooleanField"
            name: blz_get_ability_boolean_field
            raw: (which_ability: Handle<j_ability>, which_field: i32) -> bool;
            user: (which_ability: &Agent<j_ability>, which_field: i32) -> bool {
                raw::blz_get_ability_boolean_field(which_ability.to_handle(), which_field)
            };
        },
        {
            link: "BlzGetAbilityIntegerField"
            name: blz_get_ability_integer_field
            raw: (which_ability: Handle<j_ability>, which_field: i32) -> i32;
            user: (which_ability: &Agent<j_ability>, which_field: i32) -> i32 {
                raw::blz_get_ability_integer_field(which_ability.to_handle(), which_field)
            };
        },
        {
            link: "BlzGetAbilityRealField"
            name: blz_get_ability_real_field
            raw: (which_ability: Handle<j_ability>, which_field: i32) -> f32;
            user: (which_ability: &Agent<j_ability>, which_field: i32) -> f32 {
                raw::blz_get_ability_real_field(which_ability.to_handle(), which_field)
            };
        },
        {
            link: "BlzGetAbilityStringField"
            name: blz_get_ability_string_field
            raw: (which_ability: Handle<j_ability>, which_field: i32, out: *mut i8, out_len: *mut usize);
            user: (which_ability: &Agent<j_ability>, which_field: i32) -> String {
                raw::blz_get_ability_string_field(which_ability.to_handle(), which_field, stringret::ptr(), stringret::reset_len()); stringret::read_string()
            };
        },
        {
            link: "BlzGetAbilityBooleanLevelField"
            name: blz_get_ability_boolean_level_field
            raw: (which_ability: Handle<j_ability>, which_field: i32, level: i32) -> bool;
            user: (which_ability: &Agent<j_ability>, which_field: i32, level: i32) -> bool {
                raw::blz_get_ability_boolean_level_field(which_ability.to_handle(), which_field, level)
            };
        },
        {
            link: "BlzGetAbilityIntegerLevelField"
            name: blz_get_ability_integer_level_field
            raw: (which_ability: Handle<j_ability>, which_field: i32, level: i32) -> i32;
            user: (which_ability: &Agent<j_ability>, which_field: i32, level: i32) -> i32 {
                raw::blz_get_ability_integer_level_field(which_ability.to_handle(), which_field, level)
            };
        },
        {
            link: "BlzGetAbilityRealLevelField"
            name: blz_get_ability_real_level_field
            raw: (which_ability: Handle<j_ability>, which_field: i32, level: i32) -> f32;
            user: (which_ability: &Agent<j_ability>, which_field: i32, level: i32) -> f32 {
                raw::blz_get_ability_real_level_field(which_ability.to_handle(), which_field, level)
            };
        },
        {
            link: "BlzGetAbilityStringLevelField"
            name: blz_get_ability_string_level_field
            raw: (which_ability: Handle<j_ability>, which_field: i32, level: i32, out: *mut i8, out_len: *mut usize);
            user: (which_ability: &Agent<j_ability>, which_field: i32, level: i32) -> String {
                raw::blz_get_ability_string_level_field(which_ability.to_handle(), which_field, level, stringret::ptr(), stringret::reset_len()); stringret::read_string()
            };
        },
        {
            link: "BlzGetAbilityBooleanLevelArrayField"
            name: blz_get_ability_boolean_level_array_field
            raw: (which_ability: Handle<j_ability>, which_field: i32, level: i32, index: i32) -> bool;
            user: (which_ability: &Agent<j_ability>, which_field: i32, level: i32, index: i32) -> bool {
                raw::blz_get_ability_boolean_level_array_field(which_ability.to_handle(), which_field, level, index)
            };
        },
        {
            link: "BlzGetAbilityIntegerLevelArrayField"
            name: blz_get_ability_integer_level_array_field
            raw: (which_ability: Handle<j_ability>, which_field: i32, level: i32, index: i32) -> i32;
            user: (which_ability: &Agent<j_ability>, which_field: i32, level: i32, index: i32) -> i32 {
                raw::blz_get_ability_integer_level_array_field(which_ability.to_handle(), which_field, level, index)
            };
        },
        {
            link: "BlzGetAbilityRealLevelArrayField"
            name: blz_get_ability_real_level_array_field
            raw: (which_ability: Handle<j_ability>, which_field: i32, level: i32, index: i32) -> f32;
            user: (which_ability: &Agent<j_ability>, which_field: i32, level: i32, index: i32) -> f32 {
                raw::blz_get_ability_real_level_array_field(which_ability.to_handle(), which_field, level, index)
            };
        },
        {
            link: "BlzGetAbilityStringLevelArrayField"
            name: blz_get_ability_string_level_array_field
            raw: (which_ability: Handle<j_ability>, which_field: i32, level: i32, index: i32, out: *mut i8, out_len: *mut usize);
            user: (which_ability: &Agent<j_ability>, which_field: i32, level: i32, index: i32) -> String {
                raw::blz_get_ability_string_level_array_field(which_ability.to_handle(), which_field, level, index, stringret::ptr(), stringret::reset_len()); stringret::read_string()
            };
        },
        {
            link: "BlzSetAbilityBooleanField"
            name: blz_set_ability_boolean_field
            raw: (which_ability: Handle<j_ability>, which_field: i32, value: bool) -> bool;
            user: (which_ability: &Agent<j_ability>, which_field: i32, value: bool) -> bool {
                raw::blz_set_ability_boolean_field(which_ability.to_handle(), which_field, value)
            };
        },
        {
            link: "BlzSetAbilityIntegerField"
            name: blz_set_ability_integer_field
            raw: (which_ability: Handle<j_ability>, which_field: i32, value: i32) -> bool;
            user: (which_ability: &Agent<j_ability>, which_field: i32, value: i32) -> bool {
                raw::blz_set_ability_integer_field(which_ability.to_handle(), which_field, value)
            };
        },
        {
            link: "BlzSetAbilityRealField"
            name: blz_set_ability_real_field
            raw: (which_ability: Handle<j_ability>, which_field: i32, value: f32) -> bool;
            user: (which_ability: &Agent<j_ability>, which_field: i32, value: f32) -> bool {
                raw::blz_set_ability_real_field(which_ability.to_handle(), which_field, value)
            };
        },
        {
            link: "BlzSetAbilityStringField"
            name: blz_set_ability_string_field
            raw: (which_ability: Handle<j_ability>, which_field: i32, value: *const i8) -> bool;
            user: (which_ability: &Agent<j_ability>, which_field: i32, value: &CStr) -> bool {
                raw::blz_set_ability_string_field(which_ability.to_handle(), which_field, value.as_ptr())
            };
        },
        {
            link: "BlzSetAbilityBooleanLevelField"
            name: blz_set_ability_boolean_level_field
            raw: (which_ability: Handle<j_ability>, which_field: i32, level: i32, value: bool) -> bool;
            user: (which_ability: &Agent<j_ability>, which_field: i32, level: i32, value: bool) -> bool {
                raw::blz_set_ability_boolean_level_field(which_ability.to_handle(), which_field, level, value)
            };
        },
        {
            link: "BlzSetAbilityIntegerLevelField"
            name: blz_set_ability_integer_level_field
            raw: (which_ability: Handle<j_ability>, which_field: i32, level: i32, value: i32) -> bool;
            user: (which_ability: &Agent<j_ability>, which_field: i32, level: i32, value: i32) -> bool {
                raw::blz_set_ability_integer_level_field(which_ability.to_handle(), which_field, level, value)
            };
        },
        {
            link: "BlzSetAbilityRealLevelField"
            name: blz_set_ability_real_level_field
            raw: (which_ability: Handle<j_ability>, which_field: i32, level: i32, value: f32) -> bool;
            user: (which_ability: &Agent<j_ability>, which_field: i32, level: i32, value: f32) -> bool {
                raw::blz_set_ability_real_level_field(which_ability.to_handle(), which_field, level, value)
            };
        },
        {
            link: "BlzSetAbilityStringLevelField"
            name: blz_set_ability_string_level_field
            raw: (which_ability: Handle<j_ability>, which_field: i32, level: i32, value: *const i8) -> bool;
            user: (which_ability: &Agent<j_ability>, which_field: i32, level: i32, value: &CStr) -> bool {
                raw::blz_set_ability_string_level_field(which_ability.to_handle(), which_field, level, value.as_ptr())
            };
        },
        {
            link: "BlzSetAbilityBooleanLevelArrayField"
            name: blz_set_ability_boolean_level_array_field
            raw: (which_ability: Handle<j_ability>, which_field: i32, level: i32, index: i32, value: bool) -> bool;
            user: (which_ability: &Agent<j_ability>, which_field: i32, level: i32, index: i32, value: bool) -> bool {
                raw::blz_set_ability_boolean_level_array_field(which_ability.to_handle(), which_field, level, index, value)
            };
        },
        {
            link: "BlzSetAbilityIntegerLevelArrayField"
            name: blz_set_ability_integer_level_array_field
            raw: (which_ability: Handle<j_ability>, which_field: i32, level: i32, index: i32, value: i32) -> bool;
            user: (which_ability: &Agent<j_ability>, which_field: i32, level: i32, index: i32, value: i32) -> bool {
                raw::blz_set_ability_integer_level_array_field(which_ability.to_handle(), which_field, level, index, value)
            };
        },
        {
            link: "BlzSetAbilityRealLevelArrayField"
            name: blz_set_ability_real_level_array_field
            raw: (which_ability: Handle<j_ability>, which_field: i32, level: i32, index: i32, value: f32) -> bool;
            user: (which_ability: &Agent<j_ability>, which_field: i32, level: i32, index: i32, value: f32) -> bool {
                raw::blz_set_ability_real_level_array_field(which_ability.to_handle(), which_field, level, index, value)
            };
        },
        {
            link: "BlzSetAbilityStringLevelArrayField"
            name: blz_set_ability_string_level_array_field
            raw: (which_ability: Handle<j_ability>, which_field: i32, level: i32, index: i32, value: *const i8) -> bool;
            user: (which_ability: &Agent<j_ability>, which_field: i32, level: i32, index: i32, value: &CStr) -> bool {
                raw::blz_set_ability_string_level_array_field(which_ability.to_handle(), which_field, level, index, value.as_ptr())
            };
        },
        {
            link: "BlzAddAbilityBooleanLevelArrayField"
            name: blz_add_ability_boolean_level_array_field
            raw: (which_ability: Handle<j_ability>, which_field: i32, level: i32, value: bool) -> bool;
            user: (which_ability: &Agent<j_ability>, which_field: i32, level: i32, value: bool) -> bool {
                raw::blz_add_ability_boolean_level_array_field(which_ability.to_handle(), which_field, level, value)
            };
        },
        {
            link: "BlzAddAbilityIntegerLevelArrayField"
            name: blz_add_ability_integer_level_array_field
            raw: (which_ability: Handle<j_ability>, which_field: i32, level: i32, value: i32) -> bool;
            user: (which_ability: &Agent<j_ability>, which_field: i32, level: i32, value: i32) -> bool {
                raw::blz_add_ability_integer_level_array_field(which_ability.to_handle(), which_field, level, value)
            };
        },
        {
            link: "BlzAddAbilityRealLevelArrayField"
            name: blz_add_ability_real_level_array_field
            raw: (which_ability: Handle<j_ability>, which_field: i32, level: i32, value: f32) -> bool;
            user: (which_ability: &Agent<j_ability>, which_field: i32, level: i32, value: f32) -> bool {
                raw::blz_add_ability_real_level_array_field(which_ability.to_handle(), which_field, level, value)
            };
        },
        {
            link: "BlzAddAbilityStringLevelArrayField"
            name: blz_add_ability_string_level_array_field
            raw: (which_ability: Handle<j_ability>, which_field: i32, level: i32, value: *const i8) -> bool;
            user: (which_ability: &Agent<j_ability>, which_field: i32, level: i32, value: &CStr) -> bool {
                raw::blz_add_ability_string_level_array_field(which_ability.to_handle(), which_field, level, value.as_ptr())
            };
        },
        {
            link: "BlzRemoveAbilityBooleanLevelArrayField"
            name: blz_remove_ability_boolean_level_array_field
            raw: (which_ability: Handle<j_ability>, which_field: i32, level: i32, value: bool) -> bool;
            user: (which_ability: &Agent<j_ability>, which_field: i32, level: i32, value: bool) -> bool {
                raw::blz_remove_ability_boolean_level_array_field(which_ability.to_handle(), which_field, level, value)
            };
        },
        {
            link: "BlzRemoveAbilityIntegerLevelArrayField"
            name: blz_remove_ability_integer_level_array_field
            raw: (which_ability: Handle<j_ability>, which_field: i32, level: i32, value: i32) -> bool;
            user: (which_ability: &Agent<j_ability>, which_field: i32, level: i32, value: i32) -> bool {
                raw::blz_remove_ability_integer_level_array_field(which_ability.to_handle(), which_field, level, value)
            };
        },
        {
            link: "BlzRemoveAbilityRealLevelArrayField"
            name: blz_remove_ability_real_level_array_field
            raw: (which_ability: Handle<j_ability>, which_field: i32, level: i32, value: f32) -> bool;
            user: (which_ability: &Agent<j_ability>, which_field: i32, level: i32, value: f32) -> bool {
                raw::blz_remove_ability_real_level_array_field(which_ability.to_handle(), which_field, level, value)
            };
        },
        {
            link: "BlzRemoveAbilityStringLevelArrayField"
            name: blz_remove_ability_string_level_array_field
            raw: (which_ability: Handle<j_ability>, which_field: i32, level: i32, value: *const i8) -> bool;
            user: (which_ability: &Agent<j_ability>, which_field: i32, level: i32, value: &CStr) -> bool {
                raw::blz_remove_ability_string_level_array_field(which_ability.to_handle(), which_field, level, value.as_ptr())
            };
        },
        {
            link: "BlzGetItemAbilityByIndex"
            name: blz_get_item_ability_by_index
            raw: (which_item: Handle<j_item>, index: i32) -> Handle<j_ability>;
            user: (which_item: &Agent<j_item>, index: i32) -> Option<Weak<j_ability>> {
                raw::blz_get_item_ability_by_index(which_item.to_handle(), index).to_weak()
            };
        },
        {
            link: "BlzGetItemAbility"
            name: blz_get_item_ability
            raw: (which_item: Handle<j_item>, abil_code: i32) -> Handle<j_ability>;
            user: (which_item: &Agent<j_item>, abil_code: i32) -> Option<Weak<j_ability>> {
                raw::blz_get_item_ability(which_item.to_handle(), abil_code).to_weak()
            };
        },
        {
            link: "BlzItemAddAbility"
            name: blz_item_add_ability
            raw: (which_item: Handle<j_item>, abil_code: i32) -> bool;
            user: (which_item: &Agent<j_item>, abil_code: i32) -> bool {
                raw::blz_item_add_ability(which_item.to_handle(), abil_code)
            };
        },
        {
            link: "BlzGetItemBooleanField"
            name: blz_get_item_boolean_field
            raw: (which_item: Handle<j_item>, which_field: i32) -> bool;
            user: (which_item: &Agent<j_item>, which_field: i32) -> bool {
                raw::blz_get_item_boolean_field(which_item.to_handle(), which_field)
            };
        },
        {
            link: "BlzGetItemIntegerField"
            name: blz_get_item_integer_field
            raw: (which_item: Handle<j_item>, which_field: i32) -> i32;
            user: (which_item: &Agent<j_item>, which_field: i32) -> i32 {
                raw::blz_get_item_integer_field(which_item.to_handle(), which_field)
            };
        },
        {
            link: "BlzGetItemRealField"
            name: blz_get_item_real_field
            raw: (which_item: Handle<j_item>, which_field: i32) -> f32;
            user: (which_item: &Agent<j_item>, which_field: i32) -> f32 {
                raw::blz_get_item_real_field(which_item.to_handle(), which_field)
            };
        },
        {
            link: "BlzGetItemStringField"
            name: blz_get_item_string_field
            raw: (which_item: Handle<j_item>, which_field: i32, out: *mut i8, out_len: *mut usize);
            user: (which_item: &Agent<j_item>, which_field: i32) -> String {
                raw::blz_get_item_string_field(which_item.to_handle(), which_field, stringret::ptr(), stringret::reset_len()); stringret::read_string()
            };
        },
        {
            link: "BlzSetItemBooleanField"
            name: blz_set_item_boolean_field
            raw: (which_item: Handle<j_item>, which_field: i32, value: bool) -> bool;
            user: (which_item: &Agent<j_item>, which_field: i32, value: bool) -> bool {
                raw::blz_set_item_boolean_field(which_item.to_handle(), which_field, value)
            };
        },
        {
            link: "BlzSetItemIntegerField"
            name: blz_set_item_integer_field
            raw: (which_item: Handle<j_item>, which_field: i32, value: i32) -> bool;
            user: (which_item: &Agent<j_item>, which_field: i32, value: i32) -> bool {
                raw::blz_set_item_integer_field(which_item.to_handle(), which_field, value)
            };
        },
        {
            link: "BlzSetItemRealField"
            name: blz_set_item_real_field
            raw: (which_item: Handle<j_item>, which_field: i32, value: f32) -> bool;
            user: (which_item: &Agent<j_item>, which_field: i32, value: f32) -> bool {
                raw::blz_set_item_real_field(which_item.to_handle(), which_field, value)
            };
        },
        {
            link: "BlzSetItemStringField"
            name: blz_set_item_string_field
            raw: (which_item: Handle<j_item>, which_field: i32, value: *const i8) -> bool;
            user: (which_item: &Agent<j_item>, which_field: i32, value: &CStr) -> bool {
                raw::blz_set_item_string_field(which_item.to_handle(), which_field, value.as_ptr())
            };
        },
        {
            link: "BlzItemRemoveAbility"
            name: blz_item_remove_ability
            raw: (which_item: Handle<j_item>, abil_code: i32) -> bool;
            user: (which_item: &Agent<j_item>, abil_code: i32) -> bool {
                raw::blz_item_remove_ability(which_item.to_handle(), abil_code)
            };
        },
        {
            link: "BlzGetUnitBooleanField"
            name: blz_get_unit_boolean_field
            raw: (which_unit: Handle<j_unit>, which_field: i32) -> bool;
            user: (which_unit: &Agent<j_unit>, which_field: i32) -> bool {
                raw::blz_get_unit_boolean_field(which_unit.to_handle(), which_field)
            };
        },
        {
            link: "BlzGetUnitIntegerField"
            name: blz_get_unit_integer_field
            raw: (which_unit: Handle<j_unit>, which_field: i32) -> i32;
            user: (which_unit: &Agent<j_unit>, which_field: i32) -> i32 {
                raw::blz_get_unit_integer_field(which_unit.to_handle(), which_field)
            };
        },
        {
            link: "BlzGetUnitRealField"
            name: blz_get_unit_real_field
            raw: (which_unit: Handle<j_unit>, which_field: i32) -> f32;
            user: (which_unit: &Agent<j_unit>, which_field: i32) -> f32 {
                raw::blz_get_unit_real_field(which_unit.to_handle(), which_field)
            };
        },
        {
            link: "BlzGetUnitStringField"
            name: blz_get_unit_string_field
            raw: (which_unit: Handle<j_unit>, which_field: i32, out: *mut i8, out_len: *mut usize);
            user: (which_unit: &Agent<j_unit>, which_field: i32) -> String {
                raw::blz_get_unit_string_field(which_unit.to_handle(), which_field, stringret::ptr(), stringret::reset_len()); stringret::read_string()
            };
        },
        {
            link: "BlzSetUnitBooleanField"
            name: blz_set_unit_boolean_field
            raw: (which_unit: Handle<j_unit>, which_field: i32, value: bool) -> bool;
            user: (which_unit: &Agent<j_unit>, which_field: i32, value: bool) -> bool {
                raw::blz_set_unit_boolean_field(which_unit.to_handle(), which_field, value)
            };
        },
        {
            link: "BlzSetUnitIntegerField"
            name: blz_set_unit_integer_field
            raw: (which_unit: Handle<j_unit>, which_field: i32, value: i32) -> bool;
            user: (which_unit: &Agent<j_unit>, which_field: i32, value: i32) -> bool {
                raw::blz_set_unit_integer_field(which_unit.to_handle(), which_field, value)
            };
        },
        {
            link: "BlzSetUnitRealField"
            name: blz_set_unit_real_field
            raw: (which_unit: Handle<j_unit>, which_field: i32, value: f32) -> bool;
            user: (which_unit: &Agent<j_unit>, which_field: i32, value: f32) -> bool {
                raw::blz_set_unit_real_field(which_unit.to_handle(), which_field, value)
            };
        },
        {
            link: "BlzSetUnitStringField"
            name: blz_set_unit_string_field
            raw: (which_unit: Handle<j_unit>, which_field: i32, value: *const i8) -> bool;
            user: (which_unit: &Agent<j_unit>, which_field: i32, value: &CStr) -> bool {
                raw::blz_set_unit_string_field(which_unit.to_handle(), which_field, value.as_ptr())
            };
        },
        {
            link: "BlzGetUnitWeaponBooleanField"
            name: blz_get_unit_weapon_boolean_field
            raw: (which_unit: Handle<j_unit>, which_field: i32, index: i32) -> bool;
            user: (which_unit: &Agent<j_unit>, which_field: i32, index: i32) -> bool {
                raw::blz_get_unit_weapon_boolean_field(which_unit.to_handle(), which_field, index)
            };
        },
        {
            link: "BlzGetUnitWeaponIntegerField"
            name: blz_get_unit_weapon_integer_field
            raw: (which_unit: Handle<j_unit>, which_field: i32, index: i32) -> i32;
            user: (which_unit: &Agent<j_unit>, which_field: i32, index: i32) -> i32 {
                raw::blz_get_unit_weapon_integer_field(which_unit.to_handle(), which_field, index)
            };
        },
        {
            link: "BlzGetUnitWeaponRealField"
            name: blz_get_unit_weapon_real_field
            raw: (which_unit: Handle<j_unit>, which_field: i32, index: i32) -> f32;
            user: (which_unit: &Agent<j_unit>, which_field: i32, index: i32) -> f32 {
                raw::blz_get_unit_weapon_real_field(which_unit.to_handle(), which_field, index)
            };
        },
        {
            link: "BlzGetUnitWeaponStringField"
            name: blz_get_unit_weapon_string_field
            raw: (which_unit: Handle<j_unit>, which_field: i32, index: i32, out: *mut i8, out_len: *mut usize);
            user: (which_unit: &Agent<j_unit>, which_field: i32, index: i32) -> String {
                raw::blz_get_unit_weapon_string_field(which_unit.to_handle(), which_field, index, stringret::ptr(), stringret::reset_len()); stringret::read_string()
            };
        },
        {
            link: "BlzSetUnitWeaponBooleanField"
            name: blz_set_unit_weapon_boolean_field
            raw: (which_unit: Handle<j_unit>, which_field: i32, index: i32, value: bool) -> bool;
            user: (which_unit: &Agent<j_unit>, which_field: i32, index: i32, value: bool) -> bool {
                raw::blz_set_unit_weapon_boolean_field(which_unit.to_handle(), which_field, index, value)
            };
        },
        {
            link: "BlzSetUnitWeaponIntegerField"
            name: blz_set_unit_weapon_integer_field
            raw: (which_unit: Handle<j_unit>, which_field: i32, index: i32, value: i32) -> bool;
            user: (which_unit: &Agent<j_unit>, which_field: i32, index: i32, value: i32) -> bool {
                raw::blz_set_unit_weapon_integer_field(which_unit.to_handle(), which_field, index, value)
            };
        },
        {
            link: "BlzSetUnitWeaponRealField"
            name: blz_set_unit_weapon_real_field
            raw: (which_unit: Handle<j_unit>, which_field: i32, index: i32, value: f32) -> bool;
            user: (which_unit: &Agent<j_unit>, which_field: i32, index: i32, value: f32) -> bool {
                raw::blz_set_unit_weapon_real_field(which_unit.to_handle(), which_field, index, value)
            };
        },
        {
            link: "BlzSetUnitWeaponStringField"
            name: blz_set_unit_weapon_string_field
            raw: (which_unit: Handle<j_unit>, which_field: i32, index: i32, value: *const i8) -> bool;
            user: (which_unit: &Agent<j_unit>, which_field: i32, index: i32, value: &CStr) -> bool {
                raw::blz_set_unit_weapon_string_field(which_unit.to_handle(), which_field, index, value.as_ptr())
            };
        },
        {
            link: "BlzGetUnitSkin"
            name: blz_get_unit_skin
            raw: (which_unit: Handle<j_unit>) -> i32;
            user: (which_unit: &Agent<j_unit>) -> i32 {
                raw::blz_get_unit_skin(which_unit.to_handle())
            };
        },
        {
            link: "BlzGetItemSkin"
            name: blz_get_item_skin
            raw: (which_item: Handle<j_item>) -> i32;
            user: (which_item: &Agent<j_item>) -> i32 {
                raw::blz_get_item_skin(which_item.to_handle())
            };
        },
        {
            link: "BlzSetUnitSkin"
            name: blz_set_unit_skin
            raw: (which_unit: Handle<j_unit>, skin_id: i32);
            user: (which_unit: &Agent<j_unit>, skin_id: i32) {
                raw::blz_set_unit_skin(which_unit.to_handle(), skin_id)
            };
        },
        {
            link: "BlzSetItemSkin"
            name: blz_set_item_skin
            raw: (which_item: Handle<j_item>, skin_id: i32);
            user: (which_item: &Agent<j_item>, skin_id: i32) {
                raw::blz_set_item_skin(which_item.to_handle(), skin_id)
            };
        },
        {
            link: "BlzCreateItemWithSkin"
            name: blz_create_item_with_skin
            raw: (itemid: i32, x: f32, y: f32, skin_id: i32) -> Handle<j_item>;
            user: (itemid: i32, x: f32, y: f32, skin_id: i32) -> Option<Weak<j_item>> {
                raw::blz_create_item_with_skin(itemid, x, y, skin_id).to_weak()
            };
        },
        {
            link: "BlzCreateUnitWithSkin"
            name: blz_create_unit_with_skin
            raw: (id: Handle<j_player>, unitid: i32, x: f32, y: f32, face: f32, skin_id: i32) -> Handle<j_unit>;
            user: (id: &Agent<j_player>, unitid: i32, x: f32, y: f32, face: f32, skin_id: i32) -> Option<Weak<j_unit>> {
                raw::blz_create_unit_with_skin(id.to_handle(), unitid, x, y, face, skin_id).to_weak()
            };
        },
        {
            link: "BlzCreateDestructableWithSkin"
            name: blz_create_destructable_with_skin
            raw: (objectid: i32, x: f32, y: f32, face: f32, scale: f32, variation: i32, skin_id: i32) -> Handle<j_destructable>;
            user: (objectid: i32, x: f32, y: f32, face: f32, scale: f32, variation: i32, skin_id: i32) -> Option<Weak<j_destructable>> {
                raw::blz_create_destructable_with_skin(objectid, x, y, face, scale, variation, skin_id).to_weak()
            };
        },
        {
            link: "BlzCreateDestructableZWithSkin"
            name: blz_create_destructable_z_with_skin
            raw: (objectid: i32, x: f32, y: f32, z: f32, face: f32, scale: f32, variation: i32, skin_id: i32) -> Handle<j_destructable>;
            user: (objectid: i32, x: f32, y: f32, z: f32, face: f32, scale: f32, variation: i32, skin_id: i32) -> Option<Weak<j_destructable>> {
                raw::blz_create_destructable_z_with_skin(objectid, x, y, z, face, scale, variation, skin_id).to_weak()
            };
        },
        {
            link: "BlzCreateDeadDestructableWithSkin"
            name: blz_create_dead_destructable_with_skin
            raw: (objectid: i32, x: f32, y: f32, face: f32, scale: f32, variation: i32, skin_id: i32) -> Handle<j_destructable>;
            user: (objectid: i32, x: f32, y: f32, face: f32, scale: f32, variation: i32, skin_id: i32) -> Option<Weak<j_destructable>> {
                raw::blz_create_dead_destructable_with_skin(objectid, x, y, face, scale, variation, skin_id).to_weak()
            };
        },
        {
            link: "BlzCreateDeadDestructableZWithSkin"
            name: blz_create_dead_destructable_z_with_skin
            raw: (objectid: i32, x: f32, y: f32, z: f32, face: f32, scale: f32, variation: i32, skin_id: i32) -> Handle<j_destructable>;
            user: (objectid: i32, x: f32, y: f32, z: f32, face: f32, scale: f32, variation: i32, skin_id: i32) -> Option<Weak<j_destructable>> {
                raw::blz_create_dead_destructable_z_with_skin(objectid, x, y, z, face, scale, variation, skin_id).to_weak()
            };
        },
        {
            link: "BlzGetPlayerTownHallCount"
            name: blz_get_player_town_hall_count
            raw: (which_player: Handle<j_player>) -> i32;
            user: (which_player: &Agent<j_player>) -> i32 {
                raw::blz_get_player_town_hall_count(which_player.to_handle())
            };
        },
        {
            link: "BlzQueueImmediateOrderById"
            name: blz_queue_immediate_order_by_id
            raw: (which_unit: Handle<j_unit>, order: i32) -> bool;
            user: (which_unit: &Agent<j_unit>, order: i32) -> bool {
                raw::blz_queue_immediate_order_by_id(which_unit.to_handle(), order)
            };
        },
        {
            link: "BlzQueuePointOrderById"
            name: blz_queue_point_order_by_id
            raw: (which_unit: Handle<j_unit>, order: i32, x: f32, y: f32) -> bool;
            user: (which_unit: &Agent<j_unit>, order: i32, x: f32, y: f32) -> bool {
                raw::blz_queue_point_order_by_id(which_unit.to_handle(), order, x, y)
            };
        },
        {
            link: "BlzQueueTargetOrderById"
            name: blz_queue_target_order_by_id
            raw: (which_unit: Handle<j_unit>, order: i32, target_widget: Handle<j_widget>) -> bool;
            user: (which_unit: &Agent<j_unit>, order: i32, target_widget: &Agent<j_widget>) -> bool {
                raw::blz_queue_target_order_by_id(which_unit.to_handle(), order, target_widget.to_handle())
            };
        },
        {
            link: "BlzQueueInstantPointOrderById"
            name: blz_queue_instant_point_order_by_id
            raw: (which_unit: Handle<j_unit>, order: i32, x: f32, y: f32, instant_target_widget: Handle<j_widget>) -> bool;
            user: (which_unit: &Agent<j_unit>, order: i32, x: f32, y: f32, instant_target_widget: &Agent<j_widget>) -> bool {
                raw::blz_queue_instant_point_order_by_id(which_unit.to_handle(), order, x, y, instant_target_widget.to_handle())
            };
        },
        {
            link: "BlzQueueInstantTargetOrderById"
            name: blz_queue_instant_target_order_by_id
            raw: (which_unit: Handle<j_unit>, order: i32, target_widget: Handle<j_widget>, instant_target_widget: Handle<j_widget>) -> bool;
            user: (which_unit: &Agent<j_unit>, order: i32, target_widget: &Agent<j_widget>, instant_target_widget: &Agent<j_widget>) -> bool {
                raw::blz_queue_instant_target_order_by_id(which_unit.to_handle(), order, target_widget.to_handle(), instant_target_widget.to_handle())
            };
        },
        {
            link: "BlzQueueBuildOrderById"
            name: blz_queue_build_order_by_id
            raw: (which_peon: Handle<j_unit>, unit_id: i32, x: f32, y: f32) -> bool;
            user: (which_peon: &Agent<j_unit>, unit_id: i32, x: f32, y: f32) -> bool {
                raw::blz_queue_build_order_by_id(which_peon.to_handle(), unit_id, x, y)
            };
        },
        {
            link: "BlzQueueNeutralImmediateOrderById"
            name: blz_queue_neutral_immediate_order_by_id
            raw: (for_which_player: Handle<j_player>, neutral_structure: Handle<j_unit>, unit_id: i32) -> bool;
            user: (for_which_player: &Agent<j_player>, neutral_structure: &Agent<j_unit>, unit_id: i32) -> bool {
                raw::blz_queue_neutral_immediate_order_by_id(for_which_player.to_handle(), neutral_structure.to_handle(), unit_id)
            };
        },
        {
            link: "BlzQueueNeutralPointOrderById"
            name: blz_queue_neutral_point_order_by_id
            raw: (for_which_player: Handle<j_player>, neutral_structure: Handle<j_unit>, unit_id: i32, x: f32, y: f32) -> bool;
            user: (for_which_player: &Agent<j_player>, neutral_structure: &Agent<j_unit>, unit_id: i32, x: f32, y: f32) -> bool {
                raw::blz_queue_neutral_point_order_by_id(for_which_player.to_handle(), neutral_structure.to_handle(), unit_id, x, y)
            };
        },
        {
            link: "BlzQueueNeutralTargetOrderById"
            name: blz_queue_neutral_target_order_by_id
            raw: (for_which_player: Handle<j_player>, neutral_structure: Handle<j_unit>, unit_id: i32, target: Handle<j_widget>) -> bool;
            user: (for_which_player: &Agent<j_player>, neutral_structure: &Agent<j_unit>, unit_id: i32, target: &Agent<j_widget>) -> bool {
                raw::blz_queue_neutral_target_order_by_id(for_which_player.to_handle(), neutral_structure.to_handle(), unit_id, target.to_handle())
            };
        },
        {
            link: "BlzGetUnitOrderCount"
            name: blz_get_unit_order_count
            raw: (which_unit: Handle<j_unit>) -> i32;
            user: (which_unit: &Agent<j_unit>) -> i32 {
                raw::blz_get_unit_order_count(which_unit.to_handle())
            };
        },
        {
            link: "BlzUnitClearOrders"
            name: blz_unit_clear_orders
            raw: (which_unit: Handle<j_unit>, only_queued: bool);
            user: (which_unit: &Agent<j_unit>, only_queued: bool) {
                raw::blz_unit_clear_orders(which_unit.to_handle(), only_queued)
            };
        },
        {
            link: "BlzUnitForceStopOrder"
            name: blz_unit_force_stop_order
            raw: (which_unit: Handle<j_unit>, clear_queue: bool);
            user: (which_unit: &Agent<j_unit>, clear_queue: bool) {
                raw::blz_unit_force_stop_order(which_unit.to_handle(), clear_queue)
            };
        },
        {
            link: "CeGetEventAttackFlags"
            name: ce_get_event_attack_flags
            raw: () -> Handle<j_attackflags>;
            user: () -> Handle<j_attackflags> {
                raw::ce_get_event_attack_flags()
            };
        },
        {
            link: "CeGetEventDamageTypes"
            name: ce_get_event_damage_types
            raw: () -> Handle<j_damagetypes>;
            user: () -> Handle<j_damagetypes> {
                raw::ce_get_event_damage_types()
            };
        },
        {
            link: "CeSetEventDamageTypes"
            name: ce_set_event_damage_types
            raw: (damage_types: Handle<j_damagetypes>) -> bool;
            user: (damage_types: Handle<j_damagetypes>) -> bool {
                raw::ce_set_event_damage_types(damage_types)
            };
        },
        {
            link: "CeGetEventIsRanged"
            name: ce_get_event_is_ranged
            raw: () -> bool;
            user: () -> bool {
                raw::ce_get_event_is_ranged()
            };
        },
        {
            link: "CeGetMouseHoveredFrame"
            name: ce_get_mouse_hovered_frame
            raw: () -> Handle<j_framehandle>;
            user: () -> Handle<j_framehandle> {
                raw::ce_get_mouse_hovered_frame()
            };
        },
        {
            link: "CeGetFrameScale"
            name: ce_get_frame_scale
            raw: (which_frame: Handle<j_framehandle>) -> f32;
            user: (which_frame: Handle<j_framehandle>) -> f32 {
                raw::ce_get_frame_scale(which_frame)
            };
        },
        {
            link: "CeGetFrameModel"
            name: ce_get_frame_model
            raw: (which_frame: Handle<j_framehandle>) -> Handle<j_modelhandle>;
            user: (which_frame: Handle<j_framehandle>) -> Handle<j_modelhandle> {
                raw::ce_get_frame_model(which_frame)
            };
        },
        {
            link: "CeSetWar3ImageModelPath"
            name: ce_set_war3_image_model_path
            raw: (which_image: Handle<j_war3image>, model_path: *const i8);
            user: (which_image: &Agent<j_war3image>, model_path: &CStr) {
                raw::ce_set_war3_image_model_path(which_image.to_handle(), model_path.as_ptr())
            };
        },
        {
            link: "CeSetWar3ImageTexturePath"
            name: ce_set_war3_image_texture_path
            raw: (which_image: Handle<j_war3image>, texture_path: *const i8, texture_id: i32);
            user: (which_image: &Agent<j_war3image>, texture_path: &CStr, texture_id: i32) {
                raw::ce_set_war3_image_texture_path(which_image.to_handle(), texture_path.as_ptr(), texture_id)
            };
        },
        {
            link: "CeGetWar3ImageModel"
            name: ce_get_war3_image_model
            raw: (which_image: Handle<j_war3image>) -> Handle<j_modelhandle>;
            user: (which_image: &Agent<j_war3image>) -> Handle<j_modelhandle> {
                raw::ce_get_war3_image_model(which_image.to_handle())
            };
        },
        {
            link: "CeSetModelPitch"
            name: ce_set_model_pitch
            raw: (which_model: Handle<j_modelhandle>, pitch: f32);
            user: (which_model: Handle<j_modelhandle>, pitch: f32) {
                raw::ce_set_model_pitch(which_model, pitch)
            };
        },
        {
            link: "CeSetModelRoll"
            name: ce_set_model_roll
            raw: (which_model: Handle<j_modelhandle>, roll: f32);
            user: (which_model: Handle<j_modelhandle>, roll: f32) {
                raw::ce_set_model_roll(which_model, roll)
            };
        },
        {
            link: "CeSetModelYaw"
            name: ce_set_model_yaw
            raw: (which_model: Handle<j_modelhandle>, yaw: f32);
            user: (which_model: Handle<j_modelhandle>, yaw: f32) {
                raw::ce_set_model_yaw(which_model, yaw)
            };
        },
        {
            link: "CeSetModelOrientation"
            name: ce_set_model_orientation
            raw: (which_model: Handle<j_modelhandle>, yaw: f32, pitch: f32, roll: f32);
            user: (which_model: Handle<j_modelhandle>, yaw: f32, pitch: f32, roll: f32) {
                raw::ce_set_model_orientation(which_model, yaw, pitch, roll)
            };
        },
        {
            link: "CeSetModelScale"
            name: ce_set_model_scale
            raw: (which_model: Handle<j_modelhandle>, scale: f32);
            user: (which_model: Handle<j_modelhandle>, scale: f32) {
                raw::ce_set_model_scale(which_model, scale)
            };
        },
        {
            link: "CeSetModelTime"
            name: ce_set_model_time
            raw: (which_model: Handle<j_modelhandle>, time: f32);
            user: (which_model: Handle<j_modelhandle>, time: f32) {
                raw::ce_set_model_time(which_model, time)
            };
        },
        {
            link: "CeSetModelTimeScale"
            name: ce_set_model_time_scale
            raw: (which_model: Handle<j_modelhandle>, time_scale: f32);
            user: (which_model: Handle<j_modelhandle>, time_scale: f32) {
                raw::ce_set_model_time_scale(which_model, time_scale)
            };
        },
        {
            link: "CeSetModelHeight"
            name: ce_set_model_height
            raw: (which_model: Handle<j_modelhandle>, height: f32);
            user: (which_model: Handle<j_modelhandle>, height: f32) {
                raw::ce_set_model_height(which_model, height)
            };
        },
        {
            link: "CeSetModelX"
            name: ce_set_model_x
            raw: (which_model: Handle<j_modelhandle>, x: f32);
            user: (which_model: Handle<j_modelhandle>, x: f32) {
                raw::ce_set_model_x(which_model, x)
            };
        },
        {
            link: "CeSetModelY"
            name: ce_set_model_y
            raw: (which_model: Handle<j_modelhandle>, y: f32);
            user: (which_model: Handle<j_modelhandle>, y: f32) {
                raw::ce_set_model_y(which_model, y)
            };
        },
        {
            link: "CeSetModelZ"
            name: ce_set_model_z
            raw: (which_model: Handle<j_modelhandle>, z: f32);
            user: (which_model: Handle<j_modelhandle>, z: f32) {
                raw::ce_set_model_z(which_model, z)
            };
        },
        {
            link: "CeSetModelPosition"
            name: ce_set_model_position
            raw: (which_model: Handle<j_modelhandle>, x: f32, y: f32, z: f32);
            user: (which_model: Handle<j_modelhandle>, x: f32, y: f32, z: f32) {
                raw::ce_set_model_position(which_model, x, y, z)
            };
        },
        {
            link: "CeSetModelAlpha"
            name: ce_set_model_alpha
            raw: (which_model: Handle<j_modelhandle>, alpha: i32);
            user: (which_model: Handle<j_modelhandle>, alpha: i32) {
                raw::ce_set_model_alpha(which_model, alpha)
            };
        },
        {
            link: "CeSetModelColor"
            name: ce_set_model_color
            raw: (which_model: Handle<j_modelhandle>, r: i32, g: i32, b: i32);
            user: (which_model: Handle<j_modelhandle>, r: i32, g: i32, b: i32) {
                raw::ce_set_model_color(which_model, r, g, b)
            };
        },
        {
            link: "CeSetModelColorByPlayer"
            name: ce_set_model_color_by_player
            raw: (which_model: Handle<j_modelhandle>, which_player: Handle<j_player>);
            user: (which_model: Handle<j_modelhandle>, which_player: &Agent<j_player>) {
                raw::ce_set_model_color_by_player(which_model, which_player.to_handle())
            };
        },
        {
            link: "CeSetModelTexturePath"
            name: ce_set_model_texture_path
            raw: (which_model: Handle<j_modelhandle>, texture_path: *const i8, texture_id: i32);
            user: (which_model: Handle<j_modelhandle>, texture_path: &CStr, texture_id: i32) {
                raw::ce_set_model_texture_path(which_model, texture_path.as_ptr(), texture_id)
            };
        },
        {
            link: "CeGetModelTexturePath"
            name: ce_get_model_texture_path
            raw: (which_model: Handle<j_modelhandle>, texture_id: i32, out: *mut i8, out_len: *mut usize);
            user: (which_model: Handle<j_modelhandle>, texture_id: i32) -> String {
                raw::ce_get_model_texture_path(which_model, texture_id, stringret::ptr(), stringret::reset_len()); stringret::read_string()
            };
        },
        {
            link: "CeGetModelTextureCount"
            name: ce_get_model_texture_count
            raw: (which_model: Handle<j_modelhandle>) -> i32;
            user: (which_model: Handle<j_modelhandle>) -> i32 {
                raw::ce_get_model_texture_count(which_model)
            };
        },
        {
            link: "CeSetModelAnimation"
            name: ce_set_model_animation
            raw: (which_model: Handle<j_modelhandle>, which_animation: *const i8);
            user: (which_model: Handle<j_modelhandle>, which_animation: &CStr) {
                raw::ce_set_model_animation(which_model, which_animation.as_ptr())
            };
        },
        {
            link: "CeSetModelAnimationEx"
            name: ce_set_model_animation_ex
            raw: (which_model: Handle<j_modelhandle>, primary_prop: i32, flags: i32);
            user: (which_model: Handle<j_modelhandle>, primary_prop: i32, flags: i32) {
                raw::ce_set_model_animation_ex(which_model, primary_prop, flags)
            };
        },
        {
            link: "CeSetModelAnimationByIndex"
            name: ce_set_model_animation_by_index
            raw: (which_model: Handle<j_modelhandle>, which_index: i32);
            user: (which_model: Handle<j_modelhandle>, which_index: i32) {
                raw::ce_set_model_animation_by_index(which_model, which_index)
            };
        },
        {
            link: "CeSetModelAnimationWithRarity"
            name: ce_set_model_animation_with_rarity
            raw: (which_model: Handle<j_modelhandle>, which_animation: *const i8, rarity: i32);
            user: (which_model: Handle<j_modelhandle>, which_animation: &CStr, rarity: i32) {
                raw::ce_set_model_animation_with_rarity(which_model, which_animation.as_ptr(), rarity)
            };
        },
        {
            link: "CeQueueModelAnimation"
            name: ce_queue_model_animation
            raw: (which_model: Handle<j_modelhandle>, which_animation: *const i8);
            user: (which_model: Handle<j_modelhandle>, which_animation: &CStr) {
                raw::ce_queue_model_animation(which_model, which_animation.as_ptr())
            };
        },
        {
            link: "CeAddModelAttachment"
            name: ce_add_model_attachment
            raw: (which_model: Handle<j_modelhandle>, model_path: *const i8, attach_point_name: *const i8) -> Handle<j_modelhandle>;
            user: (which_model: Handle<j_modelhandle>, model_path: &CStr, attach_point_name: &CStr) -> Handle<j_modelhandle> {
                raw::ce_add_model_attachment(which_model, model_path.as_ptr(), attach_point_name.as_ptr())
            };
        },
        {
            link: "CeSetModelAttachment"
            name: ce_set_model_attachment
            raw: (which_model: Handle<j_modelhandle>, attach_model: Handle<j_modelhandle>, attach_point_name: *const i8);
            user: (which_model: Handle<j_modelhandle>, attach_model: Handle<j_modelhandle>, attach_point_name: &CStr) {
                raw::ce_set_model_attachment(which_model, attach_model, attach_point_name.as_ptr())
            };
        },
        {
            link: "CeSetModelPath"
            name: ce_set_model_path
            raw: (which_sprite: Handle<j_modelhandle>, model_path: *const i8);
            user: (which_sprite: Handle<j_modelhandle>, model_path: &CStr) {
                raw::ce_set_model_path(which_sprite, model_path.as_ptr())
            };
        },
        {
            link: "CeGetModelHeight"
            name: ce_get_model_height
            raw: (which_model: Handle<j_modelhandle>) -> f32;
            user: (which_model: Handle<j_modelhandle>) -> f32 {
                raw::ce_get_model_height(which_model)
            };
        },
        {
            link: "CeGetModelX"
            name: ce_get_model_x
            raw: (which_model: Handle<j_modelhandle>) -> f32;
            user: (which_model: Handle<j_modelhandle>) -> f32 {
                raw::ce_get_model_x(which_model)
            };
        },
        {
            link: "CeGetModelY"
            name: ce_get_model_y
            raw: (which_model: Handle<j_modelhandle>) -> f32;
            user: (which_model: Handle<j_modelhandle>) -> f32 {
                raw::ce_get_model_y(which_model)
            };
        },
        {
            link: "CeGetModelZ"
            name: ce_get_model_z
            raw: (which_model: Handle<j_modelhandle>) -> f32;
            user: (which_model: Handle<j_modelhandle>) -> f32 {
                raw::ce_get_model_z(which_model)
            };
        },
        {
            link: "CeIsKeyDown"
            name: ce_is_key_down
            raw: (key: i32) -> bool;
            user: (key: i32) -> bool {
                raw::ce_is_key_down(key)
            };
        },
        {
            link: "CeIsMouseButtonDown"
            name: ce_is_mouse_button_down
            raw: (button: i32) -> bool;
            user: (button: i32) -> bool {
                raw::ce_is_mouse_button_down(button)
            };
        },
        {
            link: "CeGetMouseX"
            name: ce_get_mouse_x
            raw: () -> i32;
            user: () -> i32 {
                raw::ce_get_mouse_x()
            };
        },
        {
            link: "CeGetMouseY"
            name: ce_get_mouse_y
            raw: () -> i32;
            user: () -> i32 {
                raw::ce_get_mouse_y()
            };
        },
        {
            link: "CeGetMouseRelativeX"
            name: ce_get_mouse_relative_x
            raw: () -> i32;
            user: () -> i32 {
                raw::ce_get_mouse_relative_x()
            };
        },
        {
            link: "CeGetMouseRelativeY"
            name: ce_get_mouse_relative_y
            raw: () -> i32;
            user: () -> i32 {
                raw::ce_get_mouse_relative_y()
            };
        },
        {
            link: "CeGetMouseHoveredUnit"
            name: ce_get_mouse_hovered_unit
            raw: () -> Handle<j_unit>;
            user: () -> Option<Weak<j_unit>> {
                raw::ce_get_mouse_hovered_unit().to_weak()
            };
        },
        {
            link: "CeGetMouseTargetedUnit"
            name: ce_get_mouse_targeted_unit
            raw: () -> Handle<j_unit>;
            user: () -> Option<Weak<j_unit>> {
                raw::ce_get_mouse_targeted_unit().to_weak()
            };
        },
        {
            link: "CeGetMouseHoveredItem"
            name: ce_get_mouse_hovered_item
            raw: () -> Handle<j_item>;
            user: () -> Option<Weak<j_item>> {
                raw::ce_get_mouse_hovered_item().to_weak()
            };
        },
        {
            link: "CeGetMouseTargetedItem"
            name: ce_get_mouse_targeted_item
            raw: () -> Handle<j_item>;
            user: () -> Option<Weak<j_item>> {
                raw::ce_get_mouse_targeted_item().to_weak()
            };
        },
        {
            link: "CeGetMouseHoveredDestructable"
            name: ce_get_mouse_hovered_destructable
            raw: () -> Handle<j_destructable>;
            user: () -> Option<Weak<j_destructable>> {
                raw::ce_get_mouse_hovered_destructable().to_weak()
            };
        },
        {
            link: "CeGetMouseTargetedDestructable"
            name: ce_get_mouse_targeted_destructable
            raw: () -> Handle<j_destructable>;
            user: () -> Option<Weak<j_destructable>> {
                raw::ce_get_mouse_targeted_destructable().to_weak()
            };
        },
    }
}
#[rustfmt::skip]
pub(crate) mod handles {
    use super::*;
    crate::define_handles! {
        j_agent: (j_handle),
        j_event: (j_agent, j_handle),
        j_player: (j_agent, j_handle),
        j_war3image: (j_agent, j_handle),
        j_widget: (j_agent, j_handle, j_war3image),
        j_unit: (j_agent, j_handle, j_war3image, j_widget),
        j_destructable: (j_agent, j_handle, j_war3image, j_widget),
        j_item: (j_agent, j_handle, j_war3image, j_widget),
        j_ability: (j_agent, j_handle),
        j_buff: (j_ability, j_agent, j_handle),
        j_force: (j_agent, j_handle),
        j_group: (j_agent, j_handle),
        j_trigger: (j_agent, j_handle),
        j_triggercondition: (j_agent, j_handle),
        j_triggeraction: (j_handle),
        j_timer: (j_agent, j_handle),
        j_location: (j_agent, j_handle),
        j_region: (j_agent, j_handle),
        j_rect: (j_agent, j_handle),
        j_boolexpr: (j_agent, j_handle),
        j_sound: (j_agent, j_handle),
        j_conditionfunc: (j_agent, j_boolexpr, j_handle),
        j_filterfunc: (j_agent, j_boolexpr, j_handle),
        j_unitpool: (j_handle),
        j_itempool: (j_handle),
        j_race: (j_handle),
        j_alliancetype: (j_handle),
        j_racepreference: (j_handle),
        j_gamestate: (j_handle),
        j_igamestate: (j_gamestate, j_handle),
        j_fgamestate: (j_gamestate, j_handle),
        j_playerstate: (j_handle),
        j_playerscore: (j_handle),
        j_playergameresult: (j_handle),
        j_unitstate: (j_handle),
        j_aidifficulty: (j_handle),
        j_eventid: (j_handle),
        j_gameevent: (j_eventid, j_handle),
        j_playerevent: (j_eventid, j_handle),
        j_playerunitevent: (j_eventid, j_handle),
        j_unitevent: (j_eventid, j_handle),
        j_limitop: (j_eventid, j_handle),
        j_widgetevent: (j_eventid, j_handle),
        j_dialogevent: (j_eventid, j_handle),
        j_unittype: (j_handle),
        j_gamespeed: (j_handle),
        j_gamedifficulty: (j_handle),
        j_gametype: (j_handle),
        j_mapflag: (j_handle),
        j_mapvisibility: (j_handle),
        j_mapsetting: (j_handle),
        j_mapdensity: (j_handle),
        j_mapcontrol: (j_handle),
        j_minimapicon: (j_handle),
        j_playerslotstate: (j_handle),
        j_volumegroup: (j_handle),
        j_camerafield: (j_handle),
        j_camerasetup: (j_handle),
        j_playercolor: (j_handle),
        j_placement: (j_handle),
        j_startlocprio: (j_handle),
        j_raritycontrol: (j_handle),
        j_blendmode: (j_handle),
        j_texmapflags: (j_handle),
        j_effect: (j_agent, j_handle, j_war3image),
        j_effecttype: (j_handle),
        j_weathereffect: (j_handle),
        j_terraindeformation: (j_handle),
        j_fogstate: (j_handle),
        j_fogmodifier: (j_agent, j_handle),
        j_dialog: (j_agent, j_handle),
        j_button: (j_agent, j_handle),
        j_quest: (j_agent, j_handle),
        j_questitem: (j_agent, j_handle),
        j_defeatcondition: (j_agent, j_handle),
        j_timerdialog: (j_agent, j_handle),
        j_leaderboard: (j_agent, j_handle),
        j_multiboard: (j_agent, j_handle),
        j_multiboarditem: (j_agent, j_handle),
        j_trackable: (j_agent, j_handle, j_war3image),
        j_gamecache: (j_agent, j_handle),
        j_version: (j_handle),
        j_itemtype: (j_handle),
        j_texttag: (j_handle),
        j_attacktype: (j_handle),
        j_attackflags: (j_handle),
        j_damagetype: (j_handle),
        j_damagetypes: (j_handle),
        j_weapontype: (j_handle),
        j_soundtype: (j_handle),
        j_lightning: (j_handle),
        j_pathingtype: (j_handle),
        j_mousebuttontype: (j_handle),
        j_animtype: (j_handle),
        j_subanimtype: (j_handle),
        j_image: (j_handle),
        j_ubersplat: (j_handle),
        j_hashtable: (j_agent, j_handle),
        j_framehandle: (j_handle),
        j_originframetype: (j_handle),
        j_framepointtype: (j_handle),
        j_textaligntype: (j_handle),
        j_frameeventtype: (j_handle),
        j_oskeytype: (j_handle),
        j_abilityintegerfield: (j_handle),
        j_abilityrealfield: (j_handle),
        j_abilitybooleanfield: (j_handle),
        j_abilitystringfield: (j_handle),
        j_abilityintegerlevelfield: (j_handle),
        j_abilityreallevelfield: (j_handle),
        j_abilitybooleanlevelfield: (j_handle),
        j_abilitystringlevelfield: (j_handle),
        j_abilityintegerlevelarrayfield: (j_handle),
        j_abilityreallevelarrayfield: (j_handle),
        j_abilitybooleanlevelarrayfield: (j_handle),
        j_abilitystringlevelarrayfield: (j_handle),
        j_unitintegerfield: (j_handle),
        j_unitrealfield: (j_handle),
        j_unitbooleanfield: (j_handle),
        j_unitstringfield: (j_handle),
        j_unitweaponintegerfield: (j_handle),
        j_unitweaponrealfield: (j_handle),
        j_unitweaponbooleanfield: (j_handle),
        j_unitweaponstringfield: (j_handle),
        j_itemintegerfield: (j_handle),
        j_itemrealfield: (j_handle),
        j_itembooleanfield: (j_handle),
        j_itemstringfield: (j_handle),
        j_movetype: (j_handle),
        j_targetflag: (j_handle),
        j_armortype: (j_handle),
        j_heroattribute: (j_handle),
        j_defensetype: (j_handle),
        j_regentype: (j_handle),
        j_unitcategory: (j_handle),
        j_pathingflag: (j_handle),
        j_commandbuttoneffect: (j_handle),
        j_modelhandle: (j_handle),
    }
}

