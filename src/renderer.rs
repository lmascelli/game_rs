use std::collections::HashMap;

pub struct Renderer {
    surface: wgpu::Surface<'static>,
    configuration: wgpu::SurfaceConfiguration,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub shaders: HashMap<String, wgpu::ShaderModule>,
}

/// # Renderer
///
/// Owner of the renderer resources.
///
impl Renderer {
    const SHADERS_FOLDER: &str = "shaders";

    pub async fn new(
        window: &winit::window::Window,
        config: &crate::game::AppConfigManager,
    ) -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            flags: wgpu::InstanceFlags::DEBUG | wgpu::InstanceFlags::VALIDATION,
            memory_budget_thresholds: wgpu::MemoryBudgetThresholds::default(),
            backend_options: wgpu::BackendOptions::default(),
            display: Some(Box::new(unsafe {
                &*(window as *const winit::window::Window)
            })),
        });

        let surface = instance
            .create_surface(unsafe { &*(window as *const winit::window::Window) })
            .expect("[ERROR]: Failed to create the surface");

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::LowPower,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .expect("[ERROR]: Failed to request the adapter");

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("Device"),
                ..Default::default()
            })
            .await
            .expect("[ERROR]: Failed to request the device");

        let mut shaders = HashMap::new();
        let shaders_path = std::path::Path::new(Self::SHADERS_FOLDER);
        if shaders_path.exists() && shaders_path.is_dir() {
            for entry in shaders_path
                .read_dir()
                .expect("[ERROR]: failed reading shaders folder")
            {
                if let Ok(dir_entry) = entry {
                    use std::io::Read;
                    let mut shader_source = String::new();
                    std::fs::File::open(dir_entry.path())
                        .expect("[ERROR]: Failed to open shader file")
                        .read_to_string(&mut shader_source)
                        .expect("[ERROR]: Failed to read the shader into memory");
                    let shader_name = dir_entry
                        .path()
                        .as_path()
                        .file_prefix()
                        .expect("[ERROR]: Failed to get the prefix of the shader path")
                        .to_str()
                        .expect("[ERROR]: Failed to get the shader file_name str")
                        .to_string();
                    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                        label: Some(&shader_name),
                        source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::from(shader_source)),
                    });
                    shaders.insert(
                        shader_name,
                        shader,
                    );
                }
            }
        } else {
            eprintln!("[ERROR]: shaders folder not found");
        }

        let capabilities = surface.get_capabilities(&adapter);
        let configuration = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: capabilities
                .formats
                .iter()
                .filter(|f| f.is_srgb())
                .next()
                .unwrap_or(&capabilities.formats[0])
                .clone(),
            width: window.inner_size().width,
            height: window.inner_size().height,
            present_mode: wgpu::PresentMode::Fifo,
            desired_maximum_frame_latency: 2,
            alpha_mode: capabilities.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&device, &configuration);

        Self {
            surface,
            configuration,
            device,
            queue,
            shaders,
        }
    }

    pub fn render(&mut self, winsize: winit::dpi::PhysicalSize<u32>, level: &crate::game::Level) {
        let current_texture = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(texture) => texture,
            wgpu::CurrentSurfaceTexture::Suboptimal(texture) => {
                self.surface.configure(&self.device, &self.configuration);
                if let wgpu::CurrentSurfaceTexture::Success(texture) =
                    self.surface.get_current_texture()
                {
                    texture
                } else {
                    panic!(
                        "[ERROR] Two consecutive configuration of surface failed during rendering"
                    );
                }
            }
            _ => {
                // Probably lost a frame for some reason. I've noticed it happens after waking the
                // pc from a sleep. Just skip this render call.
                return;
            }
        };

        let view = current_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor {
                label: Some("Suface Texture View"),
                format: Some(self.configuration.format),
                ..Default::default()
            });

        let mut command_encoder =
            self.device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Command Encoder"),
                });

        {
            let render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("RenderPass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 18.0 / 255.0,
                            g: 18.0 / 255.0,
                            b: 18.0 / 255.0,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
        }

        level.render(self, winsize);
        self.queue.submit([command_encoder.finish()]);
        current_texture.present();
    }
}
