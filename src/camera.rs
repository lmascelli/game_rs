/// The Camera type is designed to handle the drawing of a portion of the level.
/// It is the direct responsible of calling the render function of the objects and
/// tiles inside its boundaries. The camera will also own a drawable rectangle
/// that will be used to render the tiles, applying the drawed tile texture to it.
#[derive(Debug)]
pub struct Camera {
    x: f32,
    y: f32,
    pipeline: wgpu::RenderPipeline,
}

/// CameraRenderData is an helper struct used to pass parameters to the render
/// function of Camera without passing lots to parameters to it.
#[derive(Default)]
pub struct CameraRenderData<'level> {
    pub layers: Vec<&'level crate::game::level::layer::Layer>,
    pub window_size: (f32, f32),    
}

impl Camera {
    pub fn new(
        x_pos: f32, y_pos: f32,
        renderer: &mut crate::game::renderer::Renderer,
    ) -> Self {
        let camera_pipeline_layout = renderer.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Camera Pipeline Layout"),
            bind_group_layouts: &[],
            immediate_size: 0,
        });

        let camera_shader = renderer.shaders.get("tile")
            .expect("[ERROR]: Failed to find the \"tile\" shader used for camera");
        
        let pipeline = renderer.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Camera Render Pipeline"),
            layout: Some(&camera_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &camera_shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &camera_shader,
                entry_point: Some("fs_main"),
                targets: &[],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview_mask: None,
            cache: None,
        });
        
        Self {
            x: x_pos,
            y: y_pos,
            pipeline,
        }
    }
    
    pub fn render(
        &self,
        renderer: &mut crate::game::renderer::Renderer,
        camera_render_data: CameraRenderData,
    ) {
        for layer in camera_render_data.layers {
            // 1. find the coordinates of the boundaries of the viewport in the layer
            //    the layer size is expressed in tiles, so each tile must be scaled for the
            //    tile_size. Round up these coordinates to exceed the screen when rendering
            let viewport_left_boundary = self.x - camera_render_data.window_size.0 / 2.0;
            let viewport_right_boundary = self.x + camera_render_data.window_size.0 / 2.0;
            let viewport_up_boundary = self.y - camera_render_data.window_size.1 / 2.0;
            let viewport_down_boundary = self.y + camera_render_data.window_size.1 / 2.0;

            let layer_left_boundary = {
                let tmp = f32::floor(viewport_left_boundary / layer.tile_size as f32);
                if tmp <= 0.0 { 0 } else { tmp as u32 }
            };
            let layer_right_boundary = {
                let tmp = f32::ceil(viewport_right_boundary / layer.tile_size as f32);
                if tmp >= layer.width as f32 {
                    layer.width
                } else {
                    tmp as u32
                }
            };
            let layer_up_boundary = {
                let tmp = f32::floor(viewport_up_boundary / layer.tile_size as f32);
                if tmp <= 0.0 { 0 } else { tmp as u32 }
            };
            let layer_down_boundary = {
                let tmp = f32::ceil(viewport_down_boundary / layer.tile_size as f32);
                if tmp >= layer.height as f32 {
                    layer.height
                } else {
                    tmp as u32
                }
            };

            // 2. loop the tiles in these coordinates and render them
            // To render a tile a assume it will need 4 values:
            // (x pos, y pos, width, height) and a reference to the renderer
            for c in layer_left_boundary..layer_right_boundary {
                for r in layer_up_boundary..layer_down_boundary {
                    let tile_render_data = (
                        c as f32 * layer.tile_size - self.x
                            + camera_render_data.window_size.0 / 2.0,
                        r as f32 * layer.tile_size - self.y
                            + camera_render_data.window_size.1 / 2.0,
                        layer.tile_size,
                        layer.tile_size,
                    );

                    // Here we must find what tile there is in this position
                }
            }
        }
    }
}
