<div align="center">

# Rust Notion API
使用Rust调用Notion API获取内容，导入到Typecho的文章数据库中，或导出页面为`.md`或`.html`文件

</div>

## 特性
- [x] 调用Notion API获取页面内容
- [x] 写入到Typecho数据库中
- [x] 输出页面为Markdown、HTML
- [ ] 作为rust-lib调用
- [ ] 使用命令行调用
- [ ] 使用Docker运行

## 安装

## 使用
### 1.复制并修改配置文件
```shell
cp env.example .env
```
1. 参考[官方文档](https://developers.notion.com/docs/create-a-notion-integration)创建integration并获取Token，然后填入key中
2. 在你的文章Database中点击share然后copy link获取链接中的database_id
   1. 如此形式：`https://www.notion.so/{name}/{database_id}?v={view_id}`
3. 填写你的Typecho数据库配置
```
# 填入Notion Integration Token
KEY=
# 填入database id
DB_ID=

# 以下为Typecho数据库的配置
DB_HOST=
DB_USER=
DB_PASSWORD=
DB_NAME=
```

### 2.运行
```shell
cargo run
```

## TODO
- [ ] 文章发布流程
- [ ] 生成静态页面
- [ ] doc,rs文档
- [ ] 单元测试
- [ ] 容器化

## License
The MIT License (MIT). Please see [License File](LICENSE.md) for more information.
