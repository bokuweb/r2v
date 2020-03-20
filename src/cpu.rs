use crate::instruction::*;

use byteorder::{ByteOrder, LittleEndian};

const RAM_SIZE: usize = 4096;

pub(crate) struct Cpu {
    x: [i32; 32],
    pc: u32,
    csr: [u32; 1024],
    ram: [u8; RAM_SIZE],
}

impl Default for Cpu {
    fn default() -> Self {
        Cpu {
            x: [0; 32],
            pc: 0,
            csr: [0; 1024],
            ram: [0; RAM_SIZE],
        }
    }
}

impl Cpu {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn init(&mut self, pc: u32, data: &[u8]) {
        for (i, d) in data.iter().enumerate() {
            self.ram[i] = *d;
        }
        self.pc = pc;
    }

    pub fn tick(&mut self) -> Instruction {
        let data = self.fetch();
        let inst = self.decode(data);
        self.exec(inst);
        inst
    }

    fn fetch(&mut self) -> u32 {
        let word = self.read_word(self.pc);
        self.pc = self.pc.wrapping_add(4);
        word
    }

    fn exec(&mut self, inst: Instruction) {
        match inst {
            Instruction::LUI(i) => self.x[i.rd()] = i.imm() as i32,
            Instruction::AUIPC(i) => self.x[i.rd()] = self.pc.wrapping_sub(4).wrapping_add(i.imm()) as i32,
            Instruction::JAL(i) => {
                self.x[i.rd()] = self.pc as i32;
                self.pc = self.pc.wrapping_sub(4).wrapping_add(i.imm());
            }
            Instruction::JALR(i) => {
                self.x[i.rd()] = self.pc as i32;
                self.pc = (self.x[i.rs1()] as u32).wrapping_add(i.imm() as u32);
            }
            Instruction::BEQ(i) if self.x[i.rs1()] == self.x[i.rs2()] => self.pc = self.pc.wrapping_sub(4).wrapping_add(i.imm()),
            Instruction::BNE(i) if self.x[i.rs1()] != self.x[i.rs2()] => self.pc = self.pc.wrapping_sub(4).wrapping_add(i.imm()),
            Instruction::BLT(i) if self.x[i.rs1()] < self.x[i.rs2()] => self.pc = self.pc.wrapping_sub(4).wrapping_add(i.imm()),
            Instruction::BGE(i) if self.x[i.rs1()] >= self.x[i.rs2()] => self.pc = self.pc.wrapping_sub(4).wrapping_add(i.imm()),
            Instruction::BLTU(i) if (self.x[i.rs1()] as u32) < (self.x[i.rs2()] as u32) => self.pc = self.pc.wrapping_sub(4).wrapping_add(i.imm()),
            Instruction::BGEU(i) if (self.x[i.rs1()] as u32) >= (self.x[i.rs2()] as u32) => self.pc = self.pc.wrapping_sub(4).wrapping_add(i.imm()),
            Instruction::LB(i) => self.x[i.rd()] = self.read_byte(self.x[i.rs1()].wrapping_add(i.imm()) as u32) as i32,
            Instruction::LH(i) => self.x[i.rd()] = self.read_halfword(self.x[i.rs1()].wrapping_add(i.imm()) as u32) as i32,
            Instruction::LW(i) => self.x[i.rd()] = self.read_word(self.x[i.rs1()].wrapping_add(i.imm()) as u32) as i32,
            Instruction::LBU(i) => self.x[i.rd()] = self.read_byte(self.x[i.rs1()].wrapping_add(i.imm()) as u32) as i32,
            Instruction::LHU(i) => self.x[i.rd()] = self.read_halfword(self.x[i.rs1()].wrapping_add(i.imm() as i32) as u32) as i32,
            Instruction::SB(i) => self.write_byte(self.x[i.rs1()].wrapping_add(i.imm() as i32) as u32, self.x[i.rs2()] as u8),
            Instruction::SH(i) => self.write_halfword(self.x[i.rs1()].wrapping_add(i.imm() as i32) as u32, self.x[i.rs2()] as u16),
            Instruction::SW(i) => self.write_word(self.x[i.rs1()].wrapping_add(i.imm() as i32) as u32, self.x[i.rs2()] as u32),
            Instruction::ADDI(i) => self.x[i.rd()] = self.x[i.rs1()].wrapping_add(i.imm()),
            Instruction::SLTI(i) => self.x[i.rd()] = self.x[i.rs1()] << (i.imm() & 0x1f) as u32,
            Instruction::XORI(i) => self.x[i.rd()] = self.x[i.rs1()] ^ i.imm(),
            Instruction::ORI(i) => self.x[i.rd()] = self.x[i.rs1()] | i.imm(),
            Instruction::ANDI(i) => self.x[i.rd()] = self.x[i.rs1()] & i.imm(),
            Instruction::SLLI(i) => self.x[i.rd()] = self.x[i.rs1()] << ((i.imm() & 0x1f) as u32),
            Instruction::SRLI(i) => self.x[i.rd()] = self.x[i.rs1()] >> ((i.imm() & 0x1f) as u32),
            Instruction::ADD(i) => self.x[i.rd()] = self.x[i.rs1()].wrapping_add(self.x[i.rs2()]),
            Instruction::SUB(i) => self.x[i.rd()] = self.x[i.rs1()].wrapping_sub(self.x[i.rs2()]),
            Instruction::SLL(i) => self.x[i.rd()] = self.x[i.rs1()].wrapping_shl(self.x[i.rs2()] as u32),
            Instruction::SLT(i) => self.x[i.rd()] = (self.x[i.rs1()] < self.x[i.rs2()]) as i32,
            Instruction::SLTU(i) => self.x[i.rd()] = ((self.x[i.rs1()]) < (self.x[i.rs2()])) as i32,
            Instruction::XOR(i) => self.x[i.rd()] = self.x[i.rs1()] ^ self.x[i.rs2()],
            Instruction::SRL(i) => self.x[i.rd()] = (self.x[i.rs1()]).wrapping_shr(self.x[i.rs2()] as u32),
            Instruction::SRA(i) => self.x[i.rd()] = self.x[i.rs1()].wrapping_shr(self.x[i.rs2()] as u32),
            Instruction::OR(i) => self.x[i.rd()] = self.x[i.rs1()] | self.x[i.rs2()],
            Instruction::AND(i) => self.x[i.rd()] = self.x[i.rs1()] & self.x[i.rs2()],
            Instruction::FENCE(_i) => unimplemented!("I don't know what should I do :sob"),
            Instruction::FENCEI(_i) => unimplemented!("I don't know what should I do :sob"),
            Instruction::ECALL(_i) => unimplemented!("TODO: Implement later"),
            Instruction::EBREAK(_i) => unimplemented!("TODO: Implement later"),
            Instruction::CSRRW(i) => {
                self.x[i.rd()] = self.csr[i.csr()] as i32;
                self.csr[i.csr()] = self.x[i.rs1()] as u32;
                // We need to store 0 to x0
                // This is because csrw extend to `csrrw x0 csr rs1.
                self.x[0] = 0;
            }
            Instruction::CSRRS(i) => {
                self.x[i.rd()] = self.csr[i.csr()] as i32;
                self.csr[i.csr()] = (self.x[i.rd()] | self.x[i.rs1()]) as u32;
                // We need to store 0 to x0
                // This is because csrs extend to `csrrs x0 csr rs1.
                self.x[0] = 0;
            }
            Instruction::CSRRC(i) => {
                self.x[i.rd()] = self.csr[i.csr()] as i32;
                self.csr[i.csr()] = (self.x[i.rd()] & self.x[i.rs1()]) as u32;
                // We need to store 0 to x0
                // This is because csrc extend to `csrrc x0 csr rs1.
                self.x[0] = 0;
            }
            Instruction::CSRRWI(i) => {
                self.x[i.rd()] = self.csr[i.csr()] as i32;
                self.csr[i.csr()] = i.rs1() as u32;
                // We need to store 0 to x0
                // This is because csrwi extend to `csrrwi x0 csr zimm.
                self.x[0] = 0;
            }
            Instruction::CSRRSI(i) => {
                self.x[i.rd()] = self.csr[i.csr()] as i32;
                self.csr[i.csr()] = i.rs1() as u32 | i.imm() as u32;
                // We need to store 0 to x0
                // This is because csrsi extend to `csrrsi x0 csr zimm.
                self.x[0] = 0;
            }
            Instruction::CSRRCI(i) => {
                self.x[i.rd()] = self.csr[i.csr()] as i32;
                self.csr[i.csr()] = i.rs1() as u32 & i.imm() as u32;
                // We need to store 0 to x0
                // This is because csrci extend to `csrrci x0 csr zimm.
                self.x[0] = 0;
            }
            _ => unimplemented!(),
        };
    }

    fn read_byte(&mut self, addr: u32) -> u8 {
        self.ram[addr as usize]
    }

    fn read_halfword(&self, addr: u32) -> u16 {
        LittleEndian::read_u16(&self.ram[(addr as usize)..])
    }

    fn read_word(&self, addr: u32) -> u32 {
        LittleEndian::read_u32(&self.ram[(addr as usize)..])
    }

    fn write_byte(&mut self, addr: u32, data: u8) {
        self.ram[addr as usize] = data;
    }

    fn write_halfword(&mut self, addr: u32, data: u16) {
        LittleEndian::write_u16(&mut self.ram[(addr as usize)..], data);
    }

    fn write_word(&mut self, addr: u32, data: u32) {
        LittleEndian::write_u32(&mut self.ram[(addr as usize)..], data);
    }

    fn decode(&self, d: u32) -> Instruction {
        let i = Common(d);
        match i.opcode() {
            0b011_0111 => Instruction::LUI(U(d)),
            0b001_0111 => Instruction::AUIPC(U(d)),
            0b110_1111 => Instruction::JAL(J(d)),
            0b110_0111 => Instruction::JALR(I(d)),
            0b110_0011 if i.funct3() == 0b000 => Instruction::BEQ(B(d)),
            0b110_0011 if i.funct3() == 0b001 => Instruction::BNE(B(d)),
            0b110_0011 if i.funct3() == 0b100 => Instruction::BLT(B(d)),
            0b110_0011 if i.funct3() == 0b101 => Instruction::BGE(B(d)),
            0b110_0011 if i.funct3() == 0b110 => Instruction::BLTU(B(d)),
            0b110_0011 if i.funct3() == 0b111 => Instruction::BGEU(B(d)),
            0b000_0011 if i.funct3() == 0b000 => Instruction::LB(I(d)),
            0b000_0011 if i.funct3() == 0b001 => Instruction::LH(I(d)),
            0b000_0011 if i.funct3() == 0b010 => Instruction::LW(I(d)),
            0b000_0011 if i.funct3() == 0b100 => Instruction::LBU(I(d)),
            0b000_0011 if i.funct3() == 0b101 => Instruction::LHU(I(d)),
            0b010_0011 if i.funct3() == 0b000 => Instruction::SB(S(d)),
            0b010_0011 if i.funct3() == 0b001 => Instruction::SH(S(d)),
            0b010_0011 if i.funct3() == 0b010 => Instruction::SW(S(d)),
            0b001_0011 if i.funct3() == 0b000 => Instruction::ADDI(I(d)),
            0b001_0011 if i.funct3() == 0b010 => Instruction::SLTI(I(d)),
            0b001_0011 if i.funct3() == 0b100 => Instruction::XORI(I(d)),
            0b001_0011 if i.funct3() == 0b110 => Instruction::ORI(I(d)),
            0b001_0011 if i.funct3() == 0b111 => Instruction::ANDI(I(d)),
            0b001_0011 if i.funct7() == 0b000_0000 && i.funct3() == 0b001 => Instruction::SLLI(I(d)),
            0b001_0011 if i.funct7() == 0b000_0000 && i.funct3() == 0b101 => Instruction::SRLI(I(d)),
            0b011_0011 if i.funct7() == 0b000_0000 && i.funct3() == 0b000 => Instruction::ADD(R(d)),
            0b011_0011 if i.funct7() == 0b010_0000 && i.funct3() == 0b000 => Instruction::SUB(R(d)),
            0b011_0011 if i.funct7() == 0b000_0000 && i.funct3() == 0b001 => Instruction::SLL(R(d)),
            0b011_0011 if i.funct7() == 0b000_0000 && i.funct3() == 0b010 => Instruction::SLT(R(d)),
            0b011_0011 if i.funct7() == 0b000_0000 && i.funct3() == 0b011 => Instruction::SLTU(R(d)),
            0b011_0011 if i.funct7() == 0b000_0000 && i.funct3() == 0b100 => Instruction::XOR(R(d)),
            0b011_0011 if i.funct7() == 0b000_0000 && i.funct3() == 0b101 => Instruction::SRL(R(d)),
            0b011_0011 if i.funct7() == 0b010_0000 && i.funct3() == 0b101 => Instruction::SRA(R(d)),
            0b011_0011 if i.funct7() == 0b000_0000 && i.funct3() == 0b110 => Instruction::OR(R(d)),
            0b011_0011 if i.funct7() == 0b000_0000 && i.funct3() == 0b111 => Instruction::AND(R(d)),
            0b000_1111 if i.funct3() == 0b000 => Instruction::FENCE(I(d)),
            0b000_1111 if i.funct3() == 0b001 => Instruction::FENCEI(I(d)),
            0b111_0011 if i.funct7() == 0b000_0000 && i.funct3() == 0b000 => Instruction::ECALL(I(d)),
            0b111_0011 if i.funct7() == 0b000_0001 && i.funct3() == 0b000 => Instruction::EBREAK(I(d)),
            0b111_0011 if i.funct3() == 0b001 => Instruction::CSRRW(I(d)),
            0b111_0011 if i.funct3() == 0b010 => Instruction::CSRRS(I(d)),
            0b111_0011 if i.funct3() == 0b011 => Instruction::CSRRC(I(d)),
            0b111_0011 if i.funct3() == 0b101 => Instruction::CSRRWI(I(d)),
            0b111_0011 if i.funct3() == 0b110 => Instruction::CSRRSI(I(d)),
            0b111_0011 if i.funct3() == 0b111 => Instruction::CSRRCI(I(d)),
            _ => unimplemented!(),
        }
    }
}
