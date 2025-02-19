// Date: 2025-02-19
// 建立在read.rs把读写的基本操作完成的前提上, 在这里实现业务逻辑
// 左右键触发的时候的基本逻辑和对读写操作的调用


use crate::time;
use crate::read;
use crate::time::real_time_derived_lenses::year;
use crate::time::time_diff;
use crate::time::RealTime;
use crate::time::WorkTime;
use crate::AppState;
use chrono::DateTime;
use chrono::Datelike;
use chrono::Local;
use chrono::Weekday;



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
            button1_clicks: 0,
            button2_clicks: 0,
            current_time:   "0".to_string(),
            status:         false,
            total_time:     WorkTime::lazy_new(),
        }
    }
    pub fn app_init()->Self{
        // 获取时间,当前时间减去一个基准时间来获知当前是第几周
        let mut current_time:RealTime = RealTime::lazy_new();
        current_time.get_real_time();
        // 计算距离基准时间过去了多久
        let mut days_diff:WorkTime = WorkTime::lazy_new();
        let BASE_TIME :RealTime = RealTime::new(BASE_YEAR, BASE_MONTH, BASE_DAY, 
            BASE_HOUR, BASE_MINUTE, BASE_SECOND, BASE_WEEKDAY);
        days_diff = time_diff(&BASE_TIME, &current_time);
        let weeknum :u32= (days_diff.hrs)/(24 * 7);
        
        // weeknum就是当前的周数,filename是文件的名字
        let filename :String = format!("{:04}.txt",weeknum);

        println!("{}",filename);

        todo!();

    }
}



// 左键逻辑:首先检查当前状态, 如果是工作状态则停止 如果是非工作状态则:
// 1. 修改工作状态至在岗
// 2. 向文本中写入工作记录
// 3. 工作时间开始累计(时刻更新)



// 右键逻辑: 首先检查当前状态,如果是离岗状态则停止,如果是在岗状态则:
// 1. 修改工作状态至离岗
// 2. 向文本中写入离岗记录
