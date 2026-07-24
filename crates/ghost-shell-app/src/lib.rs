use std::rc::Rc;

use gpui::{
    App, Context, DisplayId, Global, IntoElement, Pixels, PlatformDisplay, Render, Size, Window,
    WindowBackgroundAppearance, WindowBounds, WindowKind, WindowOptions, div,
    layer_shell::{Anchor, KeyboardInteractivity, Layer, LayerShellOptions},
    point,
    prelude::*,
    px, rgb, rgba,
};
use serde::Deserialize;

pub struct Bar;

impl Render for Bar {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .self_flex_end()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .text_color(rgb(0xffffff))
            .bg(rgba(0x00000000))
            .text_sm()
            .child("<bar>")
    }
}

pub struct BarConfig {
    pub display_id: DisplayId,
    pub size: Size<Pixels>,
}

pub struct AppState {
    config: AppConfig,
    bars: Vec<BarConfig>,
}

impl AppState {
    pub fn new(config: AppConfig, displays: Vec<Rc<dyn PlatformDisplay>>) -> Self {
        let bars: Vec<BarConfig> = displays
            .iter()
            .map(|display| {
                let display_size = display.bounds().size;
                BarConfig {
                    display_id: display.id(),
                    size: Size::new(display_size.width, px(config.bar_height)),
                }
            })
            .collect();

        Self { config, bars: bars }
    }
}

impl Global for AppState {}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AppConfig {
    pub bar_height: f32,
    pub bar_exclusive_zone: f32,
}

pub fn windows_options(cx: &mut App) -> Vec<WindowOptions> {
    let app_state = cx.global::<AppState>();

    app_state
        .bars
        .iter()
        .map(|bar| {
            let app_id: String = format!("ghost-shell-{:?}", bar.display_id);
            let namespace: String = format!("namespace-{:?}", bar.display_id);
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(gpui::Bounds {
                    origin: point(px(0.0), px(0.0)),
                    size: bar.size,
                })),
                titlebar: None,
                focus: false,
                show: true,
                kind: WindowKind::LayerShell(LayerShellOptions {
                    namespace: namespace,
                    layer: Layer::Top,
                    anchor: Anchor::TOP | Anchor::LEFT | Anchor::RIGHT,
                    exclusive_zone: Some(px(app_state.config.bar_exclusive_zone)),
                    keyboard_interactivity: KeyboardInteractivity::OnDemand,
                    ..Default::default()
                }),
                is_movable: true,
                app_owns_titlebar_drag: false,
                is_resizable: true,
                is_minimizable: true,
                display_id: Some(bar.display_id),
                window_background: WindowBackgroundAppearance::Blurred,
                app_id: Some(app_id),
                window_min_size: None,
                window_decorations: None,
                icon: None,
                tabbing_identifier: None,
            }
        })
        .collect()
}
