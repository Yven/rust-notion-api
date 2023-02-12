<div align="center">

# Rust Notion Api
使用Rust调用Notion API获取内容，可以选择导出页面为`.md`或`.html`文件

</div>

## 特性
- [x] 调用Notion API获取页面内容
- [ ] 输出页面为Markdown
- [ ] 输出页面为HTML
- [ ] 作为rust-lib调用
- [ ] 使用命令行调用

## 安装

## 使用
### 1.复制并修改配置文件
```shell
cp secret.json.example secret.json
```
将你申请的Notion API密钥和要查询的databases id写入

### 2.运行
```shell
cargo run
```

## TODO LIST
- [ ] 构造请求筛选器
- [ ] Page/Database结构和方法
- [ ] 统一错误结构
- [ ] markdown格式映射

## License
The MIT License (MIT). Please see [License File](LICENSE.md) for more information.
