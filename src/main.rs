use std::env;

fn get_pwd_as_string() -> String {
    env::current_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap()
}

fn tilde_home_directory(path: String) -> Option<String> {
    Some(String::from("~") + path.strip_prefix(&env::var("HOME").ok()?)?)
}

fn tilde_formatted_pwd() -> String {
    match tilde_home_directory(get_pwd_as_string()) {
        Some(p) => p,
        None => get_pwd_as_string(),
    }
}

fn shorten_workspace_names(path: String) -> String {
    path.split('/')
        .enumerate()
        .map_while(|(i, s)| (i != path.split('/').count() - 1).then(|| s))
        .map(|s| match s.get(..2) {
            Some(v) => v,
            None => s,
        })
        .fold(String::from(""), |acc, cur| acc + cur + "/")
        + path.split('/').last().unwrap()
}

fn shorten_tilde_formatted_pwd() -> String {
    shorten_workspace_names(tilde_formatted_pwd())
}

fn main() {
    print!("{}", shorten_tilde_formatted_pwd());
}
