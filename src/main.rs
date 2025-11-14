use fg::*;

fn print_help() {
    println!("fast-git - A fast Git/GitHub CLI wrapper");
    println!();
    println!("CONFIGURATION:");
    println!("  --setMode <git|gh>        Set the mode (git or gh CLI)");
    println!("  --getMode                 Show current mode");
    println!();
    println!("ALIASES:");
    println!("  --createAlias <name> <cmd1> <cmd2> ...  Create a custom alias");
    println!("  --alias <name>            Run an alias");
    println!("  --listAliases             List all aliases");
    println!();
    println!("GIT COMMANDS:");
    println!("  --init                    Initialize repository");
    println!("  --add [path]              Add files (default: .)");
    println!("  --commit <message>        Commit changes");
    println!("  --pull [remote]           Pull from remote (default: origin)");
    println!("  --push [remote]           Push to remote (default: origin)");
    println!("  --setBranch <name>        Create and checkout branch");
    println!("  --new <name>              Create new branch");
    println!("  --ro <url>                Add remote origin");
    println!("  --info [path]             Show status (default: .)");
    println!();
    println!("OTHER:");
    println!("  --help                    Show this help");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 || args.iter().any(|a| a == "--help" || a == "-h") {
        print_help();
        return;
    }

    // Set mode
    if let Some(v) = get_flag_value(&args, "--setMode") {
        if v.is_empty() {
            eprintln!("Error: --setMode requires a value (git or gh)");
            std::process::exit(1);
        } else if let Err(e) = set_mode(&v) {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
        return;
    }

    // Get mode
    if args.iter().any(|a| a == "--getMode") {
        if let Err(e) = get_current_mode() {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
        return;
    }

    // Create alias
    if let Some(name) = get_flag_value(&args, "--createAlias") {
        if name.is_empty() {
            eprintln!("Error: --createAlias requires a name");
            std::process::exit(1);
        }
        let mut commands = Vec::new();
        let mut found_flag = false;
        for arg in &args {
            if found_flag {
                if !arg.starts_with("--") {
                    commands.push(arg.clone());
                } else {
                    break;
                }
            }
            if arg == "--createAlias" || arg.starts_with("--createAlias=") {
                found_flag = true;
            }
        }
        if commands.len() > 1 {
            commands.remove(0);
        }
        if commands.is_empty() {
            eprintln!("Error: --createAlias requires at least one command");
            std::process::exit(1);
        }
        if let Err(e) = create_alias(&name, commands) {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
        return;
    }

    // List aliases
    if args.iter().any(|a| a == "--listAliases") {
        if let Err(e) = list_aliases() {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
        return;
    }

    // Run alias
    if let Some(name) = get_flag_value(&args, "--alias") {
        if name.is_empty() {
            eprintln!("Error: --alias requires a name");
            std::process::exit(1);
        }
        if let Err(e) = run_alias(&name) {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
        return;
    }

    // Git commands
    if get_flag_value(&args, "--init").is_some() {
        if let Err(e) = git_init() {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }

    if let Some(v) = get_flag_value(&args, "--add") {
        if let Err(e) = git_add(&v) {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }

    if let Some(v) = get_flag_value(&args, "--commit") {
        if v.is_empty() {
            eprintln!("Error: --commit requires a message");
            std::process::exit(1);
        }
        if let Err(e) = git_commit(&v) {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }

    if let Some(v) = get_flag_value(&args, "--pull") {
        if let Err(e) = git_pull(&v) {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }

    if let Some(v) = get_flag_value(&args, "--push") {
        if let Err(e) = git_push(&v) {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }

    if let Some(v) = get_flag_value(&args, "--setBranch") {
        if v.is_empty() {
            eprintln!("Error: --setBranch requires a branch name");
            std::process::exit(1);
        }
        if let Err(e) = git_set_branch(&v) {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }

    if let Some(v) = get_flag_value(&args, "--ro") {
        if v.is_empty() {
            eprintln!("Error: --ro requires a repository URL");
            std::process::exit(1);
        }
        if let Err(e) = git_ro(&v) {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }

    if let Some(v) = get_flag_value(&args, "--info") {
        if let Err(e) = git_info(&v) {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }

    if let Some(v) = get_flag_value(&args, "--new") {
        if v.is_empty() {
            eprintln!("Error: --new requires a branch name");
            std::process::exit(1);
        }
        if let Err(e) = git_new(&v) {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
