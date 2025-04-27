pub extern fn as_init_scene(height: u32, width: u32) bool;
pub extern fn as_deinit_world() bool;
pub extern fn as_draw_to_pixel_buf(buf: [*c]u8) bool;
pub extern fn as_camera_yaw(yaw: f32) void;
pub extern fn as_camera_ztranslate(amount: f32) void;
pub extern fn as_light_position_offset(x: f32, y: f32, z: f32) void;
