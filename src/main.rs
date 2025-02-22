// Date: 2025-02-17
// Description: This is my clock program. 

mod read;
mod time;
mod actions;

use actions::left_button_click;
use actions::right_button_click;
use druid::widget::{Button, Flex, Label};
use druid::TimerToken;
use druid::{AppLauncher, Data, Env, Lens, Widget, WidgetExt, WindowDesc, 
    EventCtx, LifeCycle, LifeCycleCtx, UpdateCtx, Event, Selector, Command};
use read::{log_write, LogNode, WorkStatus};
use time::{time_add, time_diff, RealTime, WorkTime};
use std::ascii::AsciiExt;
use std::collections::HashMap;
use std::fs::{File, OpenOptions,};
use std::io;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use chrono::prelude::*;

const UPDATE_TIME: Selector = Selector::new("update_time");
#[derive(Clone, Data, Lens)]
struct AppState {
    current_time:       String,
    status:             bool,
    total_time:         WorkTime,
    current_filename:   String,

    // timer_id:           TimerToken,

}




fn build_ui() -> impl Widget<AppState> {
    let mut col = Flex::column();

    let mut realtime_row: Flex<AppState> = Flex::row();
    realtime_row.add_child(Label::new("Current time:"));
    realtime_row.add_child(Label::dynamic(|data: &AppState, _env| data.current_time.clone()));
    
    col.add_child(realtime_row);

    // let mut map :HashMap<bool,String> = HashMap::new();
    // map.insert(true, "OnDuty".to_string());
    // map.insert(false, "OffDuty".to_string());
    let mut dict:Vec<String> = Vec::new();
    dict.push("OffDuty".to_string());
    dict.push("OnDuty".to_string());
    let mut status_row = Flex::row();
    status_row.add_child(Label::new("Current status"));

    status_row.add_child(Label::dynamic(move |data:&AppState,_env| dict[data.status as usize].clone()));

    col.add_child(status_row);

    let mut total_time_row: Flex<AppState> = Flex::row();
    total_time_row.add_child(Label::new("Accumulated time: "));
    total_time_row.add_child(Label::dynamic(|data:&AppState,_env|data.total_time.get_string_time()));

    col.add_child(total_time_row);

    let mut row = Flex::row();
    row.add_child(Button::new("Come").on_click(|_ctx: &mut druid::EventCtx<'_, '_>, data: &mut AppState, _env| {

        // data.current_time = get_current_time();
        match left_button_click(data) {
            Ok(())=>{}
            Err(e)=>{eprintln!("Button failed: {}",e)}
        }
        
    }));
    row.add_child(Button::new("Leave").on_click(|_ctx, data: &mut AppState, _env| {

        match right_button_click(data){
            Ok( ())=>{}
            Err(e)=>{eprintln!("Button failed: {}",e)}
        }
    }));
    col.add_child(row);

    col
}

fn main() ->io::Result<()>{

    // 获取app初始状态
    let mut app_data: AppState = AppState::app_init();
    // item.show();

    let main_window: WindowDesc<AppState> = WindowDesc::new(build_ui())
        .window_size((600.0, 400.0))
        .title("My Clock App v0.1");

    let launcher = AppLauncher::with_window(main_window);
    let event_sink = launcher.get_external_handle();
    launcher
        // .delegate(AppState)
        .launch(app_data)
        .expect("Failed to launch application");


    todo!();

}

fn get_current_time() -> String {
    let now = SystemTime::now();
    let datetime: chrono::DateTime<chrono::Local> = now.into();
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}

