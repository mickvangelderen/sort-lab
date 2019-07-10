pub mod configuration;

pub(crate) use rand::prelude::*;

fn main() {
    let cfg = configuration::read("configuration.toml");

    let values: Vec<u32> = {
        let rng = rand::thread_rng();
        let dist = rand::distributions::Uniform::new_inclusive(cfg.input.min, cfg.input.max);
        rng.sample_iter(dist).take(cfg.input.count as usize).collect()
    };

    println!("{:?}", values);
}
