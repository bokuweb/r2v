bitfield! {
  #[derive(Debug, Clone, Copy, PartialEq)]
  pub struct Common(u32);
  pub opcode, _: 6, 0;
  _dummy0, _: 11, 7;
  pub funct3, _: 14, 12;
  _dummy1, _: 19, 15;
  _dummy3, _: 24, 20;
  pub funct7, _: 31, 25;
}

bitfield! {
  #[derive(Debug, Clone, Copy, PartialEq)]
  pub struct R(u32);
  pub opcode, _: 6, 0;
  _rd, _: 11, 7;
  pub funct3, _: 14, 12;
  _rs1, _: 19, 15;
  _rs2, _: 24, 20;
  pub funct7, _: 31, 25;
}

impl R {
  pub fn rd(self) -> usize {
    self._rd() as usize
  }

  pub fn rs1(self) -> usize {
    self._rs1() as usize
  }

  pub fn rs2(self) -> usize {
    self._rs2() as usize
  }
}

bitfield! {
  #[derive(Debug, Clone, Copy, PartialEq)]
  pub struct B(u32);
  pub opcode, _: 6, 0;
  _imm11, _: 7, 7;
  _imm4_1, _: 11, 8;
  pub funct3, _: 14, 12;
  _rs1, _: 19, 15;
  _rs2, _: 24, 20;
  _imm10_5, _: 30, 25;
  _imm12, _: 31, 31;
}

impl B {
  pub fn rs1(self) -> usize {
    self._rs1() as usize
  }

  pub fn rs2(self) -> usize {
    self._rs2() as usize
  }

  pub fn imm(self) -> u32 {
    let imm = match self.0 & 0x8000_0000 {
      0x8000_0000 => 0xffff_f800,
      _ => 0,
    };
    imm | (self._imm12() << 12 | self._imm11() << 11 | self._imm10_5() << 5 | self._imm4_1() << 1)
  }
}

bitfield! {
  #[derive(Debug, Clone, Copy, PartialEq)]
  pub struct S(u32);
  pub opcode, _: 6, 0;
  _imm4_0, _: 11, 7;
  pub funct3, _: 14, 12;
  _rs1, _: 19, 15;
  _rs2, _: 24, 20;
  _imm11_5, _: 31, 25;
}

impl S {
  pub fn rs1(self) -> usize {
    self._rs1() as usize
  }

  pub fn rs2(self) -> usize {
    self._rs2() as usize
  }

  pub fn imm(self) -> i32 {
    (match self.0 & 0x8000_0000 {
      0x8000_0000 => 0xffff_f000,
      _ => 0,
    } | (self._imm11_5() << 5 | self._imm4_0())) as i32
  }
}

bitfield! {
  #[derive(Debug, Clone, Copy, PartialEq)]
  pub struct U(u32);
  pub opcode, _: 6, 0;
  _rd, _: 11, 7;
  _imm31_12, _: 31, 12;
}

impl U {
  pub fn rd(self) -> usize {
    self._rd() as usize
  }

  pub fn imm(self) -> u32 {
    self.0 & 0xffff_f000
  }
}

bitfield! {
  #[derive(Debug, Clone, Copy, PartialEq)]
  pub struct J(u32);
  pub opcode, _: 6, 0;
  _rd, _: 11, 7;
  _imm19_12, _: 19, 12;
  _imm11, _: 20, 20;
  _imm10_1, _: 30, 21;
  _imm20, _: 31, 31;
}

impl J {
  pub fn rd(self) -> usize {
    self._rd() as usize
  }

  pub fn imm(self) -> u32 {
    let base = match self.0 & 0x8000_0000 {
      0x8000_0000 => 0xfff0_0000,
      _ => 0,
    };
    base | (self._imm20() << 20 | self._imm19_12() << 12 | self._imm11() << 11 | self._imm10_1() << 1)
  }
}

bitfield! {
  #[derive(Debug, Clone, Copy, PartialEq)]
  pub struct I(u32);
  pub opcode, _: 6, 0;
  _rd, _: 11, 7;
  pub funct3, _: 14, 12;
  _rs1, _: 19, 15;
  _imm11_0, _: 31, 20;
}

impl I {
  pub fn rd(self) -> usize {
    self._rd() as usize
  }

  pub fn rs1(self) -> usize {
    self._rs1() as usize
  }
  pub fn imm(self) -> i32 {
    (match self.0 & 0x8000_0000 {
      0x8000_0000 => 0xffff_f800,
      _ => 0,
    } | self._imm11_0()) as i32
  }

  pub fn csr(self) -> usize {
    self._imm11_0() as usize
  }
}

#[cfg(test)]
mod tests {

  use super::*;
  #[cfg(test)]
  use pretty_assertions::assert_eq;

  #[test]
  fn test_common_format() {
    let inst: u32 = 0b0000_0000_0011_0100_1010_1000_1001_0011;
    let inst = Common(inst);
    assert_eq!(inst.opcode(), 0b01_0011);
    assert_eq!(inst.funct3(), 0b010);
    assert_eq!(inst.funct7(), 0b000_0000);
  }
}
