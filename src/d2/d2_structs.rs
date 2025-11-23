#![allow(dead_code)]

use std::os::raw::{c_char, c_int, c_void};

pub type DWORD = u32;
pub type WORD = u16;
pub type BYTE = u8;
pub type BOOL = i32;

#[repr(C, packed(1))]
pub struct D2ClientStruct {
    pub dw_init: DWORD,
    pub _1: [BYTE; 0x20D - 4],
    pub fp_init: DWORD,
}

#[repr(C)]
pub struct InventoryInfo {
    pub n_location: c_int,
    pub n_max_x_cells: c_int,
    pub n_max_y_cells: c_int,
}

#[repr(C)]
pub struct CollMap {
    pub dw_pos_game_x: DWORD,
    pub dw_pos_game_y: DWORD,
    pub dw_size_game_x: DWORD,
    pub dw_size_game_y: DWORD,
    pub dw_pos_room_x: DWORD,
    pub dw_pos_room_y: DWORD,
    pub dw_size_room_x: DWORD,
    pub dw_size_room_y: DWORD,
    pub p_map_start: *mut WORD,
    pub p_map_end: *mut WORD,
}

#[repr(C)]
pub struct LevelTxt {
    pub dw_level_no: DWORD,
    pub _1: [DWORD; 60],
    pub _2: BYTE,
    pub sz_name: [c_char; 40],
    pub sz_entrance_text: [c_char; 40],
    pub sz_level_desc: [c_char; 41],
    pub w_name: [u16; 40],
    pub w_entrance_text: [u16; 40],
    pub n_obj_group: [BYTE; 8],
    pub n_obj_prb: [BYTE; 8],
}

#[repr(C, packed(1))]
pub struct RoomTile {
    pub p_room2: *mut Room2,
    pub p_next: *mut RoomTile,
    pub _2: [DWORD; 2],
    pub n_num: *mut DWORD,
}

#[repr(C, packed(1))]
pub struct QuestInfo {
    pub p_buffer: *mut c_void,
    pub _1: DWORD,
}

#[repr(C, packed(1))]
pub struct Waypoint {
    pub flags: BYTE,
}

#[repr(C, packed(1))]
pub struct PlayerData {
    pub sz_name: [c_char; 0x10],
    pub p_normal_quest: *mut QuestInfo,
    pub p_nightmare_quest: *mut QuestInfo,
    pub p_hell_quest: *mut QuestInfo,
    pub p_normal_waypoint: *mut Waypoint,
    pub p_nightmare_waypoint: *mut Waypoint,
    pub p_hell_waypoint: *mut Waypoint,
}

#[repr(C, packed(1))]
pub struct PresetUnit {
    pub _1: DWORD,
    pub dw_txt_file_no: DWORD,
    pub dw_pos_x: DWORD,
    pub p_preset_next: *mut PresetUnit,
    pub _3: DWORD,
    pub dw_type: DWORD,
    pub dw_pos_y: DWORD,
}

#[repr(C)]
pub struct Level {
    pub _1: [DWORD; 4],
    pub p_room2_first: *mut Room2,
    pub _2: [DWORD; 2],
    pub dw_pos_x: DWORD,
    pub dw_pos_y: DWORD,
    pub dw_size_x: DWORD,
    pub dw_size_y: DWORD,
    pub _3: [DWORD; 96],
    pub p_next_level: *mut Level,
    pub _4: DWORD,
    pub p_misc: *mut ActMisc,
    pub _5: [DWORD; 6],
    pub dw_level_no: DWORD,
}

#[repr(C, packed(1))]
pub struct Room2 {
    pub _1: [DWORD; 2],
    pub p_room2_near: *mut *mut Room2,
    pub _2: [DWORD; 6],
    pub p_room2_next: *mut Room2,
    pub dw_room_flags: DWORD,
    pub dw_rooms_near: DWORD,
    pub p_room1: *mut Room1,
    pub dw_pos_x: DWORD,
    pub dw_pos_y: DWORD,
    pub dw_size_x: DWORD,
    pub dw_size_y: DWORD,
    pub _3: DWORD,
    pub dw_preset_type: DWORD,
    pub p_room_tiles: *mut RoomTile,
    pub _4: [DWORD; 2],
    pub p_level: *mut Level,
    pub p_preset: *mut PresetUnit,
}

#[repr(C)]
pub struct Room1 {
    pub p_rooms_near: *mut *mut Room1,
    pub _1: [DWORD; 3],
    pub p_room2: *mut Room2,
    pub _2: [DWORD; 3],
    pub coll: *mut CollMap,
    pub dw_rooms_near: DWORD,
    pub _3: [DWORD; 9],
    pub dw_pos_x: DWORD,
    pub dw_pos_y: DWORD,
    pub dw_size_x: DWORD,
    pub dw_size_y: DWORD,
    pub _4: [DWORD; 6],
    pub p_unit_first: *mut UnitAny,
    pub _5: DWORD,
    pub p_room_next: *mut Room1,
}

#[repr(C)]
pub struct ActMisc {
    pub _1: [DWORD; 37],
    pub dw_staff_tomb_level: DWORD,
    pub _2: [DWORD; 245],
    pub p_act: *mut Act,
    pub _3: [DWORD; 3],
    pub p_level_first: *mut Level,
}

#[repr(C)]
pub struct Act {
    pub _1: [DWORD; 3],
    pub dw_map_seed: DWORD,
    pub p_room1: *mut Room1,
    pub dw_act: DWORD,
    pub _2: [DWORD; 12],
    pub p_misc: *mut ActMisc,
}

#[repr(C, packed(1))]
pub struct Path {
    pub x_offset: WORD,
    pub x_pos: WORD,
    pub y_offset: WORD,
    pub y_pos: WORD,
    pub _1: [DWORD; 2],
    pub x_target: WORD,
    pub y_target: WORD,
    pub _2: [DWORD; 2],
    pub p_room1: *mut Room1,
    pub p_room_unk: *mut Room1,
    pub _3: [DWORD; 3],
    pub p_unit: *mut UnitAny,
    pub dw_flags: DWORD,
    pub _4: DWORD,
    pub dw_path_type: DWORD,
    pub dw_prev_path_type: DWORD,
    pub dw_unit_size: DWORD,
    pub _5: [DWORD; 4],
    pub p_target_unit: *mut UnitAny,
    pub dw_target_type: DWORD,
    pub dw_target_id: DWORD,
    pub b_direction: BYTE,
}

#[repr(C, packed(1))]
pub struct ItemPath {
    pub _1: [DWORD; 3],
    pub dw_pos_x: DWORD,
    pub dw_pos_y: DWORD,
}

#[repr(C, packed(1))]
pub struct Stat {
    pub w_sub_index: WORD,
    pub w_stat_index: WORD,
    pub dw_stat_value: DWORD,
}

#[repr(C, packed(1))]
pub struct StatList {
    pub _1: [DWORD; 9],
    pub p_stat: *mut Stat,
    pub w_stat_count1: WORD,
    pub w_stat_count2: WORD,
    pub _2: [DWORD; 2],
    pub _3: *mut BYTE,
    pub _4: DWORD,
    pub p_next: *mut StatList,
}

#[repr(C, packed(1))]
pub struct Inventory1 {
    pub dw_signature: DWORD,
    pub b_game1c: *mut BYTE,
    pub p_owner: *mut UnitAny,
    pub p_first_item: *mut UnitAny,
    pub p_last_item: *mut UnitAny,
    pub _1: [DWORD; 2],
    pub dw_left_item_uid: DWORD,
    pub p_cursor_item: *mut UnitAny,
    pub dw_owner_id: DWORD,
    pub dw_item_count: DWORD,
}

#[repr(C, packed(1))]
pub struct Light {
    pub _1: [DWORD; 3],
    pub dw_type: DWORD,
    pub _2: [DWORD; 7],
    pub dw_static_valid: DWORD,
    pub pn_static_map: *mut c_int,
}

#[repr(C, packed(1))]
pub struct SkillInfo {
    pub w_skill_id: WORD,
}

#[repr(C, packed(1))]
pub struct Skill {
    pub p_skill_info: *mut SkillInfo,
    pub p_next_skill: *mut Skill,
    pub _1: [DWORD; 8],
    pub dw_skill_level: DWORD,
    pub _2: [DWORD; 2],
    pub dw_flags: DWORD,
}

#[repr(C, packed(1))]
pub struct Info {
    pub p_game1c: *mut BYTE,
    pub p_first_skill: *mut Skill,
    pub p_left_skill: *mut Skill,
    pub p_right_skill: *mut Skill,
}

#[repr(C, packed(1))]
pub struct ItemData {
    pub dw_quality: DWORD,
    pub _1: [DWORD; 2],
    pub dw_item_flags: DWORD,
    pub _2: [DWORD; 2],
    pub dw_flags: DWORD,
    pub _3: [DWORD; 3],
    pub dw_quality2: DWORD,
    pub dw_item_level: DWORD,
    pub _4: [DWORD; 2],
    pub w_prefix: WORD,
    pub _5: [WORD; 2],
    pub w_suffix: WORD,
    pub _6: DWORD,
    pub body_location: BYTE,
    pub item_location: BYTE,
    pub _7: BYTE,
    pub _8: WORD,
    pub _9: [DWORD; 4],
    pub p_owner_inventory: *mut Inventory1,
    pub _10: DWORD,
    pub p_next_inv_item: *mut UnitAny,
    pub _11: BYTE,
    pub node_page: BYTE,
    pub _12: WORD,
    pub _13: [DWORD; 6],
    pub p_owner: *mut UnitAny,
}

#[repr(C)]
pub struct MonsterData {
    pub _1: [BYTE; 22],
    pub flags: BYTE,
    pub _2: [BYTE; 5],
    pub an_enchants: [BYTE; 9],
    pub w_unique_no: WORD,
    pub _5: DWORD,
    pub w_name: [u16; 28],
}

#[repr(C)]
pub struct MonsterTxt {
    pub _1: [BYTE; 0x6],
    pub n_locale_txt_no: WORD,
    pub flag: WORD,
    pub _1a: WORD,
    pub flag1: DWORD,
    pub _2: [BYTE; 0x22],
    pub velocity: WORD,
    pub _2a: [BYTE; 0x52],
    pub tcs: [[WORD; 4]; 3],
    pub _2b: [BYTE; 0x52],
    pub sz_descriptor: [u16; 0x3c],
    pub _3: [BYTE; 0x1a0],
}

#[repr(C, packed(1))]
pub struct ObjectTxt {
    pub sz_name: [c_char; 0x40],
    pub wsz_name: [u16; 0x40],
    pub _1: [BYTE; 4],
    pub n_selectable0: BYTE,
    pub _2: [BYTE; 0x87],
    pub n_orientation: BYTE,
    pub _2b: [BYTE; 0x19],
    pub n_sub_class: BYTE,
    pub _3: [BYTE; 0x11],
    pub n_parm0: BYTE,
    pub _4: [BYTE; 0x39],
    pub n_populate_fn: BYTE,
    pub n_operate_fn: BYTE,
    pub _5: [BYTE; 8],
    pub n_auto_map: DWORD,
}

#[repr(C, packed(1))]
pub struct ObjectData {
    pub p_txt: *mut ObjectTxt,
    pub typ: DWORD,
    pub _1: [DWORD; 8],
    pub sz_owner: [c_char; 0x10],
}

#[repr(C, packed(1))]
pub struct ObjectPath {
    pub p_room1: *mut Room1,
    pub _1: [DWORD; 2],
    pub dw_pos_x: DWORD,
    pub dw_pos_y: DWORD,
}

#[repr(C)]
pub union UnitDataUnion {
    pub p_player_data: *mut PlayerData,
    pub p_item_data: *mut ItemData,
    pub p_monster_data: *mut MonsterData,
    pub p_object_data: *mut ObjectData,
}

#[repr(C)]
pub union PathUnion {
    pub p_path: *mut Path,
    pub p_item_path: *mut ItemPath,
    pub p_object_path: *mut ObjectPath,
}

#[repr(C, packed(1))]
pub struct UnitAny {
    pub dw_type: DWORD,
    pub dw_txt_file_no: DWORD,
    pub _1: DWORD,
    pub dw_unit_id: DWORD,
    pub dw_mode: DWORD,
    pub data: UnitDataUnion,
    pub dw_act: DWORD,
    pub p_act: *mut Act,
    pub dw_seed: [DWORD; 2],
    pub _2: DWORD,
    pub path: PathUnion,
    pub _3: [DWORD; 5],
    pub dw_gfx_frame: DWORD,
    pub dw_frame_remain: DWORD,
    pub w_frame_rate: WORD,
    pub _4: WORD,
    pub p_gfx_unk: *mut BYTE,
    pub p_gfx_info: *mut DWORD,
    pub _5: DWORD,
    pub p_stats: *mut StatList,
    pub p_inventory: *mut Inventory1,
    pub pt_light: *mut Light,
    pub _6: [DWORD; 9],
    pub w_x: WORD,
    pub w_y: WORD,
    pub _7: DWORD,
    pub dw_owner_type: DWORD,
    pub dw_owner_id: DWORD,
    pub _8: [DWORD; 2],
    pub p_omsg: *mut OverheadMsg,
    pub p_info: *mut Info,
    pub _9: [DWORD; 6],
    pub dw_flags: DWORD,
    pub dw_flags2: DWORD,
    pub _10: [DWORD; 5],
    pub p_changed_next: *mut UnitAny,
    pub p_room_next: *mut UnitAny,
    pub p_list_next: *mut UnitAny,
}

#[repr(C, packed(1))]
pub struct NPCMenu {
    pub dw_npc_class_id: DWORD,
    pub dw_entry_amount: DWORD,
    pub w_entry_id1: WORD,
    pub w_entry_id2: WORD,
    pub w_entry_id3: WORD,
    pub w_entry_id4: WORD,
    pub _1: WORD,
    pub dw_entry_func1: DWORD,
    pub dw_entry_func2: DWORD,
    pub dw_entry_func3: DWORD,
    pub dw_entry_func4: DWORD,
    pub _2: [BYTE; 5],
}

#[repr(C)]
pub struct OverheadMsg {
    pub _1: DWORD,
    pub dw_trigger: DWORD,
    pub _2: [DWORD; 2],
    pub msg: [c_char; 232],
}

#[repr(C)]
pub struct D2MSG {
    pub my_hwnd: *mut c_void,
    pub lp_buf: [c_char; 256],
}

#[repr(C)]
pub struct InventoryLayout {
    pub slot_width: BYTE,
    pub slot_height: BYTE,
    pub unk1: BYTE,
    pub unk2: BYTE,
    pub left: DWORD,
    pub right: DWORD,
    pub top: DWORD,
    pub bottom: DWORD,
    pub slot_pixel_width: BYTE,
    pub slot_pixel_height: BYTE,
}

#[repr(C)]
pub struct MpqTable {}

#[repr(C)]
pub struct SgptDataTable {
    pub p_player_class: *mut MpqTable,
    pub dw_player_class_records: DWORD,
    pub p_body_locs: *mut MpqTable,
    pub dw_body_locs_records: DWORD,
    pub p_store_page: *mut MpqTable,
    pub dw_store_page_records: DWORD,
    pub p_elem_types: *mut MpqTable,
}

// Unit type constants
pub const UNIT_TYPE_PLAYER: DWORD = 0;
pub const UNIT_TYPE_NPC: DWORD = 1;
pub const UNIT_TYPE_OBJECT: DWORD = 2;
pub const UNIT_TYPE_MISSILE: DWORD = 3;
pub const UNIT_TYPE_ITEM: DWORD = 4;
pub const UNIT_TYPE_TILE: DWORD = 5;
