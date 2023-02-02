use std::env;

fn get_pwd_as_string() -> String {
    match env::current_dir() {
        Ok(dir) => dir.into_os_string().into_string().unwrap(),
        Err(_) => "DIR_ERROR".to_string(),
    }
}

fn tilde_home_directory(path: &str) -> Option<String> {
    Some(String::from("~") + path.strip_prefix(&env::var("HOME").ok()?)?)
}

fn tilde_formatted_pwd() -> String {
    match tilde_home_directory(&get_pwd_as_string()) {
        Some(p) => p,
        None => get_pwd_as_string(),
    }
}

fn shorten_workspace_names(path: &str) -> String {
    path.split('/')
        .enumerate()
        .map_while(|(i, s)| (i != path.split('/').count() - 1).then_some(s))
        .map(|s| match s.get(..2) {
            Some(v) => v,
            None => s,
        })
        .fold(String::new(), |acc, cur| acc + cur + "/")
        + path.split('/').last().unwrap()
}

fn shorten_tilde_formatted_pwd() -> String {
    shorten_workspace_names(&tilde_formatted_pwd())
}

fn main() {
    print!("{}", shorten_tilde_formatted_pwd());
}
