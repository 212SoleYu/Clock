// 用来查看历史记录的按钮功能

use std::error::Error;

use chrono::{Datelike, NaiveDate, Weekday};
use druid::platform_menus::mac::file;

use crate::modules::time::{time_add, time_diff};

// 普通的weekly的页面先要有一个读文件然后返回一个总历史时间的功能.
// use crate::modules::
// use crate::modules::read::LogNode;
use super::{read::LogNode, time::WorkTime};
use super::read::log_read;
pub fn weekly_read(filename:&String)->(Vec<LogNode>,WorkTime){
    let mut time = WorkTime::lazy_new();
    // let res: Vec<LogNode> = log_read(filename);
    let res: Vec<LogNode> = match log_read(filename){
        Ok(list)=>{
            // 将历史记录捉对作差累计
            let size = list.len();
            if size % 2 == 1{
                for i in (0..size -1 ).step_by(2){
                    let tmp:WorkTime = time_diff(&list[i].time, &list[i+1].time);
                    time = time_add(&time,&tmp);
                }
            }
            else if size % 2 == 0 && size > 0 {
                for i in (0..size).step_by(2){
                    let tmp:WorkTime = time_diff(&list[i].time, &list[i+1].time);
                    time = time_add(&time,&tmp);
                }
            }

            list
            
        },
        Err(e)=>{eprintln!("Error in reading file");
        Vec::new()}
    };

    (res, time)
}


// 还要有一个函数来通过你读入的一个年月日来判断当前这天所在的周的起始和结束日期
// 参考DPSK给出的ISO方案
pub fn iso_week_range(year:u32, month:u32, day:u32)->Option<(String,String)>{
    let date = NaiveDate::from_ymd_opt(year as i32, month as u32, day as u32)?;
    let week = date.week(Weekday::Mon);


    Some((
        week.first_day().to_string(),
        week.last_day().to_string()
    ))

}