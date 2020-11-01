use shaderc;
use std::borrow::Cow;
use std::fs;
use wgpu::ShaderModuleSource;

pub struct SpirV {
    artifact: shaderc::CompilationArtifact,
}

impl SpirV {
    pub fn get_module_source(&self) -> ShaderModuleSource {
        let binary_cow = Cow::from(self.artifact.as_binary());
        ShaderModuleSource::SpirV(binary_cow)
    }
}

pub struct ShaderCompiler {
    compiler: shaderc::Compiler,
}

impl ShaderCompiler {
    pub fn new() -> ShaderCompiler {
        let compiler = shaderc::Compiler::new().unwrap();
        ShaderCompiler { compiler }
    }

    pub fn compile_vertex(&mut self, path: &str) -> SpirV {
        self.compile(path, shaderc::ShaderKind::Vertex)
    }

    pub fn compile_fragment(&mut self, path: &str) -> SpirV {
        self.compile(path, shaderc::ShaderKind::Fragment)
    }

    fn compile(&mut self, path: &str, shader_kind: shaderc::ShaderKind) -> SpirV {
        let file_content =
            fs::read_to_string(path).expect(&format!("Unsuccessful file reading [{}].", path));

        let artifact: shaderc::CompilationArtifact = self
            .compiler
            .compile_into_spirv(&file_content, shader_kind, "shader.glsl", "main", None)
            .unwrap();

        SpirV { artifact }
    }
}
