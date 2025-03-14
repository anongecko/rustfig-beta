# RustFig Installation Guide

This guide provides comprehensive instructions for installing RustFig on macOS (including Apple Silicon M-series) and Linux systems.

## Installation Overview

RustFig consists of two components:

1. **Command-line tool**: The `rustfig` binary that provides the interface and commands
2. **Background service**: A daemon process that handles predictions and AI interactions

Both components are installed together and work seamlessly.

## Prerequisites

- Rust toolchain 1.70.0 or newer (if installing from source)
- A modern terminal with 24-bit color support:
  - iTerm2, Kitty, WezTerm, Alacritty, Terminal.app (macOS)
  - GNOME Terminal, Konsole, Terminator, Alacritty (Linux)
- For local AI features: [Ollama](https://ollama.ai/) (optional)

## Installation Methods

### Method 1: One-Command Installation Script (Recommended)

```bash
# For macOS and Linux
curl -sSL https://get.rustfig.dev | bash

# Or with a specific shell
curl -sSL https://get.rustfig.dev | bash -s -- --shell zsh
```

This will:
1. Download and install the latest RustFig binary
2. Set up configuration files with sensible defaults
3. Add shell integration to your shell configuration
4. Start the background service

### Method 2: Using Cargo

```bash
# Install from crates.io
cargo install rustfig

# Set up RustFig
rustfig setup
```

### Method 3: Using Package Managers

#### macOS (Homebrew)

```bash
# Install Homebrew if needed
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install RustFig
brew install rustfig/tap/rustfig

# Set up RustFig
rustfig setup
```

#### Arch Linux

```bash
# Using yay
yay -S rustfig

# Set up RustFig
rustfig setup
```

#### Ubuntu/Debian

```bash
# Add repository
curl -sSL https://get.rustfig.dev/gpg.key | sudo apt-key add -
echo "deb [arch=amd64] https://pkg.rustfig.dev/apt stable main" | sudo tee /etc/apt/sources.list.d/rustfig.list

# Install
sudo apt update
sudo apt install rustfig

# Set up RustFig
rustfig setup
```

### Method 4: Build from Source

```bash
# Clone the repository
git clone https://github.com/rustfig/rustfig.git
cd rustfig

# Build with optimizations
RUSTFLAGS="-C target-cpu=native" cargo build --release

# Install locally
cargo install --path .

# Set up RustFig
./target/release/rustfig setup
```

## Platform-Specific Optimizations

### Apple Silicon (M-series) Macs

When building from source on Apple Silicon, we automatically enable optimizations for M-series chips:

- ARM-specific SIMD instructions
- Apple's Accelerate framework
- Memory optimizations for unified memory architecture

No additional steps are required - these optimizations are automatically applied.

### Linux Systems

On Linux, RustFig automatically detects your CPU capabilities and optimizes accordingly. For maximum performance:

```bash
# Build with target-specific optimizations
RUSTFLAGS="-C target-cpu=native" cargo build --release
```

## Manual Shell Integration

If you prefer to manually integrate RustFig with your shell:

### Bash Integration

Add to your `~/.bashrc` or `~/.bash_profile`:

```bash
# RustFig integration START
export RUSTFIG_ENABLED=1
export RUSTFIG_SHELL="bash"
export RUSTFIG_VERSION="$(rustfig --version 2>/dev/null | cut -d ' ' -f 2)"

if [ -n "$RUSTFIG_VERSION" ]; then
  # Load core functionality
  source "$(rustfig path scripts/bash/core.sh)"
  
  # Initialize prediction service
  rustfig service start >/dev/null 2>&1
  
  # Set up key bindings
  bind -x '"\C-g": "rustfig toggle-ghost"'
  bind -x '"\C-x\C-e": "rustfig explain-command"'
  
  # Hook into command execution
  eval "$(rustfig hook-install bash)"
fi
# RustFig integration END
```

### Zsh Integration

Add to your `~/.zshrc`:

```zsh
# RustFig integration START
export RUSTFIG_ENABLED=1
export RUSTFIG_SHELL="zsh"
export RUSTFIG_VERSION="$(rustfig --version 2>/dev/null | cut -d ' ' -f 2)"

if [ -n "$RUSTFIG_VERSION" ]; then
  # Load core functionality
  source "$(rustfig path scripts/zsh/core.zsh)"
  
  # Initialize prediction service
  rustfig service start >/dev/null 2>&1
  
  # Set up key bindings
  bindkey '^G' rustfig-toggle-ghost
  bindkey '^X^E' rustfig-explain-command
  
  # Hook into ZLE (Zsh Line Editor)
  zle -N rustfig-predict
  bindkey '^I' rustfig-predict
  
  # Add command hooks
  autoload -Uz add-zsh-hook
  add-zsh-hook preexec rustfig-preexec
  add-zsh-hook precmd rustfig-precmd
fi
# RustFig integration END
```

### Fish Integration

Add to your `~/.config/fish/config.fish`:

```fish
# RustFig integration START
set -gx RUSTFIG_ENABLED 1
set -gx RUSTFIG_SHELL "fish"
set -gx RUSTFIG_VERSION (rustfig --version 2>/dev/null | string split ' ')[2]

if test -n "$RUSTFIG_VERSION"
  # Load core functionality
  source (rustfig path scripts/fish/core.fish)
  
  # Initialize prediction service
  rustfig service start >/dev/null 2>&1
  
  # Set up key bindings
  bind \cg 'rustfig toggle-ghost'
  bind \cx\ce 'rustfig explain-command'
  
  # Hook into fish command execution
  function __rustfig_hook --on-event fish_prompt
    rustfig update-context
  end
end
# RustFig integration END
```

## Enabling Ollama (Local LLM) Support

1. Install Ollama from [ollama.ai](https://ollama.ai/)

2. Pull a supported model:

```bash
# For terminal-focused assistance
ollama pull codellama:7b-instruct

# Alternative lightweight model
ollama pull llama3:8b
```

3. Ensure Ollama is running:

```bash
# Start Ollama service
ollama serve
```

4. Configure RustFig to use Ollama:

```bash
# Enable Ollama integration
rustfig config set ollama.enabled true
rustfig config set ollama.model "codellama:7b-instruct"
```

## SSH Support

RustFig works seamlessly over SSH connections without special configuration. When connected to a remote server via SSH:

1. **On the local machine**: RustFig detects the SSH session and optimizes behavior
2. **On the remote server**: Install RustFig for full functionality

For optimal experience on remote servers, install RustFig using:

```bash
# On the remote server
curl -sSL https://get.rustfig.dev/remote | bash
```

This script optimizes RustFig for SSH sessions with:
- Reduced bandwidth usage
- Minimized UI animations
- Enhanced caching
- SSH-specific optimizations

## Verifying Installation

To verify that RustFig is installed and working correctly:

```bash
# Check version
rustfig --version

# Run diagnostics
rustfig doctor

# Check service status
rustfig service status
```

If everything is working correctly, you should see the ghost text and dropdown suggestions in your terminal as you type.

## Configuration

RustFig is highly configurable. The configuration files are located at:

- `~/.config/rustfig/config.yaml` - Main configuration
- `~/.config/rustfig/keybindings.yaml` - Keyboard shortcuts
- `~/.config/rustfig/appearance.yaml` - UI appearance and themes
- `~/.config/rustfig/ai_models.yaml` - AI models configuration

You can edit these files directly or use the `rustfig config` commands. For example:

```bash
# Edit configuration in your preferred editor
rustfig config edit

# Change theme
rustfig theme set dracula

# Enable ghost text
rustfig config set general.enable_ghost_text true
```

## Uninstallation

To uninstall RustFig:

```bash
# Remove shell integration
rustfig uninstall

# Optionally, remove all data and configuration
rm -rf ~/.config/rustfig
```

Then uninstall the binary using your package manager or:

```bash
cargo uninstall rustfig
```

## Troubleshooting

See [TROUBLESHOOTING.md](TROUBLESHOOTING.md) for common issues and solutions.

For more help, visit [https://rustfig.dev/support](https://rustfig.dev/support) or run:

```bash
rustfig help
rustfig doctor --verbose
```
