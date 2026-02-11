#[derive(Debug)]
pub struct Tileset {
    pub name: String,
    pub firstgid: u32,
    pub tilewidth: u32,
    pub tileheight: u32,
    pub columns: u32,
    pub tilecount: u32,
    pub image: image::RgbaImage,
    pub texture: wgpu::Texture,
}

#[derive(Debug, Default)]
pub struct TilesetBuilder {
    pub name: String,
    pub firstgid: u32,
    pub tilewidth: u32,
    pub tileheight: u32,
    pub columns: u32,
    pub tilecount: u32,
    pub image: Option<image::RgbaImage>,
}

impl TilesetBuilder {
    pub fn build(self, renderer: &mut crate::game::renderer::Renderer) -> Tileset {
        let texture = renderer.device.create_texture(&wgpu::TextureDescriptor {
            label: Some(&format!("{} texture", self.name)),
            size: wgpu::Extent3d {
                width: self.image.as_ref().unwrap().width(),
                height: self.image.as_ref().unwrap().height(),
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        renderer.queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            self.image.as_ref().unwrap(),
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * self.image.as_ref().unwrap().width()),
                rows_per_image: Some(self.image.as_ref().unwrap().height()),
            },
            wgpu::Extent3d {
                width: self.image.as_ref().unwrap().width(),
                height: self.image.as_ref().unwrap().height(),
                depth_or_array_layers: 1,
            },
        );

        Tileset {
            name: self.name,
            firstgid: self.firstgid,
            tilewidth: self.tilewidth,
            tileheight: self.tileheight,
            columns: self.columns,
            tilecount: self.tilecount,
            image: self
                .image
                .expect("[ERROR]: Building a tileset from a builder with no image"),
            texture,
        }
    }
}

pub struct Tile {
    texture_view: wgpu::TextureView,
}

impl Tile {}
