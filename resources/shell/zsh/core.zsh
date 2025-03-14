#!/usr/bin/env zsh
# RustFig core functionality for Zsh

# Avoid loading twice
if [[ -n "$RUSTFIG_CORE_LOADED" ]]; then
  return 0
fi
export RUSTFIG_CORE_LOADED=1

# Detect if running in an interactive shell
[[ -o interactive ]] || return 0

# Setup directory for runtime data
RUSTFIG_RUNTIME_DIR="${XDG_RUNTIME_DIR:-/tmp}/rustfig-$USER"
mkdir -p "$RUSTFIG_RUNTIME_DIR"

# Initialize RustFig context
rustfig-update-context() {
  rustfig update-context --shell=zsh --dir="$PWD" --term="$TERM" >/dev/null 2>&1
}

# Called before command execution
rustfig-preexec() {
  # Save the command to history
  local cmd="$1"
  rustfig record-command "$cmd" >/dev/null 2>&1
}

# Called after command completion
rustfig-precmd() {
  # Update context after directory changes
  rustfig-update-context
}

# Custom tab completion with RustFig predictions
rustfig-predict() {
  local current_buffer="$BUFFER"
  local cursor_pos="$CURSOR"
  
  # Get predictions from RustFig
  local prediction=$(rustfig predict --line="$current_buffer" --pos=$cursor_pos --format=completion)
  
  if [[ -n "$prediction" ]]; then
    # Apply the prediction
    BUFFER="$prediction"
    CURSOR=${#prediction}
  else
    # Fall back to default completion
    zle complete-word
  fi
}

# Toggle ghost text on/off
rustfig-toggle-ghost() {
  local state=$(rustfig toggle-ghost)
  echo "Ghost text: $state"
}

# Get explanation for current command
rustfig-explain-command() {
  local current_buffer="$BUFFER"
  if [[ -z "$current_buffer" ]]; then
    echo "No command to explain"
    return 1
  fi
  
  # Save current buffer and clear the line
  local saved_buffer="$BUFFER"
  local saved_cursor="$CURSOR"
  BUFFER=""
  zle redisplay
  
  # Get explanation from RustFig
  rustfig explain "$saved_buffer"
  
  # Restore the line
  BUFFER="$saved_buffer"
  CURSOR="$saved_cursor"
  zle redisplay
}

# Uninstall RustFig (removes integration block)
rustfig-uninstall() {
  rustfig service stop
  rustfig uninstall --shell=zsh
  echo "RustFig integration removed. Please restart your shell."
}

# Define ZLE widgets
zle -N rustfig-toggle-ghost
zle -N rustfig-explain-command
zle -N rustfig-predict

# Run the initial context setup
rustfig-update-context
