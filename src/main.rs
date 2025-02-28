// Date: 2025-02-17
// Description: This is my clock program. 

// 在windows环境下执行app 不显示终端
#![windows_subsystem = "windows"]


mod read;
mod time;
mod actions;

use actions::left_button_click;
use actions::right_button_click;
use druid::widget::{Button, Flex, Label};
use druid::TimerToken;
// use druid::{
//     widget::{ControllerHost}
// };
use druid::{AppLauncher, Data, Env, Lens, Widget, WidgetExt, WindowDesc, Color,
    EventCtx, Event, Selector};
// use read::{log_write, LogNode, WorkStatus};
use time::{time_add, time_diff, RealTime, WorkTime};
// use std::fs::{File, OpenOptions,};
use std::io;
// use std::time::{Duration, SystemTime, UNIX_EPOCH};
// use chrono::prelude::*;
// use druid::widget::Controller;


#[derive(Clone, Data, Lens)]
struct AppState {
    current_time:       String,
    status:             bool,
    total_time:         WorkTime,
    current_filename:   String,

    last_total_time:    WorkTime,
    last_time_stamp:    String

}

struct TimerController{
    timer_id:Option<TimerToken>,
}

impl TimerController{
    fn new() -> Self{
        Self {timer_id:None}
    }
}


impl<W:Widget<AppState>> druid::widget::Controller<AppState,W> for TimerController{
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut AppState, env: &Env) {
        match event {
            Event::WindowConnected =>{
                let token  = ctx.request_timer(std::time::Duration::from_secs(1));
                self.timer_id = Some(token);
            }

            Event::Timer(token) if self.timer_id == Some(*token)=>{
                // 先处理current_time, 每次都获取当前的时间, 重新计算一次, 
                // 相当于每次都在重写AppState中的当前时间字符串
                let mut time_now = RealTime::lazy_new();
                time_now.get_real_time();
                data.current_time = time_now.get_string_time();
                ctx.request_layout();
                
                // 但是累计时间的计算方式并不能照搬
                // 因为当前计算累计工作时间的方式是在leave的时候重读整个文档, 捉对相减来计算获得.
                // 如果想要获得累计的动态工作时间, 就必须将之前的累计时间和这次正在累计的时间分开
                // 首先要判断当前的工作状态, 如果当前是离岗, 那么什么都不要做
                // 如果当前是在岗状态, 那么需要将当前的时间和最后的工作时间做差 将这个累计时间不停的计算
                if data.status == true{
                    let mut last:RealTime =RealTime::lazy_new();
                    last.create_from_string(&data.last_time_stamp);
                    let mut worktime_diff = time_diff(&last, &time_now);
                    data.total_time = time_add(&data.last_total_time, &worktime_diff);
                }

                // 重置计时器
                let new_token = ctx.request_timer(std::time::Duration::from_secs(1));
                self.timer_id = Some(new_token);
            }

            _=>{}
        }
        child.event(ctx, event, data, env);
    }
}

fn build_ui() -> impl Widget<AppState> {
    let mut col = Flex::column();

    let mut realtime_row: Flex<AppState> = Flex::row();
    realtime_row.add_child(Label::new("Current time:"));
    realtime_row.add_child(Label::dynamic(|data: &AppState, _env| data.current_time.clone()).controller(TimerController::new()));
    
    col.add_child(realtime_row);
 

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
    let app_data: AppState = AppState::app_init();
    // item.show();

    let main_window: WindowDesc<AppState> = 
        WindowDesc::new(build_ui())
        // .with_window_icon()
        .window_size((320.0, 130.0))
        
        .title("My Clock App v0.3");

    let launcher = AppLauncher::with_window(main_window);
    launcher
        .configure_env(|env,_|{
            env.set(druid::theme::BACKGROUND_DARK, Color::rgb8(0,0,0))
        })
        .launch(app_data)
        .expect("Failed to launch application");

    todo!();

}

