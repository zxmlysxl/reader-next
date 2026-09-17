# Reader Next

面向自用与二次开发的阅读 3.0 服务端。Rust 后端负责书源解析、账号、缓存和 AI 调用，Vue 前端负责书架、阅读器、本地书籍和阅读辅助。

> **📖 Reader Next** — 基于 [reader-rust](https://github.com/givenge/reader-rust) 持续维护，重点发展 AI 智能功能（章节摘要、AI 资料库、AI 地图、关系图谱等）。

## 功能特性

- **书源**：搜索、详情、目录、正文解析，多规则解析（CSS Selector、JSONPath、XPath、Regex、JavaScript）
- **书架**：服务端书架、分组、最近阅读、阅读进度同步
- **AI 辅助**：章节摘要、要点列表、AI 资料库（角色/关系/地图）、AI 追赶
- **本地书籍**：支持 TXT、EPUB、MOBI、PDF，跨设备阅读
- **WebDAV 备份**：书架、书源、阅读进度备份与恢复
- **其他**：RSS、TTS、缓存管理、用户和权限管理

## 快速部署

### Docker 镜像（推荐）

镜像已推送至 Docker Hub：

```
zuoxm/reader-next:latest        # 最新稳定版
zuoxm/reader-next:v1.0.27      # 固定版本
```

启动服务：

```bash
docker run -d \
  --name reader-next \
  --restart unless-stopped \
  -p 18092:18080 \
  -v reader-next-storage:/app/storage \
  -e SERVER_PORT=18080 \
  -e SERVER_HOST=0.0.0.0 \
  -e SECURE=false \
  -e SECURE_KEY=your_secret_key \
  -e INVITE_CODE=zuoxm2026 \
  zuoxm/reader-next:latest
```

打开浏览器访问：

```
http://10.3.4.17:18092
```

> 首次访问需要输入邀请码 `zuoxm2026`（如已配置则无需输入）。

### 升级

```bash
docker pull zuoxm/reader-next:latest
docker stop reader-next
docker rm reader-next
# 重新运行上面的 docker run 命令（storage volume 会保留数据）
```

> **数据安全**：SQLite 数据库、上传文件和缓存都在 `/app/storage`，务必挂载 volume 或宿主机目录，删容器不会丢数据。

### Compose 部署

```bash
# 下载 docker-compose.yml
# 修改 SECURE_KEY、INVITE_CODE 等配置后
docker compose up -d
```

## 配置说明

配置从 `.env` 或环境变量读取。完整配置见 `.env.example`。

| 配置 | 默认值 | 说明 |
| --- | --- | --- |
| `SERVER_HOST` | `0.0.0.0` | 服务监听地址 |
| `SERVER_PORT` | `18080` | 服务端口 |
| `DATABASE_URL` | `sqlite:storage/reader.db?mode=rwc` | SQLite 数据库路径 |
| `WEB_ROOT` | `frontend/dist` | 前端静态资源目录 |
| `SECURE` | `false` | 安全模式（关闭则跳过认证） |
| `SECURE_KEY` | 空 | 安全模式密钥 |
| `INVITE_CODE` | 空 | 注册邀请码 |
| `USER_LIMIT` | `50` | 最大用户数 |
| `USER_BOOK_LIMIT` | `2000` | 每用户最大书籍数 |
| `LOG_LEVEL` | `info` | 日志级别 |
| `REQUEST_TIMEOUT_SECS` | `15` | HTTP 请求超时（秒） |

## 本地开发

```bash
# 构建前端
cd frontend && npm install && npm run build && cd ..

# 启动后端
SERVER_PORT=18080 cargo run
# 访问 http://localhost:18080

# 前端热更新开发（需要单独起后端）
cd frontend && npm run dev
# Vite 监听 5173，把 /reader3 代理到后端
```

## 项目结构

```
src/api/       HTTP handlers 和 /reader3 路由
src/app/       应用启动、配置加载
src/crawler/   HTTP 客户端
src/model/     数据模型和 DTO
src/parser/    CSS / JSONPath / XPath / JS / Regex 规则解析
src/service/   书源、书架、AI、章节摘要、本地书籍等业务逻辑
src/storage/   SQLite、文件缓存和上传资源
src/util/      通用工具函数
frontend/      Vue 3 + Vite + Pinia 前端
```

## 相关项目

- 原始项目：[hectorqin/reader](https://github.com/hectorqin/reader)
- 基础项目：[givenge/reader-rust](https://github.com/givenge/reader-rust)
- 本仓库：[zxmlysxl/reader-next](https://github.com/zxmlysxl/reader-next)
