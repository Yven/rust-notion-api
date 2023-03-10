<div align="center">

# Rust Notion Api
使用Rust调用Notion API获取内容，可以选择导出页面为`.md`或`.html`文件

</div>

## 特性
- [x] 调用Notion API获取页面内容
- [x] 输出页面为Markdown
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
- [x] 构造请求筛选器
- [x] 分隔筛选和排序
- [x] 分离筛选模块中的属性模块，方便复用
- [x] 添加block结构和生成方法
- [x] 优化各结构to_string方法
- [x] 整理结构和调用关系（调整notion模块的结构）
- [x] 完成筛选模块中属性的其他匹配方法
- [x] 统一错误结构
- [x] markdown格式映射
- [ ] <del>常用函数整理为宏</del>
- [x] 不同的block|rich_text附带的属性特殊处理
- [x] 优化property::new()方法
- [ ] 优化递归
- [x] 完成请求模块的其他方法
- [x] 静态request模块（或可复用）
- [ ] <del>全局可复用</del>
- [x] Page/Database结构和方法
- [ ] 请求分页参数处理
- [ ] 异步请求
- [ ] 单元测试
- [ ] doc,rs文档
- [ ] 容器化
- [x] md外链语法
- [ ] md标准输出模式
- [x] main函数导入项目包名
- [ ] 代理请求
- [ ] 调整包结构作为lib

## License
The MIT License (MIT). Please see [License File](LICENSE.md) for more information.
