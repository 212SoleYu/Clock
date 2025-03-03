// use std::collections::btree_map::Entry;
// use std::fs;
use std::path::Path;
use walkdir::{WalkDir, DirEntry};
use crate::modules::history::*;
use crate::modules::time::*;
use crate::modules::actions::*;
use crate::modules::history::*;
use druid::{ Data, Env, Lens, Widget, WidgetExt, WindowDesc, TimerToken, 
    EventCtx, Event, };
use druid::widget::{Flex,Label,Button};
// use fltk::widget::Widget;




#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub current_time:       String,
    pub status:             bool,
    pub total_time:         WorkTime,
    pub current_filename:   String,

    pub last_total_time:    WorkTime,
    pub last_time_stamp:    String

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
                    let worktime_diff = time_diff(&last, &time_now);
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

pub fn build_ui() -> impl Widget<AppState> {
    let mut col = Flex::column();

    let mut realtime_row: Flex<AppState> = Flex::row();
    realtime_row.add_child(Label::new("Current time: "));
    realtime_row.add_child(Label::dynamic(|data: &AppState, _env| data.current_time.clone()).controller(TimerController::new()));
    
    col.add_child(realtime_row);
 

    let mut dict:Vec<String> = Vec::new();
    dict.push("OffDuty".to_string());
    dict.push("OnDuty".to_string());
    let mut status_row = Flex::row();
    status_row.add_child(Label::new("Current status: "));

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
    // 新增一个打开新窗口的按键
    row.add_child(Button::new("History").on_click(|_ctx,_data:& mut AppState, _env|{
        let child_window  = WindowDesc::new(build_weekly_widget())
        .title("Weekly History")
        .window_size((400.0,300.0));

    _ctx.new_window(child_window);
    }));
    col.add_child(row);

    col
}


// 如何展示每周的工作时间? 我可以想到的就是每行展示, 展示的格式如下:
// 2025/02/21~2025/02/27, hh:mm:ss button
pub fn build_weekly_widget()-> impl Widget<AppState>{
    let mut col:Flex<AppState> = Flex::column();

    col.add_child(Label:: new("Only archived records displayed."));

    // 要多少个行呢? log文件夹中有多少个文件, 就有多少个行:
    // 如何处理当前周? todo
    for entry in WalkDir::new("log").into_iter().filter_map(|e| e.ok()){
        
        let path = entry.path();
        let mut row:Flex<AppState> =Flex::row();
        if path.is_file(){
            // 返回值是lognode和一个总时间组成的元组
            let filename_string:String = path.to_str().unwrap().to_string();
            let res: (Vec<super::read::LogNode>, WorkTime) = weekly_read(&filename_string);
            if res.0.len() == 0{
                continue;
            }
            let s = iso_week_range(res.0[0].time.year, res.0[0].time.month, res.0[0].time.day).unwrap();
            row.add_child(Label::new(format!("{}~{}: {:02}:{:02}:{:02}  ",s.0,s.1,res.1.hrs,res.1.minute,res.1.sec)));
            // row.add_child(Button::new("Show").on_click(|_ctx,_data:&mut String,_env|{

            // }));
        }

        col.add_child(row);
    }




    col

}

pub fn build_daily_widget()->impl Widget<AppState>{
    Label::new("This is a new window for daily history")
}