use std::error::Error;
use std::process::Command;
use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;

fn get_config_dir() -> PathBuf {
    let mut path = dirs::config_dir().unwrap_or_else(|| {
        let mut fallback = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        fallback.push(".config");
        fallback
    });
    path.push("fg");
    path
}

fn get_config_file() -> PathBuf {
    let mut path = get_config_dir();
    path.push("config.json");
    path
}

fn get_aliases_file() -> PathBuf {
    let mut path = get_config_dir();
    path.push("aliases.json");
    path
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
struct Config {
    mode: String, // "git" or "gh"
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
struct Aliases {
    commands: HashMap<String, Vec<String>>,
}

fn load_config() -> Config {
    let config_file = get_config_file();
    if let Ok(content) = fs::read_to_string(config_file) {
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        Config { mode: "git".to_string() }
    }
}

fn save_config(config: &Config) -> Result<(), Box<dyn Error>> {
    let config_dir = get_config_dir();
    fs::create_dir_all(&config_dir)?;
    let config_file = get_config_file();
    let content = serde_json::to_string_pretty(config)?;
    fs::write(config_file, content)?;
    Ok(())
}

fn load_aliases() -> Aliases {
    let aliases_file = get_aliases_file();
    if let Ok(content) = fs::read_to_string(aliases_file) {
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        Aliases::default()
    }
}

fn save_aliases(aliases: &Aliases) -> Result<(), Box<dyn Error>> {
    let config_dir = get_config_dir();
    fs::create_dir_all(&config_dir)?;
    let aliases_file = get_aliases_file();
    let content = serde_json::to_string_pretty(aliases)?;
    fs::write(aliases_file, content)?;
    Ok(())
}

pub fn set_mode(mode: &str) -> Result<(), Box<dyn Error>> {
    if mode != "git" && mode != "gh" {
        return Err("Mode must be 'git' or 'gh'".into());
    }
    let config = Config { mode: mode.to_string() };
    save_config(&config)?;
    println!("Mode set to: {}", mode);
    Ok(())
}

pub fn create_alias(name: &str, commands: Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut aliases = load_aliases();
    aliases.commands.insert(name.to_string(), commands.clone());
    save_aliases(&aliases)?;
    println!("Alias '{}' created with commands: {:?}", name, commands);
    Ok(())
}

pub fn run_alias(name: &str) -> Result<(), Box<dyn Error>> {
    let aliases = load_aliases();
    if let Some(commands) = aliases.commands.get(name) {
        let config = load_config();
        for cmd in commands {
            run_command(&config.mode, cmd)?;
        }
        Ok(())
    } else {
        Err(format!("Alias '{}' not found", name).into())
    }
}

pub fn list_aliases() -> Result<(), Box<dyn Error>> {
    let aliases = load_aliases();
    if aliases.commands.is_empty() {
        println!("No aliases configured");
    } else {
        println!("Configured aliases:");
        for (name, commands) in &aliases.commands {
            println!("  {} -> {:?}", name, commands);
        }
    }
    Ok(())
}

pub fn get_current_mode() -> Result<(), Box<dyn Error>> {
    let config = load_config();
    println!("Current mode: {}", config.mode);
    Ok(())
}

fn run_command(mode: &str, cmd: &str) -> Result<(), Box<dyn Error>> {
    let status = Command::new(mode).args(cmd.split_whitespace()).status()?;
    if status.success() {
        println!("{} {} -> OK", mode, cmd);
        Ok(())
    } else {
        Err(format!("{} {} failed (status: {})", mode, cmd, status).into())
    }
}

pub fn git_init() -> Result<(), Box<dyn Error>> {
    let config = load_config();
    let status = Command::new(&config.mode).arg("init").status()?;
    if status.success() {
        println!("{} init -> OK", config.mode);
        Ok(())
    } else {
        Err(format!("{} init failed (status: {})", config.mode, status).into())
    }
}

pub fn git_add(archive: &str) -> Result<(), Box<dyn Error>> {
    let config = load_config();
    let path = if archive.is_empty() { "." } else { archive };
    let status = Command::new(&config.mode).arg("add").arg(path).status()?;
    if status.success() {
        println!("{} add {} -> OK", config.mode, path);
        Ok(())
    } else {
        Err(format!("{} add failed (status: {})", config.mode, status).into())
    }
}

pub fn git_commit(text: &str) -> Result<(), Box<dyn Error>> {
    let config = load_config();
    let status = Command::new(&config.mode).arg("commit").arg("-m").arg(text).status()?;
    if status.success() {
        println!("{} commit -> OK ({})", config.mode, text);
        Ok(())
    } else {
        Err(format!("{} commit failed (status: {})", config.mode, status).into())
    }
}

pub fn git_pull(remote: &str) -> Result<(), Box<dyn Error>> {
    let config = load_config();
    let remote_arg = if remote.is_empty() { "origin" } else { remote };
    let status = Command::new(&config.mode).arg("pull").arg(remote_arg).status()?;
    if status.success() {
        println!("{} pull {} -> OK", config.mode, remote_arg);
        Ok(())
    } else {
        Err(format!("{} pull failed (status: {})", config.mode, status).into())
    }
}

pub fn git_push(remote: &str) -> Result<(), Box<dyn Error>> {
    let config = load_config();
    let remote_arg = if remote.is_empty() { "origin" } else { remote };
    let status = Command::new(&config.mode).arg("push").arg(remote_arg).status()?;
    if status.success() {
        println!("{} push {} -> OK", config.mode, remote_arg);
        Ok(())
    } else {
        Err(format!("{} push failed (status: {})", config.mode, status).into())
    }
}

pub fn git_set_branch(branch: &str) -> Result<(), Box<dyn Error>> {
    let config = load_config();
    let status = Command::new(&config.mode).arg("checkout").arg("-b").arg(branch).status()?;
    if status.success() {
        println!("{} setBranch {} -> OK", config.mode, branch);
        Ok(())
    } else {
        Err(format!("{} setBranch failed (status: {})", config.mode, status).into())
    }
}

pub fn git_ro(repository: &str) -> Result<(), Box<dyn Error>> {
    let config = load_config();
    let status = Command::new(&config.mode)
        .arg("remote")
        .arg("add")
        .arg("origin")
        .arg(repository)
        .status()?;
    if status.success() {
        println!("{} remote add origin {} -> OK", config.mode, repository);
        Ok(())
    } else {
        Err(format!("{} remote add failed (status: {})", config.mode, status).into())
    }
}

pub fn git_info(target: &str) -> Result<(), Box<dyn Error>> {
    let config = load_config();
    let arg = if target.is_empty() { "." } else { target };
    let status = Command::new(&config.mode).arg("status").arg(arg).status()?;
    if status.success() {
        println!("{} status {} -> OK", config.mode, arg);
        Ok(())
    } else {
        Err(format!("{} status failed (status: {})", config.mode, status).into())
    }
}

pub fn git_new(value: &str) -> Result<(), Box<dyn Error>> {
    let config = load_config();
    let status = Command::new(&config.mode).arg("checkout").arg("-b").arg(value).status()?;
    if status.success() {
        println!("{} checkout -b {} -> OK", config.mode, value);
        Ok(())
    } else {
        Err(format!("{} checkout failed (status: {})", config.mode, status).into())
    }
}

pub fn get_flag_value(args: &[String], flag: &str) -> Option<String> {
    for (i, a) in args.iter().enumerate() {
        if let Some(v) = a.strip_prefix(&format!("{}=", flag)) {
            return Some(v.to_string());
        }
        if a == flag {
            if let Some(next) = args.get(i + 1) {
                if !next.starts_with('-') {
                    return Some(next.clone());
                } else {
                    return Some(String::new());
                }
            } else {
                return Some(String::new());
            }
        }
    }
    None
}
