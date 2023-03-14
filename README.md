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
## NowTODO
- [ ] 查询文章是否存在，存在则修改，不存在则新增
- [ ] 发布文章后修改状态，添加更新page方法
- [ ] 下载图片
- [ ] doc,rs文档
- [ ] 单元测试
- [ ] 容器化
## TODO LIST
- [x] 分隔筛选和排序
- [x] 添加block结构和生成方法
- [x] 统一错误结构
- [x] markdown格式映射
- [x] 静态request模块（或可复用）
- [x] Page/Database结构和方法
- [x] md外链语法
- [x] main函数导入项目包名
- [x] databases的next方法
- [x] block自动获取所有分页
- [x] 请求分页参数处理
- [ ] 优化递归
- [ ] 异步请求
- [ ] md标准输出模式
- [ ] 代理请求
- [ ] 从notion模块中分离具体的业务逻辑
- [ ] 抽象数据库并适应不同的平台
- [ ] 优化文件体积
- [ ] 增加数据库连接的配置

## License
The MIT License (MIT). Please see [License File](LICENSE.md) for more information.
