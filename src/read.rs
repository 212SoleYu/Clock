// read.rs
// Date: 2025-02-17
// Description: 读写文件的操作全部在这里

use std::fs::{File, OpenOptions};
use std::io::{self, Write, BufRead, BufReader};
use std::str::FromStr;
use std::num::ParseIntError;
use chrono::format::Item;
use chrono::{ParseError, Weekday};
use druid::platform_menus::mac::file::print;
use druid::platform_menus::win::file::new;

use crate::time::RealTime;


// 初步设计日志条目格式为: YY-MM-DD hh:mm:ss action
// 表示到岗和离岗的action可以用简单的u8 或者用枚举类型
// 可能需要建立一个专门的日志条目结构体, 包含一个RealTime 和一个表示日志内容的action
#[derive(Debug,PartialEq,Clone)]
pub enum WorkStatus{
    OnDuty,
    OffDuty,
}

#[derive(Debug,Clone)]
pub struct LogNode {
    pub time :      RealTime,
    action :    WorkStatus
}

impl LogNode{
    pub fn new(another_time:&RealTime,action:WorkStatus)->Self{
        let mut time:RealTime = RealTime::lazy_new();
        time.copy_from(another_time);
        LogNode {
            time,
            action,
        }
    }

    pub fn lazy_new()->Self{
        let time:RealTime = RealTime::lazy_new();
        let status:WorkStatus = WorkStatus::OffDuty;
        LogNode {
            time:time,
            action:status,
        }
    }

    //  在读取文件的时候, 从模式化的字符串中直接初始化lognode
    pub fn new_from_string(node_string:&String)->Result<Self,ParseIntError>{
        // 字符串的模式为: "YYYY-MM-DD hh:mm:ss, Mon, Onduty", line读取没有换行符 
        // 先划分出数值
        let parts:Vec<&str> = node_string.split(|c: char|  c == ' ' || c == ',' ||c == '-' || c == ':')
        .filter(|&x| !x.is_empty())
        .collect();
        // 解析各个部分
        let year: u32 = parts[0].parse()?;
        let month: u32 = parts[1].parse()?;
        let day: u32 = parts[2].parse()?;
        let hour: u32 = parts[3].parse()?;
        let minute: u32 = parts[4].parse()?;
        let second: u32 = parts[5].parse()?;
        let weekday = Weekday::from_str(parts[6]).unwrap(); // 这里假设解析不会失败
        let action = match parts[7] {
            "OnDuty" => WorkStatus::OnDuty,
            "OffDuty" => WorkStatus::OffDuty,
            _ => WorkStatus::OffDuty, // 默认值
        };

        let time = RealTime::new(year, month, day, hour, minute, second, weekday);
        Ok(LogNode { time, action })
    }

    pub fn show(&self)->(){
        // self.time.show();
        // if  self.action == WorkStatus::OnDuty{
        //     println!("Action");
        // }
        // else {
        //     println!("OffWork");
        // }
        let s: String = self.get_string_log();
        println!("{}",self.get_string_log());
    }

    pub fn get_string_log(&self)->String{
        format!("{}, {:?}\n",self.time.get_string_time(),self.action)
    }
}


pub fn get_current_path() {
    let path = std::env::current_dir().unwrap();
    println!("The current directory is {}", path.display());
}

// 一个读文件操作 一个写文件操作 都可以写成独立的函数 

// 先来写文件操作, 将一个文件节点写入指定的文件名
// 把每周刷新的逻辑放在函数之前, 只专注读写文件的操作

// 写文件简单 把lognode的信息整理成字符串然后写入
pub fn log_write(lognode:&LogNode, logfile:String) -> io::Result<()>{
    let mut file: File = OpenOptions::new()
    .append(true)
    .create(true)
    .open(logfile)?;

    let s:String = lognode.get_string_log();
    file.write_all(s.as_bytes())?;  // 加了?可以自动处理掉失败的情况 否则会有Warning

    Ok(())
}


// 读文件的功能应该是读一整个文件
// 如果只是读取文件的话应该返回日志记录的数组
// 将所有的内容全部读出来 然后按行划分 组成一个数组
pub fn log_read(logfile:&String) -> io::Result<Vec<LogNode>>{

    let mut vec:Vec<LogNode> = Vec::new();
    let file: File = OpenOptions::new()
    .create(true)
    .read(true)
    .write(true)
    .open(logfile)?;

    let reader: BufReader<File> = BufReader::new(file);

    for line in reader.lines(){
        match line {
            Ok(line)=>{
                match LogNode::new_from_string(&line){
                    Ok(node)=>vec.push(node),
                    Err(e)=>{},
                }
                // let node:LogNode = LogNode::new_from_string(&line); 
                // vec.push(node);
            },
            Err(e)=>eprintln!("Error reading line: {}",e),
        }
    }

    // 验证
    // for item in &vec {
    //     item.show();
    // } 

    Ok(vec)

}

