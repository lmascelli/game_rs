#[path = "./camera.rs"]
mod camera;
#[path = "./layer.rs"]
mod layer;
#[path = "./tileset.rs"]
mod tileset;

use crate::game::error;
use layer::Layer;
use std::str::FromStr;
use tileset::{Tileset, TilesetBuilder};

#[derive(Debug)]
pub struct Level {
    width: u32,
    height: u32,
    tilewidth: u32,
    tileheight: u32,
    tilesets: Vec<Tileset>,
    layers: Vec<Layer>,
}

#[derive(Default, Debug)]
pub struct LevelBuilder {
    width: u32,
    height: u32,
    tilewidth: u32,
    tileheight: u32,
    tileset_builders: Vec<TilesetBuilder>,
    layers: Vec<Layer>,
}

#[derive(Default)]
struct PropertyAttribute {
    pub name: String,
    pub ptype: String,
    pub value: String,
}

impl LevelBuilder {
    const ASSETS_FOLDER: &str = "assets";
    const DEFAULT_TILE_SIZE: f32 = 32.0;

    pub fn load_from_tmx_buffer<T: AsRef<std::path::Path>>(
        path: T,
    ) -> Result<Self, error::GameError> {
        let mut ret = Self {
            width: 0,
            height: 0,
            tilewidth: 0,
            tileheight: 0,
            tileset_builders: vec![],
            layers: vec![],
        };

        let xml_buffer =
            std::io::BufReader::new(std::fs::File::open(path).expect("Failed to open the file"));
        let xml_reader = xml::EventReader::new(xml_buffer);

        let mut parse_stack = std::collections::VecDeque::new();
        let mut parsing_tileset = None;
        let mut parsing_layer = None;
        let mut parsing_layer_has_tile_size = false;
        let mut parsing_data = false;
        for event_res in xml_reader {
            if let Ok(event) = event_res {
                match event {
                    xml::reader::XmlEvent::StartElement {
                        name, attributes, ..
                    } => {
                        parse_stack.push_front(name.local_name.clone());
                        match name.local_name.as_str() {
                            "map" => {
                                for attribute in attributes {
                                    match attribute.name.local_name.as_str() {
                                        "width" => {
                                            ret.width = u32::from_str_radix(&attribute.value, 10)
                                                .expect("Failed to parse the width value");
                                        }
                                        "height" => {
                                            ret.height = u32::from_str_radix(&attribute.value, 10)
                                                .expect("Failed to parse the height value");
                                        }
                                        "tilewidth" => {
                                            ret.tilewidth =
                                                u32::from_str_radix(&attribute.value, 10)
                                                    .expect("Failed to parse the tilewidth value");
                                        }
                                        "tileheight" => {
                                            ret.tileheight =
                                                u32::from_str_radix(&attribute.value, 10)
                                                    .expect("Failed to parse the tileheight value");
                                        }
                                        _ => (),
                                    }
                                }
                            }
                            "tileset" => {
                                parsing_tileset = Some(TilesetBuilder::default());
                                let tileset = parsing_tileset.as_mut().unwrap();
                                for attribute in attributes {
                                    match attribute.name.local_name.as_str() {
                                        "firstgid" => {
                                            tileset.firstgid =
                                                u32::from_str_radix(&attribute.value, 10)
                                                    .expect("Failed to parse the firstgid value");
                                        }
                                        "name" => {
                                            tileset.name = attribute.value.clone();
                                        }
                                        "tilewidth" => {
                                            tileset.tilewidth =
                                                u32::from_str_radix(&attribute.value, 10)
                                                    .expect("Failed to parse the tilewidth value");
                                        }
                                        "tileheight" => {
                                            tileset.tileheight =
                                                u32::from_str_radix(&attribute.value, 10)
                                                    .expect("Failed to parse the tileheight value");
                                        }
                                        "columns" => {
                                            tileset.columns =
                                                u32::from_str_radix(&attribute.value, 10)
                                                    .expect("Failed to parse the columns value");
                                        }
                                        "tilecount" => {
                                            tileset.tilecount =
                                                u32::from_str_radix(&attribute.value, 10)
                                                    .expect("Failed to parse the tilecount value");
                                        }
                                        _ => (),
                                    }
                                }
                            }
                            "layer" => {
                                parsing_layer = Some(Layer::default());
                                let layer = parsing_layer.as_mut().unwrap();
                                for attribute in attributes {
                                    match attribute.name.local_name.as_str() {
                                        "id" => {
                                            layer.order = u32::from_str_radix(&attribute.value, 10)
                                                .expect("Failed to parse the id value");
                                        }
                                        "name" => {
                                            layer.name = attribute.value.clone();
                                        }
                                        "width" => {
                                            layer.width = u32::from_str_radix(&attribute.value, 10)
                                                .expect("Failed to parse the width value");
                                        }
                                        "height" => {
                                            layer.height =
                                                u32::from_str_radix(&attribute.value, 10)
                                                    .expect("Failed to parse the height value");
                                        }
                                        _ => (),
                                    }
                                }
                            }
                            "image" => {
                                if parsing_tileset.is_none() {
                                    return Err(error::GameError::LevelTmxImageOutsideTileset);
                                }
                                let tileset = parsing_tileset.as_mut().unwrap();
                                let mut image_source = None;
                                let mut image_width = None;
                                let mut image_height = None;
                                for attribute in attributes {
                                    match attribute.name.local_name.as_str() {
                                        "source" => {
                                            image_source = Some(attribute.value);
                                        }
                                        "width" => {
                                            image_width = Some(
                                                u32::from_str_radix(&attribute.value, 10).expect(
                                                    "Failed to parse the image width value",
                                                ),
                                            );
                                        }
                                        "height" => {
                                            image_height = Some(
                                                u32::from_str_radix(&attribute.value, 10)
                                                    .expect("Failed to parse the height value"),
                                            );
                                        }
                                        _ => (),
                                    }
                                }
                                if image_source.is_none() {
                                    return Err(error::GameError::LevelTxmImageNoSourceProvided);
                                } else {
                                    let mut assets_path =
                                        std::path::PathBuf::from(Self::ASSETS_FOLDER);
                                    assets_path.push(image_source.unwrap());
                                    tileset.image.replace(
                                        image::ImageReader::open(assets_path)
                                            .expect("Failed to open the image source")
                                            .decode()
                                            .expect("Failed to decode the image")
                                            .to_rgba8(),
                                    );
                                }
                            }
                            "data" => {
                                parsing_data = true;
                            }
                            "property" => {
                                let mut property = PropertyAttribute::default();
                                if parsing_layer.is_some() {
                                    for attribute in attributes {
                                        match attribute.name.local_name.as_str() {
                                            "name" => {
                                                property.name = attribute.value;
                                            }
                                            "type" => {
                                                property.ptype = attribute.value;
                                            }
                                            "value" => {
                                                property.value = attribute.value;
                                            }
                                            _ => {
                                                return Err(crate::game::GameError::LevelTmxPropertyUnhandledAttribute);
                                            }
                                        }
                                        match property.name.as_str() {
                                            "tile_size" => {
                                                parsing_layer.as_mut().unwrap().tile_size =
                                                    f32::from_str(&property.value).expect(
                                                        "Failed to parse the tile_size property",
                                                    );
                                                parsing_layer_has_tile_size = true;
                                            }
                                            _ => (),
                                        }
                                    }
                                }
                            }
                            _ => (),
                        }
                    }
                    xml::reader::XmlEvent::EndElement { name } => {
                        if *parse_stack
                            .front()
                            .expect("Closing an element of an empty stack")
                            != name.local_name
                        {
                            return Err(error::GameError::LevelTmxCloseElementFail);
                        } else {
                            match name.local_name.as_str() {
                                "tileset" => {
                                    if parsing_tileset.is_some() {
                                        ret.tileset_builders.push(parsing_tileset.take().unwrap());
                                    } else {
                                        return Err(error::GameError::LevelTmxCloseElementFail);
                                    }
                                }
                                "layer" => {
                                    if parsing_layer.is_some() {
                                        if !parsing_layer_has_tile_size {
                                            parsing_layer.as_mut().unwrap().tile_size =
                                                Self::DEFAULT_TILE_SIZE;
                                        }
                                        ret.layers.push(parsing_layer.take().unwrap());
                                    } else {
                                        return Err(error::GameError::LevelTmxCloseElementFail);
                                    }
                                }
                                "data" => {
                                    parsing_data = false;
                                }
                                _ => (),
                            }
                            parse_stack.pop_front();
                        }
                    }
                    xml::reader::XmlEvent::Characters(data) => {
                        if parsing_data && parsing_layer.is_some() {
                            let layer_data = &mut parsing_layer.as_mut().unwrap().data;
                            // PARSE CSV
                            *layer_data = data
                                .as_str()
                                .lines()
                                .flat_map(|line| line.split(','))
                                .filter(|s| !s.is_empty())
                                .map(|s| {
                                    u32::from_str_radix(s.trim(), 10)
                                        .expect("Failed to parse digits")
                                })
                                .collect();
                        } else {
                            println!("[INFO]: data outside the layer data section {data}");
                        }
                    }
                    _ => {}
                }
            }
        }

        Ok(ret)
    }

    pub fn build(mut self, renderer: &mut crate::game::renderer::Renderer) -> Level {
        self.layers.sort_by(|l1, l2| l1.order.cmp(&l2.order));
        Level {
            width: self.width,
            height: self.height,
            tilewidth: self.tilewidth,
            tileheight: self.tileheight,
            tilesets: self
                .tileset_builders
                .into_iter()
                .map(|b| b.build(renderer))
                .collect(),
            layers: self.layers,
        }
    }
}
