use druid::{BoxConstraints, LayoutCtx, Env, LifeCycle, LifeCycleCtx, Rect, EventCtx, Event, UpdateCtx, Widget, PaintCtx, Size, RenderContext, Color};

use crate::{AppState};

struct Shroud<W> {
    inner: W,
}

impl<W> Shroud<W> {
    pub fn new(inner: W) -> Shroud<W> {
        Shroud { inner }
    }
}

impl<W: Widget<AppState>> Widget<AppState> for Shroud<W> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, env: &Env) {
        match event {
            Event::WindowConnected => {
                let win = ctx.window();
                let window_center = Rect::from_origin_size(win.get_position(), win.get_size()).center();
    
                for monitor in data.monitors.iter_mut() {
                    if monitor.rect.contains(window_center) {
                        monitor.window_id = Some(ctx.window_id());
                    }
                }
            }
            _ => {}
        }
        self.inner.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &AppState, env: &Env) {
        self.inner.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &AppState, data: &AppState, env: &Env) {
        // let hide_content = match data.monitors.iter().find(|m| m.window_id == Some(ctx.window_id())) {
        //     Some(screen)  => true,
        //     _ => false
        // };
        if old_data.gui_hidden != data.gui_hidden {
            ctx.request_paint();
        }
        self.inner.update(ctx, old_data, data, env);
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &AppState,
        env: &Env,
    ) -> Size {
        self.inner.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState, env: &Env) {
        if data.gui_hidden {
            let region = ctx.region().bounding_box();
            ctx.fill(region, &Color::BLACK);
        } else {
            self.inner.paint(ctx, data, env);
        }
    }
}
