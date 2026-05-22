---
alwaysApply: false
scene: git_message
---

# Commit Message 规则

AI 生成提交信息时，使用 Conventional Commits 风格：

<type>(<scope>): <summary>

summary 使用中文，简洁明确，不超过 50 个中文字符，不加句号。

type 只能从以下值中选择：

feat、fix、refactor、perf、docs、style、test、build、ci、chore、revert。

要求：

- 根据实际 diff 内容生成，不臆测
- 优先说明本次修改的核心目的
- 避免“修改代码”“优化逻辑”“修复问题”等空泛表达
- 多个不相关修改应提示拆分提交
- 默认只输出一行 commit message
- 只有用户要求时才生成详细 body

示例：

feat(cli): 增加批量编译参数配置
fix(parser): 修复空字符串解析失败的问题
refactor(worker): 拆分任务分发与结果收集逻辑
docs(readme): 补充 Docker 启动说明。
