pub mod food;
pub mod workout;
pub mod calculator;
pub mod state;
pub mod updater;

pub use food::{FoodItem, get_food_database, search_food};
pub use workout::{Exercise, get_workout_schemas, filter_workouts_by_category};
pub use calculator::{ProfileCalculator, Gender, ActivityLevel};
pub use state::MobileAppState;
pub use updater::AppUpdater;

pub fn version() -> &'static str {
    "0.1.0-mobile"
}

#[cfg(target_os = "android")]
mod gles_render {
    use std::ffi::c_void;

    type EGLDisplay = *mut c_void;
    type EGLSurface = *mut c_void;
    type EGLContext = *mut c_void;
    type EGLConfig = *mut c_void;
    type EGLBoolean = u32;
    type EGLNativeWindowType = *mut c_void;

    const EGL_DEFAULT_DISPLAY: *mut c_void = 0 as *mut c_void;
    const EGL_NO_DISPLAY: *mut c_void = 0 as *mut c_void;
    const EGL_NO_CONTEXT: *mut c_void = 0 as *mut c_void;
    const EGL_TRUE: EGLBoolean = 1;
    const EGL_RENDERABLE_TYPE: i32 = 0x3040;
    const EGL_OPENGL_ES2_BIT: i32 = 0x0004;
    const EGL_RED_SIZE: i32 = 0x3024;
    const EGL_GREEN_SIZE: i32 = 0x3023;
    const EGL_BLUE_SIZE: i32 = 0x3022;
    const EGL_ALPHA_SIZE: i32 = 0x3021;
    const EGL_DEPTH_SIZE: i32 = 0x3025;
    const EGL_SURFACE_TYPE: i32 = 0x3033;
    const EGL_WINDOW_BIT: i32 = 0x0004;
    const EGL_NONE: i32 = 0x3038;
    const EGL_COLOR_BUFFER_BIT: u32 = 0x4000;

    #[link(name = "EGL")]
    extern "C" {
        fn eglGetDisplay(display_id: *mut c_void) -> EGLDisplay;
        fn eglInitialize(dpy: EGLDisplay, major: *mut i32, minor: *mut i32) -> EGLBoolean;
        fn eglChooseConfig(dpy: EGLDisplay, attrib_list: *const i32, configs: *mut EGLConfig, config_size: i32, num_config: *mut i32) -> EGLBoolean;
        fn eglCreateContext(dpy: EGLDisplay, config: EGLConfig, share_context: EGLContext, attrib_list: *const i32) -> EGLContext;
        fn eglCreateWindowSurface(dpy: EGLDisplay, config: EGLConfig, win: EGLNativeWindowType, attrib_list: *const i32) -> EGLSurface;
        fn eglMakeCurrent(dpy: EGLDisplay, draw: EGLSurface, read: EGLSurface, ctx: EGLContext) -> EGLBoolean;
        fn eglSwapBuffers(dpy: EGLDisplay, surface: EGLSurface) -> EGLBoolean;
    }

    #[link(name = "GLESv2")]
    extern "C" {
        fn glClearColor(red: f32, green: f32, blue: f32, alpha: f32);
        fn glClear(mask: u32);
    }

    static mut EGL_DISP: EGLDisplay = EGL_NO_DISPLAY;
    static mut EGL_SURF: EGLSurface = 0 as *mut c_void;
    static mut EGL_CTX: EGLContext = EGL_NO_CONTEXT;

    pub unsafe fn init_egl_context(window: *mut c_void) {
        let display = eglGetDisplay(EGL_DEFAULT_DISPLAY);
        if display == EGL_NO_DISPLAY {
            log::error!("eglGetDisplay failed");
            return;
        }
        let mut major = 0;
        let mut minor = 0;
        if eglInitialize(display, &mut major, &mut minor) != EGL_TRUE {
            log::error!("eglInitialize failed");
            return;
        }

        let attribs = [
            EGL_RENDERABLE_TYPE, EGL_OPENGL_ES2_BIT,
            EGL_RED_SIZE, 8,
            EGL_GREEN_SIZE, 8,
            EGL_BLUE_SIZE, 8,
            EGL_ALPHA_SIZE, 8,
            EGL_DEPTH_SIZE, 16,
            EGL_SURFACE_TYPE, EGL_WINDOW_BIT,
            EGL_NONE,
        ];

        let mut config: EGLConfig = std::ptr::null_mut();
        let mut num_config = 0;
        if eglChooseConfig(display, attribs.as_ptr(), &mut config, 1, &mut num_config) != EGL_TRUE || num_config == 0 {
            log::error!("eglChooseConfig failed");
            return;
        }

        let ctx_attribs = [0x3098, 2, EGL_NONE];
        let context = eglCreateContext(display, config, EGL_NO_CONTEXT, ctx_attribs.as_ptr());
        if context == EGL_NO_CONTEXT {
            log::error!("eglCreateContext failed");
            return;
        }

        let surface = eglCreateWindowSurface(display, config, window, std::ptr::null());
        if surface == 0 as *mut c_void {
            log::error!("eglCreateWindowSurface failed");
            return;
        }

        if eglMakeCurrent(display, surface, surface, context) != EGL_TRUE {
            log::error!("eglMakeCurrent failed");
            return;
        }

        EGL_DISP = display;
        EGL_SURF = surface;
        EGL_CTX = context;
        log::info!("EGL context initialized and made current");
    }

    pub unsafe fn render_frame() {
        if EGL_DISP != EGL_NO_DISPLAY && EGL_SURF != 0 as *mut c_void {
            // Render health-themed background color (#1B4332 -> r: 0.1, g: 0.26, b: 0.19)
            glClearColor(0.1, 0.26, 0.19, 1.0);
            glClear(EGL_COLOR_BUFFER_BIT);
            eglSwapBuffers(EGL_DISP, EGL_SURF);
        }
    }
}

#[cfg(target_os = "android")]
#[no_mangle]
pub extern "C" fn android_main(app: android_activity::AndroidApp) {
    let _ = env_logger::builder().try_init();
    log::info!("HealthyMe Android app started");

    let state = MobileAppState::new();
    log::info!("Initialized MobileAppState with TDEE: {:.1}", state.daily_calorie_target);

    let mut graphics_initialized = false;

    loop {
        app.poll_events(Some(std::time::Duration::from_millis(16)), |event| {
            match event {
                android_activity::PollEvent::Main(android_activity::MainEvent::InitWindow { .. }) => {
                    log::info!("Android window initialized");
                    if let Some(window) = app.native_window() {
                        unsafe {
                            gles_render::init_egl_context(window.ptr().as_ptr() as *mut std::ffi::c_void);
                        }
                        graphics_initialized = true;
                    }
                }
                android_activity::PollEvent::Main(android_activity::MainEvent::Destroy { .. }) => {
                    log::info!("Android window terminated");
                    graphics_initialized = false;
                }
                android_activity::PollEvent::Main(android_activity::MainEvent::RedrawNeeded { .. }) => {
                    if graphics_initialized {
                        unsafe {
                            gles_render::render_frame();
                        }
                    }
                }
                _ => {}
            }
        });

        if graphics_initialized {
            unsafe {
                gles_render::render_frame();
            }
        }
    }
}
