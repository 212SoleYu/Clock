// Date: 2025-02-19
// 建立在read.rs把读写的基本操作完成的前提上, 在这里实现业务逻辑
// 左右键触发的时候的基本逻辑和对读写操作的调用


use std::fmt::format;
use std::fs;
use std::io::Error;
use std::num::ParseIntError;
use std::path::Path;

use crate::app_state_derived_lenses::current_time;
use crate::app_state_derived_lenses::total_time;
use crate::read::log_read;
use crate::read::log_write;
use crate::read::LogNode;
use crate::read::WorkStatus;
use crate::time;
use crate::read;
use crate::time::real_time_derived_lenses::year;
use crate::time::time_add;
use crate::time::time_diff;
use crate::time::RealTime;
use crate::time::WorkTime;
use crate::AppState;
use chrono::format::Item;
use chrono::DateTime;
use chrono::Datelike;
use chrono::Local;
use chrono::Weekday;
use druid::platform_menus::mac::file;



// 用来判断是否是新的一周的基准时间
const BASE_YEAR :u32 = 2025;
const BASE_MONTH :u32 = 2;
const BASE_DAY :u32 = 17;
const BASE_HOUR :u32 = 0;
const BASE_MINUTE :u32 = 0;
const BASE_SECOND :u32 = 0;
const BASE_WEEKDAY:Weekday = Weekday::Mon;




// 要完成左键逻辑和右键逻辑,还有一个启动检查逻辑

// 启动逻辑:
// 1. 获取当前的时间,并且分清楚当前时间的周数 已完成
// 2. 如果当前是新的一周，那么则创建新的txt文件。
// 3. 当前周对应的txt文件已经存在，那么读取当前文件中的所有内容，尤其是最后一行的WorkStatus
// 4. 如果当前的状态是离岗，那么捉对计算出累计的工作时长，
// 5. 如果当前的状态时在岗，那么计算出当前的工作时长（时刻更新）
// 6. 将当前时间，累计工作时间，当前工作状态交付给AppState。

//  这个就相当于
impl AppState{
    pub fn lazy_new()->Self{
        AppState{
            current_time:   "0".to_string(),
            status:         false,
            total_time:     WorkTime::lazy_new(),
            current_filename:"noname.txt".to_string(),
        }
    }
    pub fn show(&self){
        println!("Current time: {}",self.current_time);
        println!("Current status: {}",self.status);
        println!("Current total_time: {}",self.total_time.get_string_time());
    }
    pub fn app_init()->Self{
        // 更新内容: 需要首先创建一个存放日志的目录log/, 与clock存放在同一目录下, 
        let path = "log";
        if !Path::new(path).exists()   {
            fs::create_dir(path);
            println!("Directory has been created now");
        }
        else{
            println!("Directory has already been created before")
        }


        // 获取时间,当前时间减去一个基准时间来获知当前是第几周
        let mut current_time_now:RealTime = RealTime::lazy_new();
        current_time_now.get_real_time();
        // 计算距离基准时间过去了多久
        let mut days_diff:WorkTime = WorkTime::lazy_new();
        let BASE_TIME :RealTime = RealTime::new(BASE_YEAR, BASE_MONTH, BASE_DAY, 
            BASE_HOUR, BASE_MINUTE, BASE_SECOND, BASE_WEEKDAY);
        days_diff = time_diff(&BASE_TIME, &current_time_now);
        let weeknum :u32= (days_diff.hrs)/(24 * 7);
        
        // weeknum就是当前的周数,filename是文件的名字
        let filename :String = format!("{}/{:04}.txt",path,weeknum);

        // println!("{}",filename);

        // 生成了文件名后再进行文件创建,直接将这个逻辑放在log_read中
        // 如果是新的一周那么生成新文件读一个空vec返回
        // 如果不是新的一周那么进一步操作vec来获取知识
        let mut status:bool = false;
        let mut total_time_now:WorkTime =WorkTime::lazy_new();
        match log_read(&filename){
            Ok(vec)=>{
                println!("Successfully read operation.");
                let size = vec.len();
                if size == 0 {
                    // 新文件 直接返回 状态设为离岗 工作时间为0
                    status = false;
                    // total_time_now = WorkTime::lazy_new();
                }
                else{
                    // 不是新文件, 那么需要读取vec中的所有值
                    if size % 2 == 1
                    {
                        // 奇数,说明当前是在岗的
                        status = true;
                        for i in (0..size-1).step_by(2){
                            let tmp :WorkTime = time_diff(&vec[i].time, &vec[i+1].time);
                            total_time_now = time_add(&total_time_now, &tmp);
                        }
                    }
                    else if size % 2 == 0{
                        // 偶数 说明当前离岗
                        status = false;
                        
                        for i in (0..size).step_by(2){
                            let tmp :WorkTime = time_diff(&vec[i].time, &vec[i+1].time);
                            total_time_now = time_add(&total_time_now, &tmp);
                        }
                    }
                }
            },
            Err(e)=>eprintln!("Error in creating or reading file!"),
        }
        // todo!();
        AppState { current_time: current_time_now.get_string_time(), 
            status: status, total_time: total_time_now ,current_filename:filename}
    }
}



// 左键逻辑:首先检查当前状态, 如果是工作状态则停止 如果是非工作状态则:
// 1. 修改工作状态至在岗
// 2. 向文本中写入工作记录
// 3. 工作时间开始累计(可选)
pub fn left_button_click(app_state:&mut AppState)->Result<(),Error>{
    let current_state = app_state.status;
    if current_state == true {
        return Ok(());
    }
    app_state.status = true;
    // 生成一条日志node 然后写入
    let mut current_real_time = RealTime::lazy_new();
    current_real_time.get_real_time();
    let new_status: WorkStatus = WorkStatus::OnDuty;
    let time_string:String = format!("{}, {:?}",current_real_time.get_string_time(),new_status);
    // let lognode = LogNode::new_from_string(&time_string)?;
    match LogNode::new_from_string(&time_string){
        Ok(lognode) =>{
            log_write(&lognode, app_state.current_filename.clone());
        },
        Err(e)=>{eprintln!("Failed in parse: {}",e)},
    }
    // log_write(&lognode, app_state.current_filename.clone())?;
    // // todo!()
    Ok(())
}



// 右键逻辑: 首先检查当前状态,如果是离岗状态则停止,如果是在岗状态则:
// 1. 修改工作状态至离岗
// 2. 向文本中写入离岗记录
// 3. 计算累计时间,更新当前累计时间
pub fn right_button_click(app_state:&mut AppState)->Result<(),Error>{

    let current_state:bool = app_state.status;
    if current_state == false{
        return Ok(());
    }

    app_state.status = false;
    let mut current_real_time = RealTime::lazy_new();
    current_real_time.get_real_time();
    let new_status:WorkStatus = WorkStatus::OffDuty;
    let time_string :String = format!("{}, {:?}",current_real_time.get_string_time(),new_status);

    match LogNode::new_from_string(&time_string) {
        Ok(lognode) =>{
            // 先读一下把最后一个元素读出来
            // let mut last_node:Option<LogNode> = None;
            let mut last_node:LogNode = LogNode::lazy_new();
            match log_read(&app_state.current_filename.clone()) {
                Ok(list)=>{
                    // if let Some(item) = list.last(){
                    //     last_node = Some(item.clone());
                    // }
                    match list.last() {
                        Some(item)=>{last_node = item.clone()},
                        None=>{}
                        
                    }
                },

                Err(e)=>{eprintln!("Error occured: {}",e)},
            }
            let time_diff_new:WorkTime = time_diff(&last_node.time, &lognode.time);
            log_write(&lognode, app_state.current_filename.clone());

            // 更新累计工作时间
            app_state.total_time = time_add(&app_state.total_time, &time_diff_new);
        },        
        Err(e)=>{eprintln!("Failed in parse: {}",e)},
    }



    // todo!()
    Ok(())
}