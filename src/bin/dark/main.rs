
use wintendo_tools::{Result, woe::Woe};
use druid::widget::{Flex, Label, Button, List, Either};

use druid::{AppLauncher, WindowDesc, FontDescriptor, FontFamily, Data, Lens, WidgetExt, Color, theme, Selector, AppDelegate, Rect, DelegateCtx, WindowId, Env};
use druid::im::{Vector};

mod shroud;

// const SCREEN_ADDED: Selector<Screen> = Selector::new("finish_fetch_dist");
const TOGGLE_SCREEN: Selector<usize> = Selector::new("toggle_screen");

#[derive(Data, Lens, Clone)]
struct ScreenState {
    #[data(same_fn="PartialEq::eq")]
    window_id: Option<WindowId>,
    index: usize,
    rect: druid::Rect,
    disabled: bool,
    primary: bool,
}

#[derive(Data, Lens, Clone)]
struct AppState {
    monitors: Vector<ScreenState>,
    gui_hidden: bool,
}

fn main() -> Result<()> {

    let data = AppState{
        monitors: druid::Screen::get_monitors().into_iter().enumerate()
            .map(|(index, monitor)| ScreenState{
                index, 
                rect: monitor.virtual_rect(), 
                disabled: false, 
                primary: monitor.is_primary(),
                window_id: None,
            })
            .collect(),
        gui_hidden: false
    };

    let app_delegate = Delegate{};
    let primary_monitor = data.monitors.iter()
        .find(|m| m.primary)
        .ok_or(Woe::new("No primary monitor found"))?;

    AppLauncher::with_window(build_window(&primary_monitor.rect, primary_monitor.index))
        .delegate(app_delegate)
        .configure_env(|env, _| {
            env.set(theme::BUTTON_LIGHT, Color::BLACK);
        })
        .launch(data)?;

    Ok(())
}

struct Delegate{
}

impl Delegate {
    
}

fn build_window(rect: &Rect, index: usize) -> WindowDesc<AppState> {
    WindowDesc::new(move || {
        let buttons = List::new(move || 
            Button::from_label(Label::new(|screen: &ScreenState, _: &_| {
                    let r = screen.rect;
                    format!("Display #{}\n({}x{} at {},{})", screen.index, r.x1, r.y1, r.x0, r.y0) 
                }))
                .on_click(move |ctx, screen, _| {
                    ctx.set_handled();
                    if screen.index != index {
                        ctx.submit_command(TOGGLE_SCREEN.with(screen.index));
                    }
                })
                .fix_size(200.0, 100.0)
                
            )
            .lens(AppState::monitors);

        Either::new(|data, _| data.gui_hidden, Label::new(""), Flex::column()
            .with_default_spacer()
            .with_child(Flex::row()
                .with_default_spacer()
                .with_child(
                    Label::new("Dark!")
                    .with_font(FontDescriptor::new(FontFamily::MONOSPACE))
                    .with_text_size(14.0)
                )
                .with_flex_spacer(1.0)
                .with_child(Button::new("Exit").on_click(|ctx, _:_, _: &_| ctx.window().close()))
                .with_default_spacer()
            )
            .with_flex_spacer(1.0)
            .with_child(buttons)
            .with_flex_spacer(1.0)
        )
        .background(Color::BLACK)
        .on_click(|ctx, data, _: &_| {
            if !ctx.is_handled() {
                data.gui_hidden = !data.gui_hidden
            }
        })
    })
    .title("Dark")
    .show_titlebar(false)
    .window_size(rect.size())
    .set_position(rect.origin())
}

impl AppDelegate<AppState> for Delegate {
    fn command(
        &mut self,
        ctx: &mut druid::DelegateCtx,
        _target: druid::Target,
        cmd: &druid::Command,
        data: &mut AppState,
        _env: &druid::Env,
    ) -> druid::Handled {
        if let Some(screen_index) = cmd.get(TOGGLE_SCREEN) {
            if let Some(monitor) = data.monitors.iter().find(|m| m.index == *screen_index) {
                
                ctx.new_window(build_window(&monitor.rect, monitor.index));
                println!("Toggeling monitor {}", monitor.index);
            }
            return druid::Handled::Yes;
        }
        druid::Handled::No
    }

    fn window_added(
        &mut self,
        id: WindowId,
        data: &mut AppState,
        _env: &Env,
        _ctx: &mut DelegateCtx,
    ) {
        println!("Window added, id: {:?}", id);
        
        //data.monitors.iter_mut().find(|m| m.window_id == id).id = 0;
        //self.windows.push(id);
        // for m in data.monitors.iter_mut() {
            
        //     if m.rect
        // }
        // druid::Screen::get_monitors()
    }

    fn window_removed(
        &mut self,
        id: WindowId,
        data: &mut AppState,
        _env: &Env,
        _ctx: &mut DelegateCtx,
    ) {
        println!("Window removed, id: {:?}", id);
        for monitor in data.monitors.iter_mut() {
            if monitor.window_id == Some(id) {
                monitor.window_id = None
            }
        }
    }
}

