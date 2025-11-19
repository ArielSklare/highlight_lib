pub mod get_highlighted;
pub mod keyboard_mapping;

pub use get_highlighted::{get_highlighted_text, replace_highlighted_text};
pub use keyboard_mapping::{
    KeyboardDirection, KeyboardLayout, LayoutMap, all_layout_vk_maps, get_layout,
    get_text_leyaout_map, list_layouts, shift_text_language, vk_to_char_map_default,
    vk_to_char_map_for_layout,
};
