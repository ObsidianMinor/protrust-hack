#![recursion_limit="512"]

mod generator;

use generator::Generator;
use protrust::plugin;

fn main() -> plugin::Result {
    plugin::main(plugin_main)
}

fn plugin_main(context: &mut plugin::Context) -> plugin::Result {
    let options = generator::Options::parse_from(context.parse_parameter())?;
    let mut generator = Generator::new(context, options);
    generator.generate()
}