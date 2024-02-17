use rustler;

#[rustler::nif]
pub fn to_lat(text: &str) -> String {
    crate::to_lat(text)
}

fn load(_env: rustler::Env, _info: rustler::Term) -> bool {
    true
}

rustler::init!("converter", [to_lat], load = load);