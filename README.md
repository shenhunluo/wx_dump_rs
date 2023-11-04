# wx_dump_rs
<a href="https://github.com/AdminTest0/SharpWxDump">SharpWxDump</a>、<a href="https://github.com/SpenserCai/GoWxDump">GoWxDump</a>的rust语言版。包括获取信息，复制数据库，解密数据库，搜索内存等功能

在原版基础上添加解密wal文件，可以不关闭微信而获取正在产生的新记录

## 编译
cargo build --release

## 运行
### 默认功能 获取信息、复制数据库、解密数据库 三项基础功能
```
target\release\wx_dump_rs.exe
```
### 获取帮助信息
```
target\release\wx_dump_rs.exe --help
```
### 获取子命令的帮助信息(例如解密功能的子命令)
```
target\release\wx_dump_rs.exe decrypt --help
```
### 搜索内存示例
#### 获取用户名的偏移量
```
target\release\wx_dump_rs.exe search -s "你的用户名"
\\ 将会在终端打印你搜索内容的偏移量。可能不唯一，可以关闭再打开微信挑选固定的值
[63486760, 63488320]
```
#### 获取key的偏移量
```
target\release\wx_dump_rs.exe search -s "你的key" --encode hex --from-all-data --real-addr
```
可以在内存中搜索你的key，并打印出你的key在虚拟内存中的地址 ：
base_addr: 2489020645376
[2489021885024]
```
target\release\wx_dump_rs.exe search -s "地址
(例如当前例子中的2489021885024)" --encode u64le
```
在内存中搜索上一条命令打印出的地址，可以获得当前key的偏移量
[63488256]


## 免责声明
本项目指在技术验证

本项目仅允许在授权情况下对数据库进行备份，严禁用于非法目的，否则自行承担所有相关责任。使用该工具则代表默认同意该条款;

请勿利用本项目的相关技术从事非法测试，如因此产生的一切不良后果与项目作者无关。
