# OLED-Client
一个简单的demo程序，使用tuari v2+vue3开发，用于发送消息到树莓派oled屏幕，可在src-tauri\gen\android\app\build\outputs\apk\universal\release目录下生成Android apk (本仓库已经存放了编译好的apk)，通过签名工具签名后即可安装。

# 说明
编译前需通过Android Studio IDE 安装必要的Android开发库，具体参见tauri v2文档 [https://v2.tauri.app/zh-cn/start/prerequisites/](https://v2.tauri.app/zh-cn/start/prerequisites/)

- 编译命令为
```
npm run tauri android build
```
- 调试命令为
```
npm run tauri android dev
```

服务端程序见[https://github.com/dew667/oled_server](https://github.com/dew667/oled_server)
