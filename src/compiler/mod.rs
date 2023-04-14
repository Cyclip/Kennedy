use cranelift::codegen::{
    ir::{
        types::{F32, I32, I64},
        AbiParam, // function parameter
        ArgumentPurpose, // function parameter
        ExternalName, // function name
        Function, // function
        InstBuilder, // instruction builder
        Signature, // function signature
        SourceLoc, UserFuncName, // source location
    },

    isa::TargetIsa, // target ISA
    settings, // settings
    isa::CallConv, Context, // calling convention (for function signature)
};

use cranelift::prelude::*;
use cranelift_module::{DataContext, Module};

use crate::error::{
    CompileError,
    CompileResult,
};

use cranelift_jit::{JITBuilder, JITModule};

use target_lexicon::Triple; // target triple (for target ISA)

use crate::ast;

pub struct Compiler {
    /// Basic function builder context. This is the main context that we use to
    /// create Cranelift IR.
    builder_context: FunctionBuilderContext,

    /// Main Cranelift context (contains the module and the ISA).
    ctx: Context,

    /// Data context (like ctx but for data objects, not functions)
    /// Manages the data objects (global variables) in the module.
    data_ctx: DataContext,

    /// The module being compiled
    /// Manages all the JIT'd functions and data objects
    /// Interface for adding/removing functions, and looking up
    /// functions at runtime
    module: JITModule,
}

impl Default for Compiler {
    /// Create a new compiler with the default settings
    fn default() -> Self {
        // Flag builder will be used to create the settings
        let mut flag_builder = settings::builder();
        
        // use_colocated_libcalls: use libcall functions that are colocated with the
        // generated code. Meaning, the libcall functions are generated in the same
        // object file as the generated code (default)
        flag_builder.set("use_colocated_libcalls", "false").unwrap();
        
        // is_pic: generate position-independent code (default)
        // Meaning, the generated code can be loaded at any address
        flag_builder.set("is_pic", "false").unwrap();

        // ISA builder will be used to create the target ISA
        let isa_builder = cranelift_native::builder().unwrap_or_else(|msg| {
            panic!("host machine is not supported: {}", msg);
        });

        // Create the target ISA
        let isa = isa_builder
            .finish(settings::Flags::new(flag_builder))
            .expect("failed to create target ISA");

        // Create the JIT module
        // This is the main interface for adding/removing functions, and looking up
        let builder = JITBuilder::with_isa(isa, cranelift_module::default_libcall_names());
        let module = JITModule::new(builder);

        Self {
            builder_context: FunctionBuilderContext::new(),
            ctx: module.make_context(),
            data_ctx: DataContext::new(),
            module,
        }
    }
}