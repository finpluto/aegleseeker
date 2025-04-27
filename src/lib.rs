use std::{ops::DerefMut, slice::from_raw_parts_mut, sync::Mutex};

use world::World;

pub mod geometry;
pub mod light;
pub mod scene;
pub mod world;

// Global Lock
static WORLD: Mutex<Option<World>> = Mutex::new(None);

fn with_world_opt_mut(f: impl FnOnce(&mut Option<World>)) -> bool {
    let guard = WORLD.lock();
    if guard.is_err() {
        return false;
    }
    let mut opt = guard.unwrap();
    f(opt.deref_mut());
    true
}

fn with_world(f: impl FnOnce(&World)) -> bool {
    let guard = WORLD.lock();
    if guard.is_err() {
        return false;
    }
    let opt = guard.unwrap();
    if opt.is_none() {
        return false;
    }
    f(opt.as_ref().unwrap());
    true
}

fn with_world_mut(f: impl FnOnce(&mut World)) -> bool {
    let guard = WORLD.lock();
    if guard.is_err() {
        return false;
    }
    let mut opt = guard.unwrap();
    if opt.is_none() {
        return false;
    }
    f(opt.as_mut().unwrap());
    true
}

#[unsafe(no_mangle)]
pub extern "C" fn as_init_scene(height: u32, width: u32) -> bool {
    with_world_opt_mut(|world_opt| {
        world_opt.replace(World::new(height, width));
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn as_deinit_world() -> bool {
    with_world_opt_mut(|world_opt| {
        let prev_world = world_opt.take();
        drop(prev_world);
    })
}

/// # Safety
///
/// It's caller's responsibility to provide a pixel buffer that
/// can contain pixel_num * 4 bytes data.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn as_draw_to_pixel_buf(buf: *mut u8) -> bool {
    with_world(|world| {
        world.tracing();

        // TODO: assuming ARGB here (32bits), can make it configurable.
        let buf_len = world.camera.get_canvase_size() as usize * 4;
        let buf = unsafe { from_raw_parts_mut(buf, buf_len) };

        world.dump_pixels(buf);
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn as_camera_yaw(yaw: f32) {
    with_world_mut(|world| world.camera.set_yaw(yaw));
}

#[unsafe(no_mangle)]
pub extern "C" fn as_camera_ztranslate(amount: f32) {
    with_world_mut(|world| world.camera.set_z_translate(amount));
}

#[unsafe(no_mangle)]
pub extern "C" fn as_light_position_offset(x: f32, y: f32, z: f32) {
    with_world_mut(|world| world.light.update_offset(x, y, z));
}
