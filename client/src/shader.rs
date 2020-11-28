use std::convert::From;
use wgpu;

pub struct ShaderCompiler {
    compiler: shaderc::Compiler,
}

impl ShaderCompiler {
    pub fn new() -> Self {
        let compiler = shaderc::Compiler::new().unwrap();
        Self { compiler }
    }

    pub fn compile(
        &mut self,
        path: &str,
        shader_kind: shaderc::ShaderKind,
    ) -> wgpu::ShaderModuleSource {
        println!("Compiling shader with path [{}]", path);

        let file_content =
            std::fs::read_to_string(path).expect(&format!("Unsuccessful file reading [{}].", path));

        let artifact: shaderc::CompilationArtifact = self
            .compiler
            .compile_into_spirv(&file_content, shader_kind, "shader.glsl", "main", None)
            .unwrap();

        let binary = artifact.as_binary();

        wgpu::ShaderModuleSource::SpirV(std::borrow::Cow::Owned(Vec::from(&binary[..])))
    }
}
