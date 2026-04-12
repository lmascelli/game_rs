/// # Tile
///
/// At the moment a tile is simply something to be drawed. Maybe later it will have
/// some properties (wall, door, walkable or not). So it only has to know what id it
/// has and how it can be drawed. My idea is that all tiles will have the same size
/// fixed and defined as map level configuration so the only information needed to
/// render them is the view on the texture that contains their image. For this
/// reason the only way to generate a tile is from the tileset that contains its
/// image so that it can be constructed with a view in the tileset texture and
/// with the render pipeline already set up.
#[derive(Debug)]
pub struct Tile {
    id: u32,
    render_pipeline: wgpu::RenderPipeline,
    texture_view: wgpu::TextureView,
}

impl Tile {}

/// # Tileset
///
/// A tileset is a big image that contains the content of the tiles. This leads to
/// a single texture stored once in memory. A tile will be a view on a part of this
/// texture so that drawing a tile means drawing a square (two triangles) attaching
/// the texture view on it in the fragment shader.  
/// Because the tiles are identified by an unique id and there may be more than one
/// tileset, each tileset should be able to tell if an id is part of their image,
/// returning `None` if not or a valid `Tile` if so.
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

    pub fn get_tile(&self, tile_id: u32) -> Option<Tile> {
        todo!()
    }
}
