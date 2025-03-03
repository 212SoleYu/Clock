// Date: 2025-02-19
// 建立在read.rs把读写的基本操作完成的前提上, 在这里实现业务逻辑
// 左右键触发的时候的基本逻辑和对读写操作的调用


// use std::fmt::format;
use std::fs;
use std::io::Error;
use std::path::Path;
use crate::modules::read::*;
use crate::modules::time::*;
use crate::AppState;
use chrono::Weekday;



// 用来判断是否是新的一周的基准时间
const BASE_YEAR :u32 = 2025;
const BASE_MONTH :u32 = 2;
const BASE_DAY :u32 = 17;
const BASE_HOUR :u32 = 0;
const BASE_MINUTE :u32 = 0;
const BASE_SECOND :u32 = 0;
const BASE_WEEKDAY:Weekday = Weekday::Mon;


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
            current_time:       "0".to_string(),
            status:             false,
            total_time:         WorkTime::lazy_new(),
            current_filename:   "noname.txt".to_string(),
            last_total_time:    WorkTime::lazy_new(),
            last_time_stamp:    "0".to_string(),

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
            let _ = fs::create_dir(path);
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

        // 两个累计时间都先初始化为0
        let mut total_time_now:WorkTime = WorkTime::lazy_new();
        let mut last_total_time_now:WorkTime = WorkTime::lazy_new();
        let mut last_time_stamp_now:String  = "0".to_string();
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
                        // 将当前的状态设置为在岗
                        status = true;
                        // 捉对算当前的累计时间
                        let mut i = 0;
                        for i in (0..size-1).step_by(2){
                            let tmp :WorkTime = time_diff(&vec[i].time, &vec[i+1].time);
                            last_total_time_now = time_add(&last_total_time_now, &tmp);
                        }
                        // 此前累计的工作时间存在last_total_time_now里
                        // 最老的时间戳存在last_time_stamp_now里
                        last_time_stamp_now = vec[size-1].time.get_string_time();

                        // 在岗的时候, 需要将WorkTime手动计算出来
                        total_time_now = time_add(&last_total_time_now, &time_diff(&vec[size -1].time, &current_time_now))

                    }
                    else if (size % 2 == 0) && (size > 0){
                        // 偶数 说明当前离岗
                        status = false;
                        let mut i = 0;
                        for i in (0..size).step_by(2){
                            let tmp :WorkTime = time_diff(&vec[i].time, &vec[i+1].time);
                            last_total_time_now = time_add(&last_total_time_now, &tmp);
                        }
                        // 离岗的情况下, 此前工作时间和当前工作时间是一样的内容
                        total_time_now = last_total_time_now.clone();
                        last_time_stamp_now = vec[size-1].time.get_string_time(); 
                    }
                }
            },
            Err(e)=>eprintln!("Error in creating or reading file!"),
        }
        // todo!();


        AppState { 
            current_time: current_time_now.get_string_time(), 
            status: status, 
            total_time: total_time_now ,
            current_filename:filename,
            last_total_time:last_total_time_now,
            last_time_stamp:last_time_stamp_now
        }
    }
}



// 左键逻辑:首先检查当前状态, 如果是工作状态则停止 如果是非工作状态则:
// 1. 修改工作状态至在岗
// 2. 向文本中写入工作记录
// 3. 工作时间开始累计,将最老的签到记录设置为当前时间, 最老的累计时间不变
pub fn left_button_click(app_state:&mut AppState)->Result<(),Error>{
    let current_state = app_state.status;
    if current_state == true {
        return Ok(());
    }
    // app_state.status = true;
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

    // 将此前最老是时间戳设置为当下
    app_state.last_time_stamp = current_real_time.get_string_time();
    // 最后再修改status
    app_state.status = true;
    Ok(())
}



// 右键逻辑: 首先检查当前状态,如果是离岗状态则停止,如果是在岗状态则:
// 1. 修改工作状态至离岗
// 2. 向文本中写入离岗记录
// 3. 计算累计时间,更新当前累计时间, 将最老累计时间更新至当前时间, 最老时间戳不变
pub fn right_button_click(app_state:&mut AppState)->Result<(),Error>{

    let current_state:bool = app_state.status;
    if current_state == false{
        return Ok(());
    }

    
    let mut current_real_time = RealTime::lazy_new();
    current_real_time.get_real_time();
    let new_status:WorkStatus = WorkStatus::OffDuty;
    let time_string :String = format!("{}, {:?}",current_real_time.get_string_time(),new_status);

    // 由于AppState中已经存储了最后一次工作的时间, 所以在右键时不需要再进行读文件
    match LogNode::new_from_string(&time_string){
        Ok(lognode)=>{
            let _ = log_write(&lognode, app_state.current_filename.clone());
        }
        Err(e) => {eprintln!("Failed in parse: {}",e)},
    }



    app_state.last_total_time = app_state.total_time.clone();
    // 最后再设置状态
    app_state.status = false;
    // todo!()
    Ok(())
}