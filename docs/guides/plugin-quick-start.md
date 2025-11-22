---
sidebar_position: 5
---

# Plugin Quick Start

Get started with xcargo plugins in 5 minutes.

## What are Plugins?

Plugins extend xcargo's functionality by hooking into the build process. Use plugins to:

- Send build notifications
- Collect metrics and analytics
- Add custom toolchain support
- Integrate with CI/CD systems
- Customize build workflows

## Installation

Plugins are part of xcargo core. No additional installation needed.

## Your First Plugin

### 1. Create a Plugin

Create a file `my_plugin.rs`:

```rust
use xcargo::plugin::{Plugin, PluginContext};
use xcargo::error::Result;

struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn name(&self) -> &str {
        "hello"
    }

    fn on_pre_build(&self, ctx: &PluginContext) -> Result<()> {
        println!("👋 Hello from plugin!");
        println!("   Building for: {}", ctx.target);
        Ok(())
    }

    fn on_post_build(&self, _ctx: &PluginContext) -> Result<()> {
        println!("✨ Build completed!");
        Ok(())
    }
}
```

### 2. Register the Plugin

In your main application:

```rust
use xcargo::plugin::PluginRegistry;

fn main() -> Result<()> {
    let mut registry = PluginRegistry::new();

    // Register your plugin
    registry.register(Box::new(HelloPlugin))?;

    // Plugins are now active!
    Ok(())
}
```

### 3. Test It

Run the example:

```bash
cargo run --example notification_plugin
```

Output:
```
👋 Hello from plugin!
   Building for: x86_64-unknown-linux-gnu
✨ Build completed!
```

## Common Use Cases

### Build Notifications

Send notifications when builds complete:

```rust
impl Plugin for NotificationPlugin {
    fn name(&self) -> &str { "notification" }

    fn on_post_build(&self, ctx: &PluginContext) -> Result<()> {
        let msg = format!("Build succeeded for {}", ctx.target);
        // Send notification using your preferred method
        println!("📬 {}", msg);
        Ok(())
    }

    fn on_build_failed(&self, ctx: &PluginContext, error: &str) -> Result<()> {
        println!("❌ Build failed for {}: {}", ctx.target, error);
        Ok(())
    }
}
```

### Metrics Collection

Track build times and success rates:

```rust
use std::sync::Mutex;
use std::time::Instant;

struct MetricsPlugin {
    start_time: Mutex<Option<Instant>>,
}

impl Plugin for MetricsPlugin {
    fn name(&self) -> &str { "metrics" }

    fn on_pre_build(&self, _ctx: &PluginContext) -> Result<()> {
        *self.start_time.lock().unwrap() = Some(Instant::now());
        Ok(())
    }

    fn on_post_build(&self, _ctx: &PluginContext) -> Result<()> {
        if let Some(start) = *self.start_time.lock().unwrap() {
            let duration = start.elapsed();
            println!("⏱️  Build took {:.2}s", duration.as_secs_f64());
        }
        Ok(())
    }
}
```

### Custom Environment Setup

Configure environment variables before builds:

```rust
use std::env;

impl Plugin for EnvPlugin {
    fn name(&self) -> &str { "env-setup" }

    fn on_pre_build(&self, ctx: &PluginContext) -> Result<()> {
        if ctx.target.contains("windows") {
            env::set_var("CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER",
                        "x86_64-w64-mingw32-gcc");
        }
        Ok(())
    }
}
```

## Plugin Lifecycle

Plugins can hook into these events:

| Hook | When | Use For |
|------|------|---------|
| `on_init` | Plugin registration | Setup, resource allocation |
| `on_pre_build` | Before build starts | Validation, environment setup |
| `on_post_build` | After successful build | Notifications, artifact processing |
| `on_build_failed` | When build fails | Error reporting, cleanup |
| `on_pre_toolchain_install` | Before toolchain install | Custom validation |
| `on_post_toolchain_install` | After toolchain install | Configuration |
| `on_shutdown` | Plugin unload | Cleanup, final reports |

## Accessing Build Information

Use `PluginContext` to access build details:

```rust
fn on_pre_build(&self, ctx: &PluginContext) -> Result<()> {
    println!("Target: {}", ctx.target);
    println!("Release mode: {}", ctx.release);
    println!("Project root: {:?}", ctx.project_root);

    if ctx.use_zig {
        println!("Using Zig toolchain");
    }

    if ctx.use_container {
        println!("Using container build");
    }

    Ok(())
}
```

## Sharing Data Between Hooks

Use metadata to share data:

```rust
impl Plugin for DataPlugin {
    fn on_pre_build(&self, ctx: &PluginContext) -> Result<()> {
        let mut ctx = ctx.clone();
        ctx.set_metadata("start_time".to_string(),
                        SystemTime::now().elapsed().as_secs().to_string());
        Ok(())
    }

    fn on_post_build(&self, ctx: &PluginContext) -> Result<()> {
        if let Some(start) = ctx.get_metadata("start_time") {
            println!("Started at: {}", start);
        }
        Ok(())
    }
}
```

## Error Handling

Return errors to abort operations:

```rust
fn on_pre_build(&self, ctx: &PluginContext) -> Result<()> {
    if !validate_target(&ctx.target) {
        return Err(Error::Config(
            "Target not supported by this plugin".to_string()
        ));
    }
    Ok(())
}
```

## Running Examples

xcargo includes example plugins you can run:

```bash
# Notification plugin
cargo run --example notification_plugin

# Metrics plugin
cargo run --example metrics_plugin
```

## Next Steps

- [Plugin Development Guide](plugin-development.md) - Comprehensive tutorial
- [Plugin API Reference](../api/plugins.md) - Complete API documentation
- [Example Plugins](../../examples/plugins/) - Browse example code
- [Architecture Overview](../architecture/overview.md) - System design

## Best Practices

1. **Keep plugins focused** - One plugin = one responsibility
2. **Handle errors gracefully** - Return meaningful error messages
3. **Be efficient** - Plugins run on every build
4. **Use thread-safe patterns** - Wrap mutable state in `Mutex`
5. **Document your plugin** - Add clear descriptions and examples
6. **Test thoroughly** - Write tests for all hooks

## Troubleshooting

### Plugin not called

Make sure you registered it:
```rust
registry.register(Box::new(MyPlugin))?;
```

### Thread safety errors

Implement `Send + Sync`:
```rust
struct MyPlugin {
    state: Mutex<State>,  // Wrap mutable state
}
```

### Build aborted unexpectedly

Check plugin hooks for `Err` returns:
```rust
fn on_pre_build(&self, ctx: &PluginContext) -> Result<()> {
    // Don't return Err unless you want to abort the build
    Ok(())
}
```

## Community Plugins

Coming soon: Plugin registry and marketplace!

For now, check out [discussions](https://github.com/ibrahimcesar/xcargo/discussions) for community plugins.

## Contributing

Want to share your plugin? See our [Contributing Guide](../../CONTRIBUTING.md).
