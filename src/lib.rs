/// Provides input output types for reading and writing protobuf streams
pub mod io;
/// Provides compatibility types for proto2 messages
pub mod compat;
/// Provides types for LITE_RUNTIME optimized proto files
pub mod lite {
    pub use MessageLite;
    pub use io::CodedInput;
    pub use io::InputResult;
}
/// Provides types for the main CODE_SIZE and SPEED optimized proto files
pub mod codegen {
    pub use ::lite::*;
}

pub trait GeneratedMessage {
    fn merge_with(&mut self, other: &Self);
}

/// A message with all the required information to merge, write, and calculate its size, as well as get basic reflection information
pub trait MessageLite {
    /// Merges fields from the coded input into this message
    fn merge_from(&mut self, input: &mut io::CodedInput) -> io::InputResult<()>;

    /// Calculates the size of the message and returns it as an 32-bit integer or None if the message is larger than `i32::MAX`
    fn calculate_size(&self) -> Option<i32>;

    /// Writes the fields of this message to the coded output
    fn write_to(&self, output: &mut io::CodedOutput) -> io::OutputResult;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
