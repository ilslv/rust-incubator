use std::borrow::Cow;

fn get_command_line_arg(name: &str) -> Option<String> {
    let mut v = std::env::args()
        .skip_while(|arg| arg != name)
        .take(2)
        .collect::<Vec<_>>();

    if v.len() == 1 {
        panic!("No path specified after --conf parameter!")
    }

    v.pop()
}

fn main() {
    let s: Cow<'static, str> = if let Ok(env_var) = std::env::var("APP_CONF") {
        env_var.into()
    } else if let Some(command_line_arg) = get_command_line_arg("--conf") {
        command_line_arg.into()
    } else {
        "/etc/app/app.conf".into()
    };

    println!("{}", s);
}
