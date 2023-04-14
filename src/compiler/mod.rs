pub mod symbol_table;

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
use cranelift_module::{DataContext, Module, Linkage};

use crate::error::{
    CompileError,
    CompileResult,
};

use cranelift_jit::{JITBuilder, JITModule};

use target_lexicon::Triple; // target triple (for target ISA)

use crate::ast;
use crate::parser::Parser;
use crate::lexer::lex;

pub type Compiled = Result<*const u8, String>;

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


impl Compiler {
    pub fn compile(&mut self, source: &str) -> Compiled {
        let tokens = lex(source.to_string()).unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        for function in ast.functions {
            self.compile_function(function)?;
        };
    }

    /// Compile and return a function
    fn compile_function(
        &mut self,
        function: ast::Function,
    ) -> Compiled {
        // Create a new function builder
        let mut builder = FunctionBuilder::new(&mut self.ctx.func, &mut self.builder_context);

        // Create the function signature
        let mut sig = Signature::new(CallConv::SystemV);

        // Add parameters
        for param in function.params.params {
            let ty = match param.param_type {
                ast::Type::Int => I64,
                ast::Type::Float => F32,
                _ => panic!("Unsupported type"),
            };

            sig.params.push(AbiParam::new(ty));
        };

        // Add return type
        match function.return_type {
            ast::Type::Int => sig.returns.push(AbiParam::new(I64)),
            ast::Type::Float => sig.returns.push(AbiParam::new(F32)),
            ast::Type::Null => (),
            _ => panic!("Unsupported type"),
        };

        // Create the function
        let func_id = self.module.declare_function(
            &function.ident,
            Linkage::Export,
            &sig,
        ).map_err(|e| e.to_string())?;
    
        // Create the function context
        let mut func_ctx = FunctionBuilderContext::new();
        let mut func_builder = FunctionBuilder::new(&mut self.ctx.func, &mut func_ctx);

        // Create the entry block
        // This is the first block that will be executed when the function is called
        let entry_block = func_builder.create_block();

        // Since this is the entry block, add block parameters
        // according to the function parameters
        func_builder.append_block_params_for_function_params(entry_block);

        // Set the insertion point to the entry block
        // Tells the builder to insert instructions at the end of the entry block
        func_builder.switch_to_block(entry_block);

        // Now we can start adding instructions to the entry block
        // First, we need to create a stack frame
        // This is where the function's local variables will be stored
        
    }
}