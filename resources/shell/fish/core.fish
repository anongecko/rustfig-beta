#!/usr/bin/env fish
# RustFig core functionality for Fish

# Avoid loading twice
if set -q RUSTFIG_CORE_LOADED
  return 0
end
set -gx RUSTFIG_CORE_LOADED 1

# Detect if running in an interactive shell
if not status is-interactive
  return 0
end

# Setup directory for runtime data
set -l RUSTFIG_RUNTIME_DIR $XDG_RUNTIME_DIR
if not set -q XDG_RUNTIME_DIR
  set RUSTFIG_RUNTIME_DIR /tmp
end
set RUSTFIG_RUNTIME_DIR $RUSTFIG_RUNTIME_DIR/rustfig-$USER
mkdir -p $RUSTFIG_RUNTIME_DIR

# Initialize RustFig context
function rustfig-update-context
  rustfig update-context --shell=fish --dir="$PWD" --term="$TERM" >/dev/null 2>&1
end

# Called before command execution
function __rustfig_preexec --on-event fish_preexec
  # Save the command to history
  set -l cmd $argv[1]
  rustfig record-command "$cmd" >/dev/null 2>&1
end

# Called after command completion
function __rustfig_postexec --on-event fish_postexec
  # Update context after directory changes
  rustfig-update-context
end

# Custom tab completion with RustFig predictions
function __rustfig_predict
  set -l cmdline (commandline)
  set -l cursor (commandline -C)
  
  # Get predictions from RustFig
  set -l prediction (rustfig predict --line="$cmdline" --pos=$cursor --format=completion)
  
  if test -n "$prediction"
    # Apply the prediction
    commandline -r $prediction
    commandline -C (string length $prediction)
  else
    # Fall back to default completion
    commandline -f complete
  end
end

# Toggle ghost text on/off
function rustfig-toggle-ghost
  set -l state (rustfig toggle-ghost)
  echo "Ghost text: $state"
end

# Get explanation for current command
function rustfig-explain-command
  set -l cmdline (commandline)
  if test -z "$cmdline"
    echo "No command to explain"
    return 1
  end
  
  # Get explanation from RustFig
  rustfig explain "$cmdline"
end

# Uninstall RustFig (removes integration block)
function rustfig-uninstall
  rustfig service stop
  rustfig uninstall --shell=fish
  echo "RustFig integration removed. Please restart your shell."
end

# Setup key bindings
bind \t '__rustfig_predict'

# Run the initial context setup
rustfig-update-context
