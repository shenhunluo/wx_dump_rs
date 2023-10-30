# wx_dump_rs
<a href="https://github.com/AdminTest0/SharpWxDump">SharpWxDump</a>、<a href="https://github.com/SpenserCai/GoWxDump">GoWxDump</a>的rust语言版。包括获取信息，复制数据库，解密数据库，搜索内存等功能

在原版基础上添加解密wal文件，可以不关闭微信而获取正在产生的新记录

## 编译
cargo build --release

## 运行
target\release\wx_dump_rs.exe 执行 获取信息、复制数据库、解密数据库 三项基础功能

target\release\wx_dump_rs.exe --help 可获取帮助信息


## 免责声明
本项目指在技术验证

本项目仅允许在授权情况下对数据库进行备份，严禁用于非法目的，否则自行承担所有相关责任。使用该工具则代表默认同意该条款;

请勿利用本项目的相关技术从事非法测试，如因此产生的一切不良后果与项目作者无关。
