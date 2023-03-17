use crate::console;
use std::marker::PhantomData;

pub const AUTH_CLIENT_TOKEN: console::CVar<String> = console::CVar {
    ty: PhantomData,
    name: "auth_client_token",
    description: r#"auth_client_token is a token that stays static between sessions.
Used to identify this client vs others."#,
    mutable: false,
    serializable: true,
    default: &String::new,
};

pub fn register_vars(vars: &mut console::Vars) {
    vars.register(AUTH_CLIENT_TOKEN);
}
