use std::env;

fn get_pwd_as_string() -> String {
    match env::current_dir() {
        Ok(dir) => dir.into_os_string().into_string().unwrap(),
        Err(_) => "DIR_ERROR".to_string(),
    }
}

fn home_path() -> Option<String> {
    if cfg!(windows) {
        env::var("USERPROFILE").ok()
    } else {
        env::var("HOME").ok()
    }
}

fn tilde_home_directory(path: &str) -> Option<String> {
    Some(String::from("~") + path.strip_prefix(&home_path()?)?)
}

fn tilde_formatted_pwd() -> String {
    match tilde_home_directory(&get_pwd_as_string()) {
        Some(p) => p,
        None => get_pwd_as_string(),
    }
}

fn split_char() -> char {
    if cfg!(windows) {
        '\\'
    } else {
        '/'
    }
}

fn shorten_workspace_names(path: &str) -> String {
    let split = split_char();
    path.split(split)
        .enumerate()
        .map_while(|(i, s)| (i != path.split(split).count() - 1).then_some(s))
        .map(|s| match s.get(..2) {
            Some(v) => v,
            None => s,
        })
        .fold(String::new(), |acc, cur| acc + cur + &split.to_string())
        + path.split(split).last().unwrap()
}

fn shorten_tilde_formatted_pwd() -> String {
    shorten_workspace_names(&tilde_formatted_pwd())
}

fn main() {
    print!("{}", shorten_tilde_formatted_pwd());
}
