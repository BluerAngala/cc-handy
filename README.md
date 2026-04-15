# cc-handy (二次开发版)

本项目基于 [cjpais/Handy](https://github.com/cjpais/Handy) 项目进行二次开发。

cc-handy 是一款跨平台的桌面级语音转文本（Speech-to-Text）应用程序，基于 Tauri 构建，采用 Rust 处理高性能后端计算，React/TypeScript 构建现代化前端界面。

## ✨ 主要功能

- **本地离线语音识别**：内置 Whisper 模型，支持 GPU 加速推理，所有录音和处理均在本地完成，确保隐私安全且无需网络。
- **全局快捷键与对讲机模式**：支持全局快捷键唤醒录音，并提供类似对讲机的“按住说话”（Push-to-Talk）功能。
- **智能静音检测 (VAD)**：集成 Silero VAD 进行精确的语音活动检测，有效过滤空白噪音，提升识别速度。
- **无缝光标输入**：识别完成后，自动将转换出的文本粘贴到当前系统活跃应用程序的光标位置。
- **多语言与翻译支持**：支持多语言的语音输入，并内置实时翻译选项。
- **系统托盘运行**：开机自启并默认最小化到系统托盘，保持单实例后台运行，随时待命。
- **🚀 二次开发新增特性**：
  - **HuggingFace 模型市场集成**：内置模型市场，支持浏览、搜索并直接下载 HuggingFace 上的 Whisper 开源模型。
  - **模型趋势数据展示**：直观展示各类语音模型的流行趋势数据，方便用户选择最适合的推理模型。

## 🛠️ 技术栈与核心组件

- **前端架构**：React 18, TypeScript, Vite, TailwindCSS, Zustand 状态管理
- **后端架构**：Tauri (Rust)
- **底层驱动核心**：
  - `whisper-rs`：基于 Whisper 模型的本地硬件加速推理引擎（macOS Metal / Windows Vulkan / Linux OpenBLAS）。
  - `cpal`：实现跨平台的低延迟音频输入/输出捕捉。
  - `vad-rs`：Silero VAD 语音活动检测。
  - `rdev`：系统级全局键盘快捷键监听。
  - `rubato` / `rodio`：音频重采样处理与系统交互反馈音效。

## 🚀 开发指南

### 前置要求

- [Rust](https://rustup.rs/) (最新稳定版)
- [Bun](https://bun.sh/) 包管理器

### 快速开始

1. **安装项目依赖**

   ```bash
   bun install
   ```

2. **准备基础 VAD 模型（开发环境必须）**

   ```bash
   mkdir -p src-tauri/resources/models
   curl -o src-tauri/resources/models/silero_vad_v4.onnx https://hf-mirror.com/wongeuler/silero-vad/resolve/main/silero_vad_v4.onnx
   ```

3. **启动开发服务器**

   ```bash
   bun run tauri dev

   # 如果在 macOS 上遇到 cmake 错误，请使用以下命令：
   # CMAKE_POLICY_VERSION_MINIMUM=3.5 bun run tauri dev
   ```

4. **构建生产版本**
   ```bash
   bun run tauri build
   ```

### 独立前端开发

如果你只希望修改界面 UI，可以单独运行前端构建流：

```bash
bun run dev        # 启动 Vite 开发服务器
bun run build      # 仅构建前端
bun run preview    # 预览构建的前端产物
```

## 🏗️ 架构概览

cc-handy 核心采用了 **Manager Pattern（管理器模式）** 和 **Pipeline Processing（流水线处理）**：

1. **音频捕获 (Audio Manager)**：监听系统麦克风。
2. **语音活动检测 (VAD Pipeline)**：Silero VAD 识别有效人声片段。
3. **模型推理 (Model & Transcription Manager)**：将音频流送入本地 Whisper 模型进行文本转录。
4. **Tauri 消息总线 (Commands/Events)**：识别结果通过 Tauri Event 机制返回给 React 前端。
5. **剪贴板注入**：后端自动模拟键盘/剪贴板操作，将结果上屏至任意活跃窗口。

---

_本项目采用 MIT 协议开源。_
