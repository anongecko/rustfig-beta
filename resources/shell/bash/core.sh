#!/usr/bin/env bash
# RustFig core functionality for Bash

# Avoid loading twice
if [ -n "$RUSTFIG_CORE_LOADED" ]; then
  return 0
fi
export RUSTFIG_CORE_LOADED=1

# Detect if running in an interactive shell
if [[ $- != *i* ]]; then
  return 0
fi

# Setup directory for runtime data
RUSTFIG_RUNTIME_DIR="${XDG_RUNTIME_DIR:-/tmp}/rustfig-$USER"
mkdir -p "$RUSTFIG_RUNTIME_DIR"

# Initialize RustFig context
__rustfig_init_context() {
  rustfig update-context --shell=bash --dir="$PWD" --term="$TERM" >/dev/null 2>&1
}

# Called before command execution
__rustfig_preexec() {
  # Save the command to history
  local cmd="$1"
  rustfig record-command "$cmd" >/dev/null 2>&1
}

# Called after command completion
__rustfig_postcmd() {
  # Update context after directory changes
  __rustfig_init_context
}

# Custom tab completion with RustFig predictions
__rustfig_complete() {
  local current_line=${READLINE_LINE}
  local cursor_pos=${READLINE_POINT}
  
  # Get predictions from RustFig
  local prediction=$(rustfig predict --line="$current_line" --pos=$cursor_pos --format=completion)
  
  if [ -n "$prediction" ]; then
    # Apply the prediction
    READLINE_LINE="$prediction"
    READLINE_POINT=${#prediction}
  else
    # Fall back to default completion
    return 1
  fi
}

# Toggle ghost text on/off
rustfig-toggle-ghost() {
  local state=$(rustfig toggle-ghost)
  echo "Ghost text: $state"
}

# Get explanation for current command
rustfig-explain-command() {
  local current_line=${READLINE_LINE}
  if [ -z "$current_line" ]; then
    echo "No command to explain"
    return 1
  fi
  
  # Get explanation from RustFig
  rustfig explain "$current_line"
}

# Uninstall RustFig (removes integration block)
rustfig-uninstall() {
  rustfig service stop
  rustfig uninstall --shell=bash
  echo "RustFig integration removed. Please restart your shell."
}

# Run the initial context setup
__rustfig_init_context

# Set bash-specific hooks if available
if [ -n "$PROMPT_COMMAND" ]; then
  PROMPT_COMMAND="__rustfig_postcmd;$PROMPT_COMMAND"
else
  PROMPT_COMMAND="__rustfig_postcmd"
fi

trap '__rustfig_preexec "$BASH_COMMAND"' DEBUG
