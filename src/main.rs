// Date: 2025-02-17
// Description: This is my clock program. 

mod read;
mod time;
mod actions;

use actions::left_button_click;
use actions::right_button_click;
use druid::widget::{Button, Flex, Label};
use druid::{AppLauncher, Data, Env, Lens, Widget, WidgetExt, WindowDesc, EventCtx, LifeCycle, LifeCycleCtx, UpdateCtx, Event, Selector, Command};
use read::{log_write, LogNode, WorkStatus};
use time::{time_add, time_diff, RealTime, WorkTime};
use std::ascii::AsciiExt;
use std::fs::{File, OpenOptions,};
use std::io;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use chrono::prelude::*;

const UPDATE_TIME: Selector = Selector::new("update_time");
#[derive(Clone,Data, Lens)]
struct AppState {
    button1_clicks:     u32,
    button2_clicks:     u32,
    current_time:       String,
    status:             bool,
    // total_time:     String,
    total_time:         WorkTime,
    current_filename:   String,

}


fn build_ui() -> impl Widget<AppState> {
    let mut col = Flex::column();

    let mut realtime_row: Flex<AppState> = Flex::row();
    realtime_row.add_child(Label::new("Current time:"));
    realtime_row.add_child(Label::dynamic(|data: &AppState, _env| data.current_time.clone()));
    
    col.add_child(realtime_row);

    let mut status_row = Flex::row();
    status_row.add_child(Label::new("Current status"));
    status_row.add_child(Label::dynamic(|data:&AppState,_env| data.status.to_string().clone()));

    col.add_child(status_row);

    let mut total_time_row: Flex<AppState> = Flex::row();
    total_time_row.add_child(Label::new("Accumulated time: "));
    total_time_row.add_child(Label::dynamic(|data:&AppState,_env|data.total_time.get_string_time()));

    col.add_child(total_time_row);

    let mut row = Flex::row();
    row.add_child(Button::new("Come").on_click(|_ctx: &mut druid::EventCtx<'_, '_>, data: &mut AppState, _env| {
        data.button1_clicks += 1;
        // data.current_time = get_current_time();
        match left_button_click(data) {
            Ok(())=>{}
            Err(e)=>{eprintln!("Button failed: {}",e)}
        }
        
    }));
    row.add_child(Button::new("Leave").on_click(|_ctx, data: &mut AppState, _env| {
        data.button2_clicks += 1;
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
    // let initial_data = AppState {
    //     button1_clicks: 0,
    //     button2_clicks: 0,
    //     // current_time: get_current_time(),
    //     current_time:   "0".to_string(),
    //     status: true,
    //     // total_time: get_current_time(),
    //     total_time: WorkTime::lazy_new(),
    // };

    let launcher = AppLauncher::with_window(main_window);
    let event_sink = launcher.get_external_handle();
    launcher
        // .delegate(AppState)
        .launch(app_data)
        .expect("Failed to launch application");


    // let mut time1: WorkTime = WorkTime::new(1,20,0);
    // time1.show();
    // let mut time2 :WorkTime = WorkTime::new(2,45,7);
    // time2.show();
    // time2 = time_add(time1, time2);
    // time2.show();

    // let now_weekday:Weekday = Weekday::Tue;
    // // let mut earlier: RealTime = RealTime::new(2025, 2, 17, 20, 0, 0, 1);
    // let mut earlier:RealTime = RealTime::lazy_new();
    // let mut later: RealTime = RealTime::new(2025, 2, 18, 1, 30, 45, now_weekday);

    // later.get_real_time();
    
    // earlier.show();
    
    // later.show();

    // earlier.copy_from(&later);
    // earlier.show();
    // // let diff:WorkTime = time_diff(earlier, later);
    // // diff.show();

    // let now_status:WorkStatus = WorkStatus::OnDuty;
    // let mut log:LogNode = LogNode::new(&later, now_status);
    // log.show();

    // match log_write(&log, "test.txt".to_string()){
    //     Ok(())=> println!("OK"),
    //     Err(e) => println!("Not OK"),
    // }

    // let node :LogNode = LogNode::lazy_new();
    // let s: String = "2025-02-19 15:19:24, Tue, OnDuty".to_string();
    // match LogNode::new_from_string(&s){
    //     Ok(node)=> {
    //         println!("{}",node.get_string_log());
    //     },
    //     Err(e) => eprintln!("Error"),

    // }

    // read::log_read("test.txt".to_string());

    


//     match OpenOptions::new()
//     .create(true)
//     .read(true)
//     .write(true)
//     .open("0000.txt") {
//     Ok(item) => {
//         println!("File opened or created successfully.");
//     },
//     Err(e) => {
//         eprintln!("Error occurred: {}", e);
//     },
// }
    todo!();

}

fn get_current_time() -> String {
    let now = SystemTime::now();
    let datetime: chrono::DateTime<chrono::Local> = now.into();
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}

