use std::error::Error;
use std::process::Command;

fn git_init() -> Result<(), Box<dyn Error>> {
    let status = Command::new("git").arg("init").status()?;
    if status.success() {
        println!("git init -> OK");
        Ok(())
    } else {
        Err(format!("git init failed (status: {})", status).into())
    }
}

fn git_add(archive: &str) -> Result<(), Box<dyn Error>> {
    // Ejecuta: git add <path> y comprueba el estado
    let status = Command::new("git").arg("add").arg(if archive.is_empty() { "." } else { archive }).status()?;
    if status.success() {
        println!("git add {} -> OK", if archive.is_empty() { "." } else { archive });
        Ok(())
    } else {
        Err(format!("git add failed (status: {})", status).into())
    }
}

fn git_commit(text: &str) -> Result<(), Box<dyn Error>> {
    let status = Command::new("git").arg("commit").arg("-m").arg(text).status()?;
    if status.success() {
        println!("git commit -> OK ({})", text);
        Ok(())
    } else {
        Err(format!("git commit failed (status: {})", status).into())
    }
}

fn git_pull(remote: &str) -> Result<(), Box<dyn Error>> {
    let remote_arg = if remote.is_empty() { "origin" } else { remote };
    let status = Command::new("git").arg("pull").arg(remote_arg).status()?;
    if status.success() {
        println!("git pull {} -> OK", remote_arg);
        Ok(())
    } else {
        Err(format!("git pull failed (status: {})", status).into())
    }
}

fn git_set_branch(branch: &str) -> Result<(), Box<dyn Error>> {
    let status = Command::new("git").arg("checkout").arg("-b").arg(branch).status()?;
    if status.success() {
        println!("git setBranch {} -> OK", branch);
        Ok(())
    } else {
        Err(format!("git setBranch failed (status: {})", status).into())
    }
}

fn git_ro(repository: &str) -> Result<(), Box<dyn Error>> {
    let status = Command::new("git").arg("remote").arg("add").arg("ro").arg(repository).status()?;
    if status.success() {
        println!("git ro {} -> OK", repository);
        Ok(())
    } else {
        Err(format!("git ro failed (status: {})", status).into())
    }
}

fn git_info(target: &str) -> Result<(), Box<dyn Error>> {
    let arg = if target.is_empty() { "." } else { target };
    let status = Command::new("git").arg("status").arg(arg).status()?;
    if status.success() {
        println!("git info {} -> OK", arg);
        Ok(())
    } else {
        Err(format!("git info failed (status: {})", status).into())
    }
}

fn git_new(value: &str) -> Result<(), Box<dyn Error>> {
    let status = Command::new("git").arg("checkout").arg("-b").arg(value).status()?;
    if status.success() {
        println!("git new {} -> OK", value);
        Ok(())
    } else {
        Err(format!("git new failed (status: {})", status).into())
    }
}

fn get_flag_value(args: &[String], flag: &str) -> Option<String> {
    // Accepts: "--flag", "--flag value", "--flag=value".
    // If flag is present without a value, returns Some(empty string).
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

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if let Some(_v) = get_flag_value(&args, "--init") {
        if let Err(e) = git_init() {
            eprintln!("git init error: {}", e);
        }
    }

    if let Some(v) = get_flag_value(&args, "--add") {
        // if v is empty, git_add will treat it as "."
        if let Err(e) = git_add(&v) {
            eprintln!("git add error: {}", e);
        }
    }

    if let Some(v) = get_flag_value(&args, "--commit") {
        if v.is_empty() {
            eprintln!("git commit error: missing commit message");
        } else if let Err(e) = git_commit(&v) {
            eprintln!("git commit error: {}", e);
        }
    }

    if let Some(v) = get_flag_value(&args, "--pull") {
        if let Err(e) = git_pull(&v) {
            eprintln!("git pull error: {}", e);
        }
    }

    if let Some(v) = get_flag_value(&args, "--setBranch") {
        if v.is_empty() {
            eprintln!("git setBranch error: missing branch name");
        } else if let Err(e) = git_set_branch(&v) {
            eprintln!("git setBranch error: {}", e);
        }
    }

    if let Some(v) = get_flag_value(&args, "--ro") {
        if v.is_empty() {
            eprintln!("git ro error: missing repository URL");
        } else if let Err(e) = git_ro(&v) {
            eprintln!("git ro error: {}", e);
        }
    }

    if let Some(v) = get_flag_value(&args, "--info") {
        if let Err(e) = git_info(&v) {
            eprintln!("git info error: {}", e);
        }
    }

    if let Some(v) = get_flag_value(&args, "--new") {
        if v.is_empty() {
            eprintln!("git new error: missing branch name");
        } else if let Err(e) = git_new(&v) {
            eprintln!("git new error: {}", e);
        }
    }
}
