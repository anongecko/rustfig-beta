# RustFig CLI Command Structure

RustFig provides a comprehensive set of commands for installation, configuration, and interaction.
All commands follow the pattern: `rustfig [command] [subcommand] [options]`

## Primary Commands

### Installation and Setup

```
# Display version information
rustfig version

# Run initial setup
rustfig setup [--minimal] [--verbose]

# Generate shell integration code (for .bashrc, .zshrc, etc.)
rustfig init [bash|zsh|fish] [--minimal] [--full]

# Install integration into shell configuration
rustfig install [--shell=bash|zsh|fish] [--force]

# Uninstall from shell configuration
rustfig uninstall [--shell=bash|zsh|fish]

# Run system checks to diagnose issues
rustfig doctor [--fix] [--verbose]
```

### Service Management

```
# Start prediction service
rustfig service start [--verbose]

# Stop prediction service
rustfig service stop [--force]

# Restart prediction service
rustfig service restart

# Check service status
rustfig service status [--verbose]

# View service logs
rustfig service logs [--lines=N] [--follow]
```

### Configuration

```
# Edit main configuration file
rustfig config edit [--editor=vim|nano|code]

# Generate default configuration
rustfig config generate [--output=PATH]

# Get a specific configuration value
rustfig config get SETTING [--format=yaml|json|text]

# Set a configuration value
rustfig config set SETTING VALUE

# Reset configuration to defaults
rustfig config reset [--keep-credentials]

# Validate configuration file
rustfig config validate [--path=PATH]

# List all configuration options
rustfig config list [--format=yaml|json|text] [--verbose]

# Import configuration from file
rustfig config import PATH

# Export configuration to file
rustfig config export PATH [--format=yaml|json]

# Show effective configuration (merged default and user)
rustfig config show [--format=yaml|json|text]
```

### Appearance and Themes

```
# List available themes
rustfig theme list [--verbose]

# Show details of a theme
rustfig theme show NAME [--format=yaml|json|text]

# Set active theme
rustfig theme set NAME

# Create a new theme
rustfig theme create NAME [--base=EXISTING_THEME]

# Edit a theme
rustfig theme edit NAME [--editor=vim|nano|code]

# Import theme from file
rustfig theme import PATH [--name=NAME]

# Export theme to file
rustfig theme export NAME PATH [--format=yaml|json]

# Preview a theme
rustfig theme preview NAME
```

### Keybindings

```
# List all keybindings
rustfig keybindings list [--format=yaml|json|text]

# Set a keybinding
rustfig keybindings set ACTION KEY

# Reset keybindings to default
rustfig keybindings reset

# Test keybindings
rustfig keybindings test
```

### AI and Assistance

```
# Ask a question (single query)
rustfig ask "QUESTION" [--model=MODEL] [--markdown]

# Start interactive chat session
rustfig chat [--model=MODEL] [--conversation=ID]

# List available conversations
rustfig chat list

# Continue specific conversation
rustfig chat continue ID

# Explain a command
rustfig explain "COMMAND" [--verbose]

# Generate a command from description
rustfig generate "DESCRIPTION" [--shell=bash|zsh|fish]

# List available AI models
rustfig models list [--remote] [--local]

# Test AI connection
rustfig models test [--model=MODEL]

# Pull/download a new model (for local models)
rustfig models pull MODEL [--force]

# Show model details
rustfig models info MODEL
```

### History and Learning

```
# Show command history
rustfig history [--limit=N] [--search=PATTERN]

# Clear learned patterns
rustfig learning clear [--confirm]

# Show learned patterns
rustfig learning show [--limit=N] [--format=yaml|json|text]

# Export learned patterns
rustfig learning export PATH

# Import learned patterns
rustfig learning import PATH [--merge]

# Reset learning system
rustfig learning reset [--confirm]
```

### Plugins

```
# List installed plugins
rustfig plugins list

# Install a plugin
rustfig plugins install NAME [--version=VERSION] [--force]

# Remove a plugin
rustfig plugins remove NAME [--keep-data]

# Enable a plugin
rustfig plugins enable NAME

# Disable a plugin
rustfig plugins disable NAME

# Update all plugins
rustfig plugins update [--dry-run]

# Show plugin details
rustfig plugins info NAME
```

### Utility Commands

```
# Show a specific path
rustfig path [config|data|plugins|cache|log]

# Benchmark performance
rustfig benchmark [--full] [--iterations=N]

# Update RustFig to latest version
rustfig update [--check] [--force]

# Submit feedback or bug report
rustfig feedback [--type=bug|suggestion|feedback]

# Show help information
rustfig help [COMMAND]

# Open dashboard (if installed)
rustfig dashboard

# Temporarily enable/disable features
rustfig toggle [ghost|predictions|ai|suggestions]

# Show statistics about usage
rustfig stats [--reset] [--format=yaml|json|text]

# Export or import command snippets
rustfig snippets [import|export|list|add|remove|edit]

# Clean temporary files
rustfig clean [--all] [--cache] [--logs]
```

## Command Options

Most commands support common options:

```
--help, -h       Show help for a command
--quiet, -q      Suppress output
--verbose, -v    Show more detailed output
--json           Output in JSON format
--yaml           Output in YAML format
--debug          Show debug information
--no-color       Disable colored output
```

## Examples

```bash
# Install RustFig integration for zsh
rustfig install --shell=zsh

# Edit configuration in preferred editor
rustfig config edit

# Ask AI for help with a command
rustfig ask "How do I find large files in Linux?"

# Start an interactive chat session
rustfig chat

# Change theme to Nord
rustfig theme set nord

# Show current configuration
rustfig config show

# List recent command history
rustfig history --limit=20

# Benchmark suggestion performance
rustfig benchmark --iterations=100

# Check system for issues
rustfig doctor

# Generate a command from description
rustfig generate "create a backup of my home directory excluding node_modules"
```
