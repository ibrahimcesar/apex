---
sidebar_position: 5
---

# Plugin API Reference

## Overview

The xcargo plugin system provides a trait-based API for extending functionality. This document covers the complete plugin API surface.

## Core Traits

### `Plugin` Trait

The main trait that all plugins must implement.

```rust
pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    fn version(&self) -> &str { "0.1.0" }
    fn description(&self) -> &str { "" }
    fn author(&self) -> &str { "" }

    // Build lifecycle hooks
    fn on_pre_build(&self, ctx: &PluginContext) -> Result<()> { Ok(()) }
    fn on_post_build(&self, ctx: &PluginContext) -> Result<()> { Ok(()) }
    fn on_build_failed(&self, ctx: &PluginContext, error: &str) -> Result<()> { Ok(()) }

    // Toolchain lifecycle hooks
    fn on_pre_toolchain_install(&self, ctx: &PluginContext) -> Result<()> { Ok(()) }
    fn on_post_toolchain_install(&self, ctx: &PluginContext) -> Result<()> { Ok(()) }

    // Plugin lifecycle
    fn on_init(&self) -> Result<()> { Ok(()) }
    fn on_shutdown(&self) -> Result<()> { Ok(()) }
}
```

#### Required Methods

- **`name(&self) -> &str`**
  Returns the plugin's unique identifier. Must be unique across all registered plugins.

  **Example**: `"notification"`, `"metrics"`, `"custom-linker"`

#### Optional Methods

All other methods have default implementations that do nothing and return `Ok(())`.

##### Metadata Methods

- **`version(&self) -> &str`**
  Plugin version string. Defaults to `"0.1.0"`.
  Recommended to use semantic versioning.

- **`description(&self) -> &str`**
  Brief description of plugin functionality.

- **`author(&self) -> &str`**
  Plugin author name and/or email.

##### Build Lifecycle Hooks

- **`on_pre_build(&self, ctx: &PluginContext) -> Result<()>`**
  Called before build starts.
  Return `Err` to abort the build.

  **Use cases**:
  - Pre-build validation
  - Environment setup
  - Cache checking
  - Custom compilation flags

- **`on_post_build(&self, ctx: &PluginContext) -> Result<()>`**
  Called after successful build completion.

  **Use cases**:
  - Notifications
  - Artifact processing
  - Metrics collection
  - Deployment

- **`on_build_failed(&self, ctx: &PluginContext, error: &str) -> Result<()>`**
  Called when build fails.
  Receives error message as parameter.

  **Use cases**:
  - Error reporting
  - Cleanup
  - Failure notifications
  - Logging

##### Toolchain Lifecycle Hooks

- **`on_pre_toolchain_install(&self, ctx: &PluginContext) -> Result<()>`**
  Called before toolchain installation.
  Return `Err` to skip installation.

  **Use cases**:
  - Custom toolchain validation
  - Alternative toolchain providers
  - Installation prerequisites check

- **`on_post_toolchain_install(&self, ctx: &PluginContext) -> Result<()>`**
  Called after toolchain installation.

  **Use cases**:
  - Toolchain configuration
  - Installation verification
  - Additional tool installation

##### Plugin Lifecycle

- **`on_init(&self) -> Result<()>`**
  Called when plugin is registered.
  Runs once during plugin registration.

  **Use cases**:
  - One-time initialization
  - Resource allocation
  - Configuration loading

- **`on_shutdown(&self) -> Result<()>`**
  Called when plugin is unloaded.
  Runs during application shutdown or plugin unregistration.

  **Use cases**:
  - Cleanup
  - Resource deallocation
  - Final statistics reporting

## Data Types

### `PluginContext`

Context information passed to plugin hooks.

```rust
#[derive(Debug, Clone, Default)]
pub struct PluginContext {
    pub target: String,
    pub release: bool,
    pub project_root: PathBuf,
    pub cargo_args: Vec<String>,
    pub toolchain: Option<String>,
    pub use_container: bool,
    pub use_zig: bool,
    pub metadata: HashMap<String, String>,
}
```

#### Fields

- **`target: String`**
  Target triple being built (e.g., `"x86_64-unknown-linux-gnu"`)

- **`release: bool`**
  `true` for release builds, `false` for debug builds

- **`project_root: PathBuf`**
  Absolute path to project root directory

- **`cargo_args: Vec<String>`**
  Additional arguments passed to cargo

- **`toolchain: Option<String>`**
  Toolchain being used, if specified (e.g., `"stable"`, `"nightly"`)

- **`use_container: bool`**
  `true` if using container-based build

- **`use_zig: bool`**
  `true` if using Zig for cross-compilation

- **`metadata: HashMap<String, String>`**
  Custom metadata for inter-plugin communication

#### Builder Methods

```rust
impl PluginContext {
    pub fn new(target: String) -> Self;
    pub fn with_release(self, release: bool) -> Self;
    pub fn with_project_root(self, root: PathBuf) -> Self;
    pub fn with_cargo_args(self, args: Vec<String>) -> Self;
    pub fn with_toolchain(self, toolchain: Option<String>) -> Self;
    pub fn with_container(self, use_container: bool) -> Self;
    pub fn with_zig(self, use_zig: bool) -> Self;
}
```

#### Metadata Methods

```rust
impl PluginContext {
    pub fn set_metadata(&mut self, key: String, value: String);
    pub fn get_metadata(&self, key: &str) -> Option<&String>;
}
```

**Example**:

```rust
let mut ctx = PluginContext::new("x86_64-unknown-linux-gnu".to_string())
    .with_release(true);

ctx.set_metadata("build_time".to_string(), "42s".to_string());

if let Some(time) = ctx.get_metadata("build_time") {
    println!("Build took: {}", time);
}
```

### `PluginMetadata`

Plugin metadata structure.

```rust
#[derive(Debug, Clone)]
pub struct PluginMetadata {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub enabled: bool,
}
```

#### Methods

```rust
impl PluginMetadata {
    pub fn new(name: String, version: String) -> Self;
}
```

### `PluginHook`

Enum representing hook execution points.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PluginHook {
    PreBuild,
    PostBuild,
    BuildFailed,
    PreToolchainInstall,
    PostToolchainInstall,
    Init,
    Shutdown,
}
```

#### Methods

```rust
impl PluginHook {
    pub fn as_str(&self) -> &str;
    pub fn execute(&self, plugin: &dyn Plugin, ctx: &PluginContext) -> Result<()>;
    pub fn execute_with_error(&self, plugin: &dyn Plugin, ctx: &PluginContext, error: &str) -> Result<()>;
}
```

## Plugin Registry

### `PluginRegistry`

Manages registered plugins and executes hooks.

```rust
#[derive(Default)]
pub struct PluginRegistry {
    // Internal fields
}
```

#### Construction

```rust
impl PluginRegistry {
    pub fn new() -> Self;
}
```

#### Plugin Management

```rust
impl PluginRegistry {
    pub fn register(&mut self, plugin: Box<dyn Plugin>) -> Result<()>;
    pub fn unregister(&mut self, name: &str) -> Result<()>;
    pub fn get(&self, name: &str) -> Option<Arc<dyn Plugin>>;
    pub fn contains(&self, name: &str) -> bool;
    pub fn count(&self) -> usize;
    pub fn list(&self) -> Vec<String>;
}
```

- **`register(plugin)`**
  Register a new plugin. Returns error if plugin with same name already exists.
  Automatically calls `on_init()`.

- **`unregister(name)`**
  Unregister a plugin by name. Calls `on_shutdown()` before removal.

- **`get(name)`**
  Get a reference to a registered plugin.

- **`contains(name)`**
  Check if a plugin is registered.

- **`count()`**
  Get number of registered plugins.

- **`list()`**
  Get list of registered plugin names.

#### Hook Execution

```rust
impl PluginRegistry {
    pub fn execute_hook(&self, hook: PluginHook, ctx: &PluginContext) -> Result<()>;
    pub fn execute_hook_with_error(&self, hook: PluginHook, ctx: &PluginContext, error: &str) -> Result<()>;
    pub fn shutdown(&mut self) -> Result<()>;
}
```

- **`execute_hook(hook, ctx)`**
  Execute a hook on all registered plugins in registration order.
  Stops on first error.

- **`execute_hook_with_error(hook, ctx, error)`**
  Execute hook with error message (for `BuildFailed` hook).

- **`shutdown()`**
  Shutdown all plugins and clear registry.

#### Execution Order

```rust
impl PluginRegistry {
    pub fn set_execution_order(&mut self, order: Vec<String>) -> Result<()>;
}
```

- **`set_execution_order(order)`**
  Set custom execution order for plugins.
  Returns error if any plugin name is not registered.

**Example**:

```rust
registry.set_execution_order(vec![
    "cache-validator".to_string(),
    "metrics".to_string(),
    "notification".to_string(),
])?;
```

## Module Functions

### `init()`

Initialize the plugin system.

```rust
pub fn init() -> Result<PluginRegistry>
```

Returns a new `PluginRegistry` with built-in plugins registered (if any).

**Example**:

```rust
use xcargo::plugin;

let registry = plugin::init()?;
```

## Error Handling

All plugin methods return `Result<()>`. Plugins should use the xcargo error types:

```rust
use xcargo::error::{Error, Result};

impl Plugin for MyPlugin {
    fn on_pre_build(&self, ctx: &PluginContext) -> Result<()> {
        if some_condition {
            return Err(Error::Config("Invalid configuration".to_string()));
        }
        Ok(())
    }
}
```

### Error Types

Plugins can return any of these error variants:

- `Error::Config(String)` - Configuration errors
- `Error::Build(String)` - Build errors
- `Error::Io(std::io::Error)` - I/O errors
- `Error::TargetNotFound(String)` - Target errors
- `Error::ToolchainNotFound(String)` - Toolchain errors

## Thread Safety

All plugins must implement `Send + Sync` to support:

- Parallel builds
- Multi-threaded execution
- Arc-based sharing

**Example**:

```rust
use std::sync::Mutex;

struct ThreadSafePlugin {
    state: Mutex<PluginState>,
}

impl Plugin for ThreadSafePlugin {
    // Implementation
}
```

## Complete Example

```rust
use xcargo::error::Result;
use xcargo::plugin::{Plugin, PluginContext, PluginRegistry, PluginHook};
use std::sync::Mutex;
use std::time::Instant;

struct TimerPlugin {
    start_time: Mutex<Option<Instant>>,
}

impl TimerPlugin {
    fn new() -> Self {
        Self {
            start_time: Mutex::new(None),
        }
    }
}

impl Plugin for TimerPlugin {
    fn name(&self) -> &str {
        "timer"
    }

    fn version(&self) -> &str {
        "1.0.0"
    }

    fn description(&self) -> &str {
        "Measures build duration"
    }

    fn on_pre_build(&self, ctx: &PluginContext) -> Result<()> {
        *self.start_time.lock().unwrap() = Some(Instant::now());
        println!("⏱️  Starting timer for {}", ctx.target);
        Ok(())
    }

    fn on_post_build(&self, _ctx: &PluginContext) -> Result<()> {
        if let Some(start) = *self.start_time.lock().unwrap() {
            let duration = start.elapsed();
            println!("✅ Build completed in {:.2}s", duration.as_secs_f64());
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let mut registry = PluginRegistry::new();
    registry.register(Box::new(TimerPlugin::new()))?;

    let ctx = PluginContext::new("x86_64-unknown-linux-gnu".to_string());

    registry.execute_hook(PluginHook::PreBuild, &ctx)?;
    // ... perform build ...
    registry.execute_hook(PluginHook::PostBuild, &ctx)?;

    Ok(())
}
```

## See Also

- [Plugin Development Guide](../guides/plugin-development.md) - Tutorial and examples
- [Architecture Overview](../architecture/overview.md) - System architecture
- [API Overview](overview.md) - General API documentation
