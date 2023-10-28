use std::sync::atomic::AtomicU64;

use crate::*;

#[derive(Clone, Debug)]
pub enum RenderTargetId {
    Named(String),
    Generated(u64),
}

static GENERATED_RENDER_TARGET_IDS: AtomicU64 = AtomicU64::new(0);
static SHADER_IDS: AtomicU64 = AtomicU64::new(0);

/// Allocates a new render target for later use. If a label is provided
/// it'll be used to set the debug name so graphic debuggers like RenderDoc
/// can display it properly.
pub fn gen_render_target(_label: Option<&str>) -> RenderTargetId {
    // TODO: use the label
    //
    let id = GENERATED_RENDER_TARGET_IDS
        .fetch_add(1, std::sync::atomic::Ordering::SeqCst);

    RenderTargetId::Generated(id)
}

#[derive(Copy, Clone, Debug)]
pub struct ShaderId(u64);

#[derive(Clone, Debug)]
pub enum ShaderError {
    CompileError(String),
}

pub fn create_shader(
    renderer: &WgpuRenderer,
    name: &str,
    source: &str,
    _uniforms_with_defaults: HashMap<&'static str, UniformDesc>,
) -> Result<ShaderId, ShaderError> {
    let id = SHADER_IDS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

    let mut shaders = renderer.shaders.borrow_mut();

    if shaders.contains_key(name) {
        return Err(ShaderError::CompileError(format!(
            "Shader with name '{}' already exists",
            name
        )));
    }

    shaders.insert(name.to_string(), Shader {
        name: format!("{} Shader", name),
        source: source.to_string(),
    });

    Ok(ShaderId(id))
}

#[derive(Clone, Debug)]
pub enum UniformDesc {
    F32(f32),
    Custom { default_data: Vec<u8>, wgsl_decl: String },
}


#[derive(Clone, Debug)]
pub enum Uniform {
    F32(f32),
    Custom(Vec<u8>),
}

pub static CURRENT_SHADER: Lazy<AtomicRefCell<Option<ShaderId>>> =
    Lazy::new(|| AtomicRefCell::new(None));

pub fn set_shader(shader_id: ShaderId) {
    *CURRENT_SHADER.borrow_mut() = Some(shader_id);
}

pub fn set_default_shader() {
    *CURRENT_SHADER.borrow_mut() = None;
}

pub fn set_uniform(_name: &str, _value: Uniform) {}