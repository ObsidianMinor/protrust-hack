/// A full descriptor
pub struct Descriptor;

/// A minimal descriptor for lite runtimes
pub struct LiteDescriptor;

/// A descriptor containing name information for a message
pub trait DescriptorName {
    /// Gets the name of the message
    fn name(&self) -> &str;
    /// Gets the package name of the message
    fn package(&self) -> &str;
}