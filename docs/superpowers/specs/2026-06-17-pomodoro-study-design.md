# Pomodoro Study — Design Spec

**日期**: 2026-06-17
**状态**: Approved (brainstorm 阶段产物)
**项目根目录**: `C:\Users\31445\Documents\pomodoro-study`

---

## 1. 目标与范围

一款桌面端学习用番茄钟工具,核心价值是 **「让长期学习目标可量化」**。

### 1.1 必做功能 (in-scope)

- 番茄钟计时(学习 / 短休 / 长休)
- 学习目标管理(长期目标)
- 任务清单管理(挂在目标下,可预估番茄数)
- 任务 ↔ 番茄关联(每个番茄绑定一个任务,自动累计统计)
- 打断记录(主动打断 + 自定义文本原因)
- 统计与历史(今日 / 本周 / 本月 / 累计)
- 多种 UI 风格切换(首发: 酸性艺术;预留: 合成器浪潮等)
- 系统通知(toast / 提示音 / 全屏遮罩 / 任务栏闪烁,均可关)
- 设置项(时长 / 通知 / 主题 / 数据导入导出)

### 1.2 不做 (out-of-scope)

- 白噪音 / 背景音
- 网站或应用屏蔽
- 团队 / 社交 / 协作功能
- 云同步 (后续可加,首版纯本地)
- 移动端

### 1.3 非功能性目标

- **响应式设计**:窗口大小变化 UI 自适应
- **精致微交互与动效**:符合主题风格(酸性艺术 = 硬切 / 硬阴影 / 步进动画)
- **视觉层次清晰**
- **高端、专业、科技感**
- **未来 Agent 友好**:文档完备,边界清晰,加新主题 / 新功能不破坏既有结构

---

## 2. 技术栈

| 层 | 选型 | 理由 |
|---|---|---|
| 桌面壳 | **Tauri 2.x** | 包小 (< 15MB),启动快,内存占用低 |
| 后端语言 | Rust | Tauri 原生 |
| 前端框架 | **Svelte 5 + TypeScript** | 编译期框架,体积/性能最佳;内置动画原语适合微交互需求;agent 维护无大坑 |
| 构建 | Vite | Svelte 标配 |
| 数据存储 | **SQLite** (rusqlite) | 统计查询友好,单文件可备份,数据可靠 |
| 包管理 | pnpm | 推荐,npm/yarn 也可 |
| 测试 | cargo test (Rust) + Vitest + Svelte Testing Library (前端) | 主流组合 |

---

## 3. 总体架构

```
┌─────────────────────────────────────────────┐
│  Tauri Shell  (Rust 进程)                   │
│  ├─ 系统通知 / 窗口控制 / 提示音播放         │
│  ├─ 全局快捷键 (开始/暂停)                   │
│  └─ SQLite 数据访问 (rusqlite)               │
├─────────────────────────────────────────────┤
│  Webview  (Svelte 5 + TypeScript)            │
│  ├─ 路由: 主界面 / 任务清单 / 统计 / 设置    │
│  ├─ 状态: $state 全局 store + persisted      │
│  ├─ 主题层: CSS 变量 + 主题切换器            │
│  └─ 动效层: Svelte transitions + 自定义指令  │
└─────────────────────────────────────────────┘
              ↕ Tauri IPC (invoke)
        ┌──────────────────────┐
        │  SQLite 单文件数据库  │
        │  ~/AppData/Local/    │
        │  pomodoro-study/     │
        │  data.db             │
        └──────────────────────┘
```

### 3.1 职责切分

- **Rust 层**: 与操作系统打交道的事(通知、声音、文件、快捷键、数据库读写)
- **Svelte 层**: 所有 UI、计时逻辑、用户交互
- **IPC**: 仅在需要 OS 能力或数据持久化时调用 Rust 命令

未来 agent 修 UI 永远只动前端;修系统集成只动 Rust。边界清晰。

---

## 4. 数据模型

### 4.1 SQLite Schema

```sql
-- 学习目标(长期)
CREATE TABLE goals (
  id          INTEGER PRIMARY KEY,
  title       TEXT NOT NULL,                 -- "考完 CET-6"
  description TEXT,
  color       TEXT,                          -- 标签色,默认 acid green
  status      TEXT NOT NULL DEFAULT 'active',-- active / archived / done
  created_at  INTEGER NOT NULL,              -- unix ms
  archived_at INTEGER
);

-- 任务(挂在目标下)
CREATE TABLE tasks (
  id              INTEGER PRIMARY KEY,
  goal_id         INTEGER REFERENCES goals(id) ON DELETE SET NULL,
  title           TEXT NOT NULL,             -- "背单词 list 1-30"
  estimated_pomos INTEGER DEFAULT 1,
  status          TEXT NOT NULL DEFAULT 'active', -- active / done / abandoned
  created_at      INTEGER NOT NULL,
  done_at         INTEGER,
  sort_order      INTEGER DEFAULT 0
);

-- 番茄记录(核心事实表,不可变历史)
CREATE TABLE pomodoros (
  id            INTEGER PRIMARY KEY,
  task_id       INTEGER REFERENCES tasks(id) ON DELETE SET NULL,
  goal_id       INTEGER REFERENCES goals(id) ON DELETE SET NULL,
  started_at    INTEGER NOT NULL,
  ended_at      INTEGER,
  planned_secs  INTEGER NOT NULL,
  actual_secs   INTEGER,
  status        TEXT NOT NULL,               -- completed / interrupted / abandoned
  date_local    TEXT NOT NULL                -- 'YYYY-MM-DD',统计用
);

-- 打断记录
CREATE TABLE interrupts (
  id          INTEGER PRIMARY KEY,
  pomodoro_id INTEGER NOT NULL REFERENCES pomodoros(id) ON DELETE CASCADE,
  reason      TEXT,                          -- 用户自定义文本
  occurred_at INTEGER NOT NULL
);

-- 设置(键值,值统一存 JSON 字符串)
CREATE TABLE settings (
  key   TEXT PRIMARY KEY,
  value TEXT NOT NULL
);

CREATE INDEX idx_pomos_date ON pomodoros(date_local);
CREATE INDEX idx_pomos_task ON pomodoros(task_id);
CREATE INDEX idx_pomos_goal ON pomodoros(goal_id);
CREATE INDEX idx_tasks_goal ON tasks(goal_id);
```

### 4.2 关键设计点

- **`pomodoros` 是事实表**,不可变。任务 / 目标改了不影响过去的统计。这就是为什么冗余 `goal_id`(任务可能改归属,但当时这个番茄属于哪个目标要锁住)。
- **`date_local`** 用字符串存当地日期 → 跨夜番茄归属当天,不受时区漂移影响。
- **`interrupts`** 单独表,一个番茄可有多次打断。
- **`settings`** 用 JSON 字符串值,加新配置不用改 schema。
- 删除目标 / 任务时用 `ON DELETE SET NULL`,保留历史番茄记录。

### 4.3 默认设置项

| key | 默认值 | 说明 |
|---|---|---|
| `timer.work_secs` | 1500 | 学习时长(秒),25min |
| `timer.break_secs` | 300 | 短休时长,5min |
| `timer.long_break_secs` | 900 | 长休时长,15min |
| `timer.long_break_every` | 4 | 每 N 个番茄触发长休 |
| `timer.auto_continue` | false | 番茄结束后是否自动开始下一个 |
| `notify.system` | true | 系统 toast 通知 |
| `notify.sound` | true | 提示音 |
| `notify.sound_file` | "ding.mp3" | 提示音文件 |
| `notify.fullscreen` | true | 结束全屏遮罩 |
| `notify.taskbar` | true | 任务栏闪烁 |
| `theme` | "acid" | 当前主题 id |

### 4.4 统计语义

- **当天统计**: `WHERE date_local = ?` (number of completed pomos, total focus seconds, interrupt count)
- **总计统计**: 所有 status='completed' 的累计
- 两套都做,在统计页可切换视图

---

## 5. UI 风格与主题切换框架

### 5.1 首发主题: 酸性艺术 (Acid)

**约束** (硬要求):

- 唯一强调色: **酸柠檬绿** `#c6ff00` (色值可调,但只有这一种强调色)
- 基础色板: 黑 `#0a0a0a` / 深灰 `#141414` / 灰 `#1f1f1f` / 白 `#ffffff`
- 需要新色块时用 `--color-accent` 的低透明版 `--color-accent-low` (不引入额外颜色)
- **禁止大面积渐变作为装饰**(内容性渐变除外,如长进度条)
- **所有阴影都是硬阴影** (box-shadow 的 blur-radius = 0)
- **圆角一律 0** (border-radius: 0)
- 排版: 粗黑无衬线 + 等宽字体辅助
- 风格关键词: 科技、专业、高端

**典型组件视觉**(摘自原型确认):

- 计时器: 黑底白字大数字,2px 白框,`box-shadow: 8px 8px 0 #c6ff00`
- 主按钮: 酸绿底黑字,字间距 3px,大写
- 进度条: 字符级 `▮▯` 而非平滑填充
- Hover 微交互: 硬阴影偏移变化(4px 4px → 8px 8px),无缓动模糊
- 任务勾选: 绿色方块从左下角"刷"过去
- 番茄结束: 数字快速闪烁酸绿,屏幕边缘出现硬边框扫描线
- Tab 切换: 整块硬切(无淡入淡出),酸绿边框瞬间出现

### 5.2 主题切换三层架构

```
┌──────────────────────────────────────────────┐
│  Layer 1: 设计 Token (CSS 变量)              │
│  --color-bg / --color-fg / --color-accent    │
│  --shadow-hard / --border-width / ...        │
├──────────────────────────────────────────────┤
│  Layer 2: 主题包 (themes/<id>/theme.css)     │
│  acid.css → 定义所有 token 的具体值          │
│  synthwave.css → 同上,不同值                 │
├──────────────────────────────────────────────┤
│  Layer 3: 组件 (只用 token,不写死颜色)       │
│  .button { background: var(--color-accent) } │
└──────────────────────────────────────────────┘
```

### 5.3 Token 列表

```ts
// src/themes/_tokens.ts
export const TOKEN_KEYS = [
  // 色彩
  'color-bg', 'color-bg-elevated', 'color-fg', 'color-fg-muted',
  'color-accent', 'color-accent-low',
  'color-border', 'color-success', 'color-danger',
  // 字体
  'font-display', 'font-body', 'font-mono',
  // 形状
  'radius', 'border-width',
  // 阴影
  'shadow-hard-sm',   // acid: '4px 4px 0 var(--color-accent)'
  'shadow-hard-md',   // acid: '8px 8px 0 var(--color-accent)'
  'shadow-hard-lg',
  // 动效
  'ease-snap',        // acid: steps(3, end)
  'ease-smooth',
  'duration-fast', 'duration-base'
] as const
```

### 5.4 切换 API

```ts
// src/themes/registry.ts
import { setTheme, getCurrentTheme, listThemes } from '$themes/registry'

setTheme('acid')        // 切换主题,持久化到 settings 表
listThemes()             // [{ id, displayName, preview, status }]
getCurrentTheme()        // 当前 id
```

切换原理: 在 `<html data-theme="acid">` 上挂属性,CSS 用 `[data-theme="acid"] { --color-bg: ... }` 作用域生效。切换时只换属性,无重渲染抖动。

### 5.5 给后续 agent 的硬约束

- 加新主题 = 复制 `themes/acid/` 文件夹,改 token 值,在 `registry.ts` 注册。**零业务代码改动**。
- 业务组件**禁止写死颜色** `#xxx` / 阴影 / 圆角等,必须用 `var(--token)`。
- 用 stylelint 规则强制约束(`color-no-hex` 在组件 CSS 中开启,主题 CSS 排除)。
- 这套规则写进 `docs/THEMING.md`。

### 5.6 预留主题位

`src/themes/synthwave/` 文件夹已存在,内含 `meta.ts`,标 `status: 'coming-soon'`。在主题切换页可见但禁用,点击提示「即将推出」。

---

## 6. UI 路由与核心交互

### 6.1 主导航(单窗口 + 左侧栏)

```
▮ FOCUS    主界面 (计时器 + 当前任务)
● TASKS    任务清单 (目标 + 任务管理)
▤ STATS    统计 (今日 / 总计 / 趋势)
◐ THEME    主题切换
⚙ SETTINGS 设置 (时长 / 通知 / 数据)
```

### 6.2 核心流程: 开始一个番茄

1. 用户在 TASKS 页选一个任务 → 点 `▶ FOCUS`,或在 FOCUS 页直接选下拉里的任务
2. 跳到 FOCUS 页,倒计时 25:00 开始
   - 计时由前端 `requestAnimationFrame` + `Date.now()` 驱动(不依赖 `setInterval`,避免后台节流误差)
3. 用户中途点 `[⊗ 打断]`:
   - 弹小窗输入打断原因(自定义文本)
   - `pomodoros.status = 'interrupted'`,`interrupts` 表插一条
   - 选项: `[继续(补完剩余)]` / `[放弃]` / `[重新开始]`
4. 自然结束(00:00):
   - 写 `pomodoros` 表(`status='completed'`, `actual_secs`)
   - 触发通知组合(toast + 提示音 + 自动激活窗口 + 任务栏闪烁,按设置开关)
   - 全屏酸绿动画 "FOCUS COMPLETE"
   - 自动进入休息态
5. 休息倒计时:
   - 第 4/8/12... 个番茄后,休息切换到长休 15:00
   - 休息结束 → 提示返回学习 → 用户确认或自动进入下一番茄(取决于 `timer.auto_continue`)

### 6.3 任务清单页

- 目标卡片 + 卡片下展开任务列表
- 每个任务行格式: `☐ 标题  ▮▮▮▯ 3/4 番茄  [⋯]`
- 操作: 拖拽排序(`sort_order`)、勾选完成、右键菜单(编辑/删除/归档)
- 任务标记完成 = 写 `done_at`,从「active 列表」消失,但统计仍可见
- 「无目标任务」也允许(`goal_id IS NULL`)

### 6.4 统计页

- 顶部三卡: 今日番茄数 / 今日学习时长 / 连续学习天数
- 中间: 本周柱状图(字符级 ▮ 风格)
- 底部: 按目标聚合的堆叠条 + 打断原因 Top 5
- 时间维度切换: 今天 / 本周 / 本月 / 全部

### 6.5 设置页

- 时长设置(学习 / 短休 / 长休 / 长休周期)
- 通知开关(系统通知 / 提示音 / 全屏遮罩 / 任务栏闪烁,各自独立)
- 提示音文件选择
- 主题切换入口
- 数据: 导出 JSON / 导入 JSON / 清空所有数据(危险操作,需要二次确认)

---

## 7. 项目结构

```
pomodoro-study/
├── README.md                    # 快速上手(开发/构建/打包)
├── package.json
├── pnpm-lock.yaml
├── tsconfig.json
├── svelte.config.js
├── vite.config.ts
├── .gitignore
│
├── docs/                        # 给后续 agent 看的文档
│   ├── ARCHITECTURE.md
│   ├── DATA_MODEL.md
│   ├── THEMING.md
│   ├── IPC_API.md
│   ├── AGENT_GUIDE.md
│   └── superpowers/
│       └── specs/
│           └── 2026-06-17-pomodoro-study-design.md  # 本文档
│
├── src-tauri/                   # Rust 端
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   ├── src/
│   │   ├── main.rs
│   │   ├── commands/            # IPC 命令(分文件)
│   │   │   ├── mod.rs
│   │   │   ├── pomodoros.rs
│   │   │   ├── tasks.rs
│   │   │   ├── goals.rs
│   │   │   ├── stats.rs
│   │   │   ├── settings.rs
│   │   │   └── notify.rs
│   │   ├── db/
│   │   │   ├── mod.rs
│   │   │   ├── migrations/
│   │   │   └── queries.rs
│   │   └── notifier.rs
│   └── icons/
│
├── src/                         # Svelte 前端
│   ├── app.html
│   ├── main.ts
│   ├── App.svelte
│   ├── lib/
│   │   ├── ipc.ts
│   │   ├── time.ts              # 计时器核心逻辑
│   │   ├── stores/
│   │   │   ├── timer.svelte.ts
│   │   │   ├── tasks.svelte.ts
│   │   │   ├── goals.svelte.ts
│   │   │   └── settings.svelte.ts
│   │   └── components/
│   │       ├── Sidebar.svelte
│   │       ├── TimerDisplay.svelte
│   │       ├── TaskRow.svelte
│   │       ├── GoalCard.svelte
│   │       ├── InterruptDialog.svelte
│   │       ├── CompleteOverlay.svelte
│   │       └── ProgressBar.svelte
│   ├── routes/
│   │   ├── focus/+page.svelte
│   │   ├── tasks/+page.svelte
│   │   ├── stats/+page.svelte
│   │   ├── theme/+page.svelte
│   │   └── settings/+page.svelte
│   ├── themes/
│   │   ├── _tokens.ts
│   │   ├── _base.css
│   │   ├── registry.ts
│   │   ├── acid/
│   │   │   ├── theme.css
│   │   │   ├── animations.css
│   │   │   └── meta.ts
│   │   └── synthwave/
│   │       └── meta.ts          # 占位
│   └── styles/
│       └── reset.css
│
└── tests/
    ├── rust/                    # cargo test
    └── frontend/                # Vitest + Svelte Testing Library
```

---

## 8. 文档清单(给后续 agent)

| 文档 | 内容 |
|---|---|
| `README.md` | 一页:项目简介 / `pnpm install` / `pnpm tauri dev` / 打包命令 |
| `docs/ARCHITECTURE.md` | 架构图、Rust/前端职责边界、IPC 模式、为什么这么切 |
| `docs/DATA_MODEL.md` | 每张表每个字段的语义、为什么冗余 `goal_id`、迁移规范 |
| `docs/THEMING.md` | Token 全列表、加新主题 5 步流程、CSS 变量约束、stylelint 规则 |
| `docs/IPC_API.md` | 每个 Rust 命令: 参数 / 返回 / 调用示例 / 错误码 |
| `docs/AGENT_GUIDE.md` | **关键** 命名约定、不许改的边界、常见维护任务模板:<br>· 加一个新主题<br>· 加一个新设置项<br>· 加一种统计图<br>· 加一种打断原因预设<br>· 数据库迁移流程 |

---

## 9. 测试策略

务实路线,不追求 100% 覆盖:

- **Rust 端**: 数据库查询 + 时间逻辑做单元测试 (`cargo test`),目标 >70% 覆盖
- **前端**:
  - 计时器状态机用 Vitest 单测(开始/暂停/恢复/打断/完成的状态转移必须全覆盖)
  - 关键组件用 Svelte Testing Library 烟雾测试(任务行、统计卡片渲染)
  - 不写端到端测试,番茄钟没那么复杂
- **手工验收清单** 写在 `docs/AGENT_GUIDE.md`,发版前过一遍

---

## 10. 构建与发布

- 开发: `pnpm tauri dev`
- 打包: `pnpm tauri build` → 产出 `.msi` Windows 安装包
- 体积预期: 8-15 MB
- 数据位置: `%LOCALAPPDATA%\pomodoro-study\data.db`
- 无网络请求 = 无隐私问题 = 不需要 ToS / Privacy 文档

---

## 11. 关键决策回顾(给后续 agent 看的"为什么")

| 决策 | 为什么 |
|---|---|
| Tauri 而不是 Electron | 包小、内存占用低,匹配「专业、高端」定位;番茄钟轻量工具不需要 Electron 的重量 |
| Svelte 5 而不是 React/Vue | 编译期框架体积最小;内置动画原语适合「精致微交互」需求 |
| SQLite 而不是 JSON 文件 | 统计查询(GROUP BY 日期 / 目标聚合)用 SQL 一句话,JSON 要在内存里折腾 |
| 两层结构(目标/任务)而不是三层 | 三层是项目管理工具的复杂度,番茄钟不需要;两层既有长期方向感又不过重 |
| 主动打断按钮而不是窗口失焦自动算 | 学习要查资料就会切窗,自动判断误伤多;主动按按钮 = 主动反思 |
| 全部都做硬约束(直角/硬阴影/单一强调色) | 用户明确要求,避免「AI 套路」感;同时也是主题包的设计契约 |
| 番茄钟事实表不可变 | 任务/目标可能被改名甚至删除,但历史统计不能跟着变,否则用户信任崩塌 |

---

## 12. 后续工作

本设计文档批准后,进入实现计划(plan)阶段:

1. 由 `writing-plans` skill 产出分阶段实现计划
2. 计划应至少包含: 项目初始化 / DB 层 / IPC 层 / 主题框架 / FOCUS 页 / TASKS 页 / STATS 页 / SETTINGS 页 / 通知系统 / 文档撰写
3. 每个阶段完成后增量验证,不一次性合并
