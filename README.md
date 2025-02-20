# MyClock in Rust v0.1
## overview

这是一个尝试使用rust编写的工作时间累计打卡器, 其基本功能是打开然后打卡, 记录到岗和离岗时间, 从而在每周记录总的工作时长.

## Install and Build

开发模式下需要配置rust环境和cargo包管理工具, 在当前目录下使用

```shell
cargo run
```

即可自动编译并且执行.

---

由于打卡器是静态编译, 因此直接运行

```shell
/target/debug/Clock.exe
```

同样可以执行, 不受限于平台.

## How to use

用法简单, 只需要在签到时时点击左按钮, 签退时点击右按钮即可.

打卡器会显示累计的工作时长,每周刷新.

## Acknowledgment & Future

后续更新计划:

1. 更新动态时间显示
2. 优化日志路径存储
3. 重构内部结构体声明类型
4. 优化界面显示