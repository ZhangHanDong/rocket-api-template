# 说明

基于[Rocket](https://github.com/SergioBenitez/Rocket)框架快速建立API微服务的项目模板，方便快捷。

更多示例可以去官方demo查看。

# 本地安装和测试

## 安装

需要Rust最新nightly版本，推荐使用rustup安装脚本：

```
curl -sSf https://static.rust-lang.org/rustup.sh | sh
```

## 启动测试环境

```
$ cargo run
```

运行测试：

```
$ cargo test
```

# Launch Server

```
#development
cargo run

# production
 sudo ROCKET_ENV=production /home/vagrant/.cargo/bin/cargo run --release
```

# Docker命令

```
$  docker-compose up -d

# rebuild

$ docker-compose up -d --build

```

# TODO

1. DB支持
2. 处理JSON最佳实践
3. 中间件支持
4. 其他
