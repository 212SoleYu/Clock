// Date: 2025-02-17
// Description: This is my clock program. 

mod read;
mod time;


use druid::widget::{Button, Flex, Label};
use druid::{AppLauncher, Data, Env, Lens, TimerToken, Widget, WidgetExt, WindowDesc};
use time::{time_add, time_diff, RealTime, WorkTime};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use chrono::prelude::*;


#[derive(Clone, Data, Lens)]
struct AppState {
    button1_clicks: u32,
    button2_clicks: u32,
    current_time:   String,
    status:         bool,
    total_time:     String,
}

fn build_ui() -> impl Widget<AppState> {
    let mut col = Flex::column();

    let mut realtime_row: Flex<AppState> = Flex::row();
    realtime_row.add_child(Label::new("Current time:"));
    realtime_row.add_child(Label::dynamic(|data: &AppState, _env| data.current_time.clone()));
    
    col.add_child(realtime_row);
    // col.add_child(Label::dynamic(|data: &AppState, _env| data.current_time.clone()).center());
    // col.add_child(Label::new("Current status:").center());

    let mut status_row = Flex::row();
    status_row.add_child(Label::new("Current status"));
    status_row.add_child(Label::dynamic(|data:&AppState,_env| data.status.to_string().clone()));

    col.add_child(status_row);
    col.add_child(Label::new("Accumulated time:").center());

    let mut row = Flex::row();
    row.add_child(Button::new("Come").on_click(|_ctx, data: &mut AppState, _env| {
        data.button1_clicks += 1;
        data.current_time = get_current_time();
        data.status = data.status ^ true;
    }));
    row.add_child(Button::new("Leave").on_click(|_ctx, data: &mut AppState, _env| {
        data.button2_clicks += 1;
    }));
    col.add_child(row);
    col
}

fn main() {
    // let main_window: WindowDesc<AppState> = WindowDesc::new(build_ui())
    //     .window_size((600.0, 400.0))
    //     .title("My Clock App");
    // let initial_data = AppState {
    //     button1_clicks: 0,
    //     button2_clicks: 0,
    //     current_time: get_current_time(),
    //     status: true,
    //     total_time: get_current_time(),
    // };

    // let launcher = AppLauncher::with_window(main_window);
    // let event_sink = launcher.get_external_handle();
    // launcher
    //     // .delegate(AppState)
    //     .launch(initial_data)
    //     .expect("Failed to launch application");


    let mut time1: WorkTime = WorkTime::new(1,20,0);
    time1.show();
    let mut time2 :WorkTime = WorkTime::new(2,45,7);
    time2.show();
    time2 = time_add(time1, time2);
    time2.show();


    let earlier: RealTime = RealTime::new(2025, 2, 17, 20, 0, 0, 1);
    let mut later: RealTime = RealTime::new(2025, 2, 18, 1, 30, 45, 2);
    later.get_real_time();
    earlier.show();
    later.show();
    let diff:WorkTime = time_diff(earlier, later);
    diff.show();

 








}

fn get_current_time() -> String {
    let now = SystemTime::now();
    let datetime: chrono::DateTime<chrono::Local> = now.into();
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}

