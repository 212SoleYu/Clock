// Date: 2025-02-17
// Description: This is my clock program. 

// 在windows环境下执行app 不显示终端
#![windows_subsystem = "windows"]



mod modules;

use crate::modules::app::*;
use druid::{AppLauncher,WindowDesc,Color};
use std::io;


fn main() ->io::Result<()>{

    // 获取app初始状态
    let app_data: AppState = AppState::app_init();
    // item.show();

    let main_window: WindowDesc<AppState> = 
        WindowDesc::new(build_ui())
        // .with_window_icon()
        .window_size((320.0, 130.0))
        
        .title("My Clock App v0.4");

    let launcher = AppLauncher::with_window(main_window);
    launcher
        .configure_env(|env,_|{
            env.set(druid::theme::BACKGROUND_DARK, Color::rgb8(0,0,0))
        })
        .launch(app_data)
        .expect("Failed to launch application");

    todo!();

}

