// Date: 2025-02-18

// use core::time;
use std::{fmt::format, time::SystemTime};
use chrono::prelude::*;
use druid::{Data, Lens};
// use druid::platform_menus::mac::file::print;

#[derive(Debug,Lens,Clone)]
pub struct RealTime{
    year:       u32,
    month:      u32,
    day:        u32,
    hrs:        u32,
    minute:     u32,
    sec:        u32,
    weekday:    Weekday, 
    // action:     String, // 工作日的表示和签到/签离 尝试使用枚举类型?
}



impl RealTime {
    pub fn new (year: u32, month: u32, day: u32, hrs: u32, minute: u32, sec: u32, weekday: Weekday) -> Self{
        RealTime {
            year,
            month,
            day,
            hrs,
            minute,
            sec,
            weekday,
        }
    }

    pub fn lazy_new() -> Self {
        let weekday:Weekday = Weekday::Mon;
        RealTime{
            year:0,
            month: 0,
            day:0,
            hrs:0,
            minute:0,
            sec:0,
            weekday: weekday
        }
    }

    pub fn copy_from(&mut self,item:&RealTime) {
        self.year = item.year;
        self.month = item.month;
        self.day = item.day;
        self.hrs = item.hrs;
        self.minute = item.minute;
        self.sec = item.sec;
        self.weekday  = item.weekday;

    }

    // 将真实时间整理成字符串方便读写操作
    pub fn get_string_time(&self)->String{
        format!("{:04}-{:02}-{:02} {:02}:{:02}:{:02}, {:?}",
            self.year, self.month, self.day, self.hrs, self.minute, self.sec, self.weekday
        )
    }
    
    pub fn show(&self) {
    //    let s: String = format!("{:02}-{:02}-{:02} {:02}:{:02}:{:02}, {}",self.year,self.month,self.day,self.hrs,self.minute,self.sec,self.weekday);
       let s: String = self.get_string_time();
       println!("RealTime: {}",s);
    }

    pub fn get_real_time(&mut self) {
        let now: SystemTime = SystemTime::now();
        let datetime: DateTime<Local> = now.into();
        let timestring = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
        
        // 这里是一个必须显式的声明rust类型的地方 不然会报错
        let parts:Vec<&str> = timestring.split(|c: char| c == ' ' || c == '-' || c == ':').collect();

        let mut tmp : Result<u32,_> = parts[0].parse();
        match  tmp {
            Ok(tmp) => self.year = tmp,
            Err(_) => println!("Failed to transfer time to digital!"),
        }
        tmp = parts[1].parse();
        match  tmp {
            Ok(tmp) => self.month = tmp,
            Err(_) => println!("Failed to transfer time to digital!"),
        }
        tmp = parts[2].parse();
        match  tmp {
            Ok(tmp) => self.day = tmp,
            Err(_) => println!("Failed to transfer time to digital!"),
        }
        tmp = parts[3].parse();
        match  tmp {
            Ok(tmp) => self.hrs = tmp,
            Err(_) => println!("Failed to transfer time to digital!"),
        }
        tmp = parts[4].parse();
        match  tmp {
            Ok(tmp) => self.minute = tmp,
            Err(_) => println!("Failed to transfer time to digital!"),
        }
        tmp = parts[5].parse();
        match  tmp {
            Ok(tmp) => self.sec = tmp,
            Err(_) => println!("Failed to transfer time to digital!"),
        }

        // 更新周数
        self.weekday = datetime.weekday();
        return ();
    }

    pub fn to_naive_datetime(&self) -> NaiveDateTime {
        NaiveDateTime::parse_from_str(
            &format!(
                "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
                self.year, self.month, self.day, self.hrs, self.minute, self.sec
            ),
            "%Y-%m-%d %H:%M:%S",
        )
        .unwrap()
    }
}

// 累计工作的时间 不需要天以上的单位 也不需要周几
#[derive(Clone,Data,Lens)]
pub struct WorkTime{
    pub hrs:        u32,
    minute:     u32,
    sec:        u32,
}

impl WorkTime {
    pub fn new(h:u32,m:u32,s:u32) -> Self{
        WorkTime {hrs:h, minute:m, sec:s}
    }
    pub fn lazy_new()->Self{
        WorkTime{hrs:0,minute:0,sec:0}
    }
    pub fn show(&self){ 
        println!("WorkTime: {:02}:{:02}:{:02}",self.hrs,self.minute,self.sec);
    }
    pub fn get_string_time(&self)->String{
        format!("{:02}:{:02}:{:02}",
             self.hrs, self.minute, self.sec
        )
    }
}

pub fn time_add(earlier:&WorkTime, time_length:&WorkTime) -> WorkTime{

    // 实现时间结构体的加法,只需要考虑到小时级别就可以, 小时以上不需要再进位

    // 秒级别
    let mut sec :u32 = earlier.sec + time_length.sec;
    let sec_flag:u32;
    if sec >= 60{
        sec = sec- 60;
        sec_flag = 1;
    }
    else {
        sec_flag = 0;
    }

    // 分钟级别
    let mut minute: u32 = earlier.minute + time_length.minute + sec_flag;
    let  min_flag:u32;
    if minute >= 60  {
        minute = minute - 60;
        min_flag = 1;
    }
    else {
        min_flag = 0;
    }

    //小时级别
    let hrs:u32 = earlier.hrs + time_length.hrs + min_flag;

 
    WorkTime {hrs:hrs,minute:minute,sec:sec}
}

pub fn time_diff(earlier:&RealTime,later:&RealTime) -> WorkTime{

    // 由于时间相减是比较复杂的, 设计很多种特殊情况, 因此尝试直接使用chrono库计算
    let earlier_dt: NaiveDateTime = earlier.to_naive_datetime();
    let later_dt: NaiveDateTime = later.to_naive_datetime();
    let length: chrono::TimeDelta =later_dt - earlier_dt;

    // println!("{}:{:02}:{:02}:{:02}",length.num_days() % 10, length.num_hours() % 24 ,length.num_minutes() % 60,length.num_seconds() % 60);

    WorkTime {
        hrs:(length.num_hours()) as u32,
        minute:(length.num_minutes() % 60) as u32,
        sec:(length.num_seconds() % 60) as u32,
    }
       
}