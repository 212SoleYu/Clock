// build.rs
extern crate embed_resource;

// extern crate winres;
fn main() {
    println!("cargo:warning=构建脚本已执行！");
    if cfg!(target_os = "windows") { // 仅对 Windows 生效
        // let mut res = winres::WindowsResource::new();
        // res.set_icon("alarm.ico"); // 图标路径（相对项目根目录）
        //     // .set("ProductName", "My Clock App") // 可选：设置其他元数据
        //     // .set("FileDescription", "Awesome Application");
        // res.compile().unwrap();

        embed_resource::compile("./icon.rc");
    }
    
}