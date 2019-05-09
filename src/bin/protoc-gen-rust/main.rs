#![recursion_limit="512"]

mod generator;

use generator::Generator;
use protrust::plugin;

fn main() -> plugin::Result {
    plugin::main(plugin_main)
}

fn plugin_main(context: &mut plugin::Context) -> plugin::Result {
    let options = generator::Options::parse_from(context.parse_parameter())
        .map_err(|(k, v)| UnknownParameterError(k.to_string(), v.map(|v| v.to_string())))?;
    let mut generator = Generator::new(context, options);
    generator.generate()
}

#[derive(Debug)]
struct UnknownParameterError(String, Option<String>);

impl std::fmt::Display for UnknownParameterError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.1 {
            Some(v) => write!(f, "{}={}", self.0, v),
            None => write!(f, "{}", self.0)
        }
    }
}

impl std::error::Error for UnknownParameterError { }