use winit::{
    dpi::{PhysicalSize, Size},
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

pub fn get_screen_resolution() -> Result<PhysicalSize<u32>, &'static str> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).map_err(|_| "Failed to create a window")?;

    let monitor = window
        .current_monitor()
        .ok_or("Failed to get the current monitor")?;
    let size = monitor.size();

    Ok(size)
}
