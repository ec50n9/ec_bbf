use tauri::{Manager, Runtime};
use window_shadows::set_shadow;

/// 设置窗口阴影
pub fn set_window_shadow<R: Runtime>(app: &tauri::App<R>) {
    let window = app.get_window("customization").unwrap();
    set_shadow(&window, true).expect("Unsupported platform!")
}

/// 自定义handler注册器，可以批量注册
#[macro_export]
macro_rules! register_handlers {
    ($( $module:ident :: { $( $handler:ident ),* } ),*) => {
        tauri::generate_handler![
            $(
                $( $module::$handler ),*
            ),*
        ]
    };
}
