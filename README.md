# fast-git (fg)

A fast and efficient Git/GitHub CLI wrapper with custom aliases support.

## Features

- 🚀 **Streamlined Git commands** - Simplified interface for common Git operations
- 🔄 **Dual mode support** - Switch between `git` and `gh` (GitHub CLI) seamlessly
- 🎯 **Custom aliases** - Create and manage your own command aliases
- ⚙️ **Persistent configuration** - Settings stored in your config directory
- 📦 **Zero external dependencies** - Lightweight and fast

## Installation

Install via cargo:

```bash
cargo install fg
```

Or build from source:

```bash
git clone https://github.com/VMASPAD/fast-git.git
cd fast-git
cargo build --release
```

The binary will be available as `fast-git`.

## Quick Start

```bash
# Initialize a repository
fast-git --init

# Add files
fast-git --add .

# Commit changes
fast-git --commit "Initial commit"

# Add remote origin
fast-git --ro https://github.com/username/repo.git

# Push to origin
fast-git --push
```

## Configuration

### Set Mode

Switch between `git` and GitHub CLI (`gh`):

```bash
# Use git (default)
fast-git --setMode git

# Use GitHub CLI
fast-git --setMode gh

# Check current mode
fast-git --getMode
```

## Commands

### Git Operations

| Command | Description | Example |
|---------|-------------|---------|
| `--init` | Initialize a Git repository | `fast-git --init` |
| `--add [path]` | Add files to staging (default: `.`) | `fast-git --add src/` |
| `--commit <message>` | Commit changes | `fast-git --commit "Fix bug"` |
| `--pull [remote]` | Pull from remote (default: `origin`) | `fast-git --pull` |
| `--push [remote]` | Push to remote (default: `origin`) | `fast-git --push origin` |
| `--setBranch <name>` | Create and checkout a new branch | `fast-git --setBranch feature` |
| `--new <name>` | Create a new branch | `fast-git --new develop` |
| `--ro <url>` | Add remote origin | `fast-git --ro https://github.com/user/repo.git` |
| `--info [path]` | Show repository status (default: `.`) | `fast-git --info` |

### Aliases

Create custom command sequences:

```bash
# Create an alias
fast-git --createAlias quickpush "add ." "commit -m 'Quick update'" "push"

# List all aliases
fast-git --listAliases

# Run an alias
fast-git --alias quickpush
```

### Help

```bash
fast-git --help
```

## Usage Examples

### Basic Workflow

```bash
# Initialize and configure repository
fast-git --init
fast-git --ro https://github.com/username/myproject.git

# Make changes and commit
fast-git --add .
fast-git --commit "Add new feature"
fast-git --push
```

### Branch Management

```bash
# Create a new feature branch
fast-git --new feature/awesome-feature

# Make changes
fast-git --add .
fast-git --commit "Implement awesome feature"

# Push the new branch
fast-git --push origin
```

### Using Aliases

```bash
# Create a sync alias
fast-git --createAlias sync "pull" "add ." "commit -m 'Sync'" "push"

# Use it anytime
fast-git --alias sync
```

### GitHub CLI Mode

```bash
# Switch to GitHub CLI
fast-git --setMode gh

# Now all commands use gh instead of git
fast-git --push  # Uses: gh push origin
```

## Configuration Files

Configuration is stored in your system's config directory:

- **Linux/macOS**: `~/.config/fg/`
- **Windows**: `%APPDATA%\fg\`

Files:
- `config.json` - Mode configuration
- `aliases.json` - Custom aliases

## Library Usage

`fast-git` can also be used as a library in your Rust projects:

```toml
[dependencies]
fg = "0.1.0"
```

```rust
use fg::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize repository
    git_init()?;
    
    // Add all files
    git_add(".")?;
    
    // Commit
    git_commit("Initial commit")?;
    
    // Create and run custom alias
    create_alias("deploy", vec![
        "add .".to_string(),
        "commit -m 'Deploy'".to_string(),
        "push".to_string()
    ])?;
    
    run_alias("deploy")?;
    
    Ok(())
}
```

## Requirements

- Rust 1.56 or later
- Git installed on your system
- (Optional) GitHub CLI (`gh`) for `gh` mode

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Author

VMASPAD

## Links

- [Repository](https://github.com/VMASPAD/fast-git)
- [Issues](https://github.com/VMASPAD/fast-git/issues)
- [Crates.io](https://crates.io/crates/fg)
