/// The Layer type is designed to encapsulate the tiles structure of the layer.
///
/// Its fields are:
/// - name: This is the unique ID of the layers.
///
/// - order: This field set the order in which the layers are renderer, the lower
///   the order, the earlier the layer is drawed.
///
/// - width, height: The number of tiles in both dimensions.
///
/// - data: The unrolled array, one row after the other, of IDs of the tiles.
#[derive(Default)]
pub struct Layer {
    pub name: String,
    pub order: u32,
    pub width: u32,
    pub height: u32,
    pub tile_size: f32,
    pub data: Vec<u32>,
}

impl std::fmt::Debug for Layer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(f, "Layer: {{")?;
        writeln!(f, "\tname: {},", self.name)?;
        writeln!(f, "\torder: {},", self.order)?;
        writeln!(f, "\twidth: {},", self.width)?;
        writeln!(f, "\theight: {},", self.height)?;
        writeln!(f, "\ttile_size: {},", self.tile_size)?;
        writeln!(f, "\tdata:")?;
        for r in 0..self.height {
            write!(f, "\t\t")?;
            for c in 0..self.width {
                write!(f, "{} ", self.data[(r * self.width + c) as usize])?;
            }
            writeln!(f, "")?;
        }
        write!(f, "}}")?;
        Ok(())
    }
}
