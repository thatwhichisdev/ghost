use gpui::{App, AppContext};

fn main() {
    let app = gpui_platform::application();
    let app_config = ghost_shell_config::load().expect("ghost-shell config is not present");

    app.run(|cx: &mut App| {
        let app_state = ghost_shell_app::AppState::new(app_config, cx.displays());

        cx.set_global(app_state);

        for window_options in ghost_shell_app::windows_options(cx) {
            cx.open_window(window_options, |_, cx| cx.new(|_| ghost_shell_app::Bar))
                .expect("failed to create window on display");
        }

        cx.activate(true);
    });
}
