///////////////////
//  ROM formats  //
///////////////////
#[derive(Debug)]
pub enum Rom_Format {
  INES,
  INES2_0,
}

// Optional fmt implementation for printing.
impl std::fmt::Display for Rom_Format {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match *self {
      Rom_Format::INES => write!(f, "INES"),
      Rom_Format::INES2_0 => write!(f, "INES 2.0"),
    }
  }
}

/////////////////////
//  Mapping modes  //
/////////////////////
#[derive(Debug)]
pub enum rom_mapper {
  NROM,
  MMC1,
  UXROM,
  CNROM,
  MMC3,
  AXROM,
  MMC2,
}