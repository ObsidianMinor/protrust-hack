/// Provides input output types for reading and writing protobuf streams
pub mod io;
/// Provides compatibility types for proto2 messages
pub mod compat;
/// Provides reflection acccess for messages
pub mod reflect;
/// Provides types for LITE_RUNTIME optimized proto files
pub mod lite {
    pub use MessageLite;
    pub use GeneratedLiteMessage;
    pub use io::CodedInput;
    pub use io::InputResult;
    pub use io::CodedOutput;
    pub use io::OutputResult;
}
/// Provides types for the main CODE_SIZE and SPEED optimized proto files
pub mod codegen {
    pub use ::lite::*;
    pub use Message;
    pub use GeneratedMessage;
}

/// A message with all the required information to merge, write, and calculate its size, as well as get basic reflection information
pub trait MessageLite {
    /// Gets a lite descriptor of this message
    fn descriptor(&self) -> &reflect::LiteDescriptor;
    /// Merges fields from the coded input into this message
    fn merge_from(&mut self, input: &mut io::CodedInput) -> io::InputResult<()>;
    /// Calculates the size of the message and returns it as an 32-bit integer or None if the message is larger than `i32::MAX`
    fn calculate_size(&self) -> Option<i32>;
    /// Writes the fields of this message to the coded output
    fn write_to(&self, output: &mut io::CodedOutput) -> io::OutputResult;
}

/// A generated lite message
pub trait GeneratedLiteMessage {
    /// Gets the lite descriptor for this message
    fn descriptor() -> &'static reflect::LiteDescriptor;
    /// Merges the other message of the same type into this message
    fn merge_with(&mut self, other: &Self);
}

pub trait Message : MessageLite {
    fn descriptor(&self) -> &reflect::Descriptor;
}

pub trait GeneratedMessage : GeneratedLiteMessage {
    fn descriptor() -> &'static reflect::Descriptor;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
