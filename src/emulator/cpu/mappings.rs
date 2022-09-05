///////////////////
//  ROM formats  //
///////////////////
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

//////////////////////////////
//  Mapping modes for iNES  //
//////////////////////////////
/* https://www.nesdev.org/wiki/INES */
pub enum Rom_Mapper {
  NROM = 0,
  MMC1 = 1,
  UXROM = 2,
  CNROM = 3,
  MMC3 = 4,
  AXROM = 7,
  MMC2 = 9
}

// Optional fmt implementation for printing.
impl std::fmt::Display for Rom_Mapper {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match *self {
      Rom_Mapper::NROM => write!(f, "NROM"),
      Rom_Mapper::MMC1 => write!(f, "MMC1"),
      Rom_Mapper::UXROM => write!(f, "UXROM"),
      Rom_Mapper::CNROM => write!(f, "CNROM"),
      Rom_Mapper::MMC3 => write!(f, "MMC3"),
      Rom_Mapper::AXROM => write!(f, "AXROM"),
      Rom_Mapper::MMC2 => write!(f, "MMC2"),
    }
  }
}