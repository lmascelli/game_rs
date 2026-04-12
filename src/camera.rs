/// The Camera type is designed to handle the drawing of a portion of the level.
/// It is the direct responsible of calling the render function of the objects and
/// tiles inside its boundaries.
#[derive(Default, Debug)]
pub struct Camera {
    pub x: f32,
    pub y: f32,
}

/// CameraRenderData is an helper struct used to pass parameters to the render
/// function of Camera without passing lots to parameters to it.
#[derive(Default)]
pub struct CameraRenderData<'level> {
    pub layers: Vec<&'level crate::game::level::layer::Layer>,
    pub window_size: (f32, f32),
}

impl Camera {
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
                }
            }
        }
    }
}
