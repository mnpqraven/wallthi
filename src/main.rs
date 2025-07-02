/// later to be dotfile
struct DotfileTreeConfig {
    profile: Profile,
    presets: Vec<Preset>,
}

struct Preset {
    profile: Profile,
}

struct Profile(String);

fn main() {
    println!("Hello, world!");
}
