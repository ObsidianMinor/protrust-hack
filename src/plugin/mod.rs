/// The original generated code for plugin.proto
pub use crate::generated::google_protobuf_compiler_plugin_proto as proto;

/// A module containing functions plugins can use for standard naming conventions
pub mod names;

use crate::reflect::{DescriptorPool, FileDescriptor};
use crate::{CodedMessage, LiteMessage};
use proto::code_generator_response::File;
use std::error::Error;

pub type Result<T = ()> = std::result::Result<T, Box<dyn Error>>;

/// The context for a code generation request
pub struct Context<'a> {
    request: &'a proto::CodeGeneratorRequest,
    pool: DescriptorPool<'a>,
    files: Vec<(String, String)>,
}

impl<'a> Context<'a> {
    /// Gets the compiler version that created this invocation
    pub fn version(&self) -> Option<&proto::Version> {
        self.request.compiler_version()
    }

    pub fn parameter(&self) -> &str {
        self.request.parameter()
    }

    pub fn parse_parameter(&self) -> Option<impl Iterator<Item = (&str, Option<&str>)>> {
        if self.request.has_parameter() {
            Some(self.request.parameter().split(',').map(|s| {
                let mut iter = s.splitn(2, '=');
                (
                    iter.next().expect("splitn returns at least one item"),
                    iter.next(),
                )
            }))
        } else {
            None
        }
    }

    pub fn input_files(&self) -> impl Iterator<Item = &FileDescriptor<'a>> {
        self.request.file_to_generate().iter().map(move |s| {
            self.pool
                .find_file_by_name(s)
                .expect("file_to_generate was not found in context pool")
        })
    }

    pub fn add_output_file(&mut self, path: String, stream: String) {
        self.files.push((path, stream))
    }
}

impl<'a> From<&'a proto::CodeGeneratorRequest> for Context<'a> {
    fn from(request: &'a proto::CodeGeneratorRequest) -> Self {
        Context {
            request,
            pool: DescriptorPool::build_from_files(request.proto_file()),
            files: Vec::new(),
        }
    }
}

impl From<Context<'_>> for proto::CodeGeneratorResponse {
    fn from(context: Context<'_>) -> Self {
        let mut response = Self::new();
        for (path, stream) in context.files {
            let mut file = File::new();
            file.set_name(path);
            file.set_content(stream);
            response.file_mut().push(file);
        }
        response
    }
}

/// A main method for invoking plugins. Uses stdin and stdout for input and output
pub fn main<T: FnOnce(&mut Context) -> Result>(generator: T) -> Result {
    let stdin = std::io::stdin();
    let mut stdin_lock = stdin.lock();

    let stdout = std::io::stdout();
    let mut stdout_lock = stdout.lock();

    main_io(generator, &mut stdin_lock, &mut stdout_lock)
}

/// A main method for invoking plugins with user defined inputs and outputs
pub fn main_io<T: FnOnce(&mut Context) -> Result>(
    generator: T,
    input: &mut dyn std::io::Read,
    output: &mut dyn std::io::Write,
) -> Result {
    let request = LiteMessage::read_new(input)?;
    let response = main_request(generator, &request)?;
    response.write(output)?;
    Ok(())
}

/// A main method for invoking plugins with a user defined code generator request
pub fn main_request<T: FnOnce(&mut Context) -> Result>(
    generator: T,
    input: &proto::CodeGeneratorRequest,
) -> Result<proto::CodeGeneratorResponse> {
    let mut context = Context::from(input);
    generator(&mut context)?;
    Ok(proto::CodeGeneratorResponse::from(context))
}
