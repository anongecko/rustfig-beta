# RustFig: Ultra-Fast Terminal Assistant

<p align="center">
  <img src="assets/rustfig-logo.png" alt="RustFig Logo" width="180"/>
</p>

<p align="center">
  <a href="https://github.com/rustfig/rustfig/releases"><img src="https://img.shields.io/github/v/release/rustfig/rustfig" alt="Latest Release"></a>
  <a href="https://github.com/rustfig/rustfig/blob/master/LICENSE"><img src="https://img.shields.io/github/license/rustfig/rustfig" alt="License"></a>
  <a href="https://github.com/rustfig/rustfig/actions"><img src="https://github.com/rustfig/rustfig/workflows/CI/badge.svg" alt="Build Status"></a>
  <a href="https://crates.io/crates/rustfig"><img src="https://img.shields.io/crates/v/rustfig.svg" alt="Crates.io"></a>
  <a href="https://docs.rs/rustfig"><img src="https://docs.rs/rustfig/badge.svg" alt="Documentation"></a>
</p>

**RustFig** is a blazingly fast, context-aware terminal assistant built entirely in Rust. It brings modern IDE-like features to your terminal with **zero latency** and **complete privacy**. Think of Amazon Q or Warp terminal suggestions, but open-source, lightweight, and running entirely on your machine.

<p align="center">
  <img src="assets/rustfig-demo.gif" alt="RustFig Demo" width="600"/>
</p>

## ✨ Key Features

### 🚀 Ultra-Fast Suggestion Dropdowns
- **Context-aware suggestions** appear in a sleek dropdown interface as you type
- **Sub-5ms response time** guaranteed on all operations (even on modest hardware)
- **Zero latency** ghost text suggestions that feel like magic
- **Rich visualizations** with file/command icons, syntax highlighting, and more

### 🔒 100% Privacy-Focused
- **No telemetry or data collection** - your commands stay on your machine
- **No network requests** for core functionality
- **Optional AI** features are explicit and configurable
- **Self-host everything** including AI capabilities
- **Open source** codebase you can audit

### 🧠 Terminal Intelligence
- **Project-aware** suggestions based on context (Git, Docker, language-specific)
- **Learns from your usage patterns** for personalized suggestions
- **Predictive command intelligence** offers commands before you type them
- **Shell syntax awareness** for bash, zsh, and fish
- **Path completion** with file type detection and previews
- **Command flag suggestions** with documentation

### 💬 AI Terminal Assistant
- **Chat with AI directly** in your terminal (`rustfig ask "How do I..."`)
- **Support for multiple AI providers**:
  - OpenAI (including Azure OpenAI)
  - Anthropic Claude models
  - Local models via Ollama
  - Any OpenAI-compatible API
- **Conversation memory** for contextual discussions
- **Command explanations** for any terminal command
- **Error analysis** to fix broken commands

### 🎨 Highly Customizable
- **Extensive configuration options** via YAML files
- **Growing theme collection** for terminal integration
- **Fully customizable keybindings**
- **Terminal-specific optimizations**
- **Configurable UI elements** (icons, animations, colors)

### ⚡️ Lightweight & Efficient
- **Minimal resource usage** (~15-30MB RAM)
- **No background processes** when idle
- **Optimized for Apple Silicon** and modern CPUs
- **Works over SSH** connections seamlessly
- **Fast startup time** (<100ms)

### 🔌 Modern Integration
- **Works with all major terminals** and shells
- **SSH-aware** for remote connections
- **Plugin system** for extensibility
- **Cross-platform** support (macOS, Linux, WSL)

## 📦 Installation

### One-Line Install (macOS & Linux)

```bash
curl -sSL https://get.rustfig.dev | bash
```

### Using Cargo

```bash
cargo install rustfig
rustfig setup
```

### Package Managers

**macOS (Homebrew):**
```bash
brew install rustfig/tap/rustfig
```

**Arch Linux:**
```bash
yay -S rustfig
```

**Ubuntu/Debian:**
```bash
curl -sSL https://get.rustfig.dev/gpg.key | sudo apt-key add -
echo "deb [arch=amd64] https://pkg.rustfig.dev/apt stable main" | sudo tee /etc/apt/sources.list.d/rustfig.list
sudo apt update && sudo apt install rustfig
```

### Build from Source

```bash
git clone https://github.com/rustfig/rustfig.git
cd rustfig
cargo build --release
./target/release/rustfig setup
```

For more detailed instructions, see [INSTALL.md](INSTALL.md).

## 🖥️ Usage Examples

### Basic Commands

```bash
# Display help
rustfig help

# Check system status and fix issues
rustfig doctor --fix

# Edit configuration
rustfig config edit

# Set a specific configuration option
rustfig config set ui.theme "nord"

# List available themes
rustfig theme list

# Apply a theme
rustfig theme set dracula
```

### AI Features

```bash
# Ask a quick question
rustfig ask "How do I find large files in Linux?"

# Start interactive chat session
rustfig chat

# Explain a command
rustfig explain "awk '{print $1}' file.txt | sort | uniq -c"

# Generate a command from description
rustfig generate "create a backup of my home directory excluding node_modules"
```

### Service Management

```bash
# Check service status
rustfig service status

# Restart the service
rustfig service restart

# View service logs
rustfig service logs --follow
```

## ⚙️ Configuration

RustFig is highly configurable. Configuration files are located at:

```
~/.config/rustfig/
├── config.yaml            # Main configuration
├── keybindings.yaml       # Keyboard shortcuts
├── appearance.yaml        # UI themes and appearance
├── ai_models.yaml         # AI provider configuration
└── themes/                # Custom themes
```

### Theme Customization

RustFig includes multiple built-in themes:
- `default` - Clean, minimalist theme
- `dark` - High-contrast dark theme
- `light` - Bright theme for light terminals
- `nord` - Popular Nord color scheme
- `dracula` - Famous Dracula color scheme
- `monokai` - Vibrant Monokai inspired
- `solarized` - Both light and dark Solarized variants

Switch themes with:
```bash
rustfig theme set nord
```

Or create your own:
```bash
rustfig theme create mytheme --base=dracula
rustfig theme edit mytheme
```

### Keybinding Customization

Customize keybindings in `~/.config/rustfig/keybindings.yaml` or with:

```bash
# List all keybindings
rustfig keybindings list

# Change a keybinding
rustfig keybindings set AcceptGhost "Alt+Right"
```

### AI Configuration

Configure AI providers in `~/.config/rustfig/ai_models.yaml` or with:

```bash
# Set up OpenAI
rustfig config set ai.api_key "your-api-key"
rustfig config set ai.model "gpt-4o"

# Or use Ollama locally
rustfig config set ollama.enabled true
rustfig config set ollama.model "codellama:7b-instruct"

# Or Azure OpenAI
rustfig config set ai.api_endpoint "https://your-resource.openai.azure.com/openai/deployments/your-deployment/v1"
rustfig config set ai.api_key "your-azure-api-key"
```

## 🔍 How It Works

RustFig combines multiple techniques to provide intelligent assistance:

1. **Context Analysis**: Analyzes your current directory, git status, and environment
2. **Command Parsing**: Understands shell syntax and command structure
3. **Predictive Engine**: Predicts what you might type next based on patterns
4. **Local Learning**: Learns from your command history and preferences
5. **Terminal Integration**: Seamlessly integrates with your shell's command line

Everything happens locally on your machine with minimal resource usage.

## 🥊 Comparison with Alternatives

| Feature | RustFig | Amazon Q | Warp | Fig |
|---------|---------|----------|------|-----|
| **Privacy** | ✅ 100% local | ❌ Sends data to AWS | ❌ Cloud-based | ❌ Cloud-dependent |
| **Performance** | ✅ <5ms latency | ⚠️ Variable | ⚠️ Electron-based | ⚠️ High resource usage |
| **Resource Usage** | ✅ ~15-30MB RAM | ❌ 200MB+ | ❌ 500MB+ | ❌ 200MB+ |
| **Open Source** | ✅ Fully open | ❌ Closed | ❌ Closed | ✅ Partially |
| **Self-hosting** | ✅ Full support | ❌ No | ❌ No | ❌ Limited |
| **SSH Support** | ✅ Native | ⚠️ Limited | ❌ No | ⚠️ Limited |
| **Customization** | ✅ Extensive | ⚠️ Limited | ⚠️ Limited | ⚠️ Moderate |
| **Shells** | ✅ Bash, Zsh, Fish | ✅ Multiple | ❌ Custom shell | ✅ Multiple |
| **Works Offline** | ✅ Full functionality | ❌ Limited | ❌ No | ❌ Limited |

## 🛠️ Advanced Features

### SSH Integration
RustFig maintains full functionality over SSH connections, automatically optimizing for remote connections with reduced bandwidth usage and enhanced performance.

### Command Chains
Intelligently suggests piped commands and complex command chains based on the expected output of the current command.

### Error Prevention
Proactively detects potential errors in commands before execution and offers corrections.

### Snippets System
Save and recall common command patterns with the snippets system:

```bash
# Save the current command as a snippet
rustfig snippets add find-large "find . -type f -size +100M"

# List available snippets
rustfig snippets list

# Use a snippet
rustfig snippets use find-large
```

### Project Detection
Automatically detects project types (Node.js, Rust, Python, etc.) and offers contextual suggestions.

### Custom Completers
Extend RustFig with custom completers for specialized tools and workflows.

## 🤝 Contributing

Contributions are welcome! Check out [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## 📃 License

RustFig is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

RustFig draws inspiration from:
- The terminal completion of Fig
- The suggestions UI of Amazon Q
- The AI capabilities of GitHub Copilot
- The performance focus of ripgrep and other Rust CLI tools

## 🔮 Roadmap

- [ ] Command chaining and multi-step suggestions
- [ ] Advanced visualization capabilities
- [ ] Dashboard UI for configuration
- [ ] Enhanced plugin ecosystem
- [ ] Additional AI model integrations

---

<p align="center">
  <sub>Built with ❤️ by the RustFig team</sub>
</p>

<p align="center">
  <a href="https://rustfig.dev">Website</a> •
  <a href="https://rustfig.dev/docs">Documentation</a> •
  <a href="https://discord.gg/rustfig">Discord</a> •
  <a href="https://twitter.com/rustfig">Twitter</a>
</p>
