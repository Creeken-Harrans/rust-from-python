# 安装 Rust

## 推荐方式：rustup

[rustup](https://rustup.rs/) 是 Rust 官方推荐的安装和管理工具，类似 Python 的 `pyenv` + `pip` 合体。

### Linux / macOS

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

安装完成后，重启终端或执行：

```bash
source "$HOME/.cargo/env"
```

### Windows

下载并运行 [rustup-init.exe](https://win.rustup.rs/)。

### 验证安装

```bash
rustc --version   # 应显示：rustc 1.xx.x (xxxxxxx 20xx-xx-xx)
cargo --version   # 应显示：cargo 1.xx.x (xxxxxxx 20xx-xx-xx)
```

## 更新 Rust

```bash
rustup update
```

## 卸载 Rust

```bash
rustup self uninstall
```

## IDE / 编辑器推荐

- **VS Code** + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) 扩展
- **JetBrains CLion / IntelliJ** + Rust 插件
- **Vim / Neovim** + rust-analyzer LSP

## 国内镜像（可选）

如果下载缓慢，可以配置国内镜像源（如中科大或清华源），详见 [Rustup 镜像帮助](https://mirrors.tuna.tsinghua.edu.cn/help/rustup/)。

## 下一步

安装完成后，回到 [README.md](README.md) 继续教程。
