const CARRY_FLAG: u8 = 1 << 0;
const ZERO_FLAG: u8 = 1 << 1;
const INTERRUPT_DISABLE_FLAG: u8 = 1 << 2;
const DECIMAL_FLAG: u8 = 1 << 3;
const OVERFLOW_FLAG: u8 = 1 << 6;
const NEGATIVE_FLAG: u8 = 1 << 7;

pub struct Cpu {
  pub program_counter: u16,
  pub stack_pointer: u8,
  pub accumulator: u8,
  pub register_x: u8,
  pub processor_status: u8,
}

impl Cpu {
  pub fn new() -> Self {
    Cpu {
      program_counter: 0,
      stack_pointer: 0,
      accumulator: 0,
      register_x: 0,
      processor_status: 0,
    }
  }

  fn lda(&mut self, value: u8) {
    self.accumulator = value;
    self.update_negative_and_zero_flags(self.accumulator);
  }

  fn tax(&mut self) {
    self.register_x = self.accumulator;
    self.update_negative_and_zero_flags(self.register_x);
  }

  fn inx(&mut self) {
    self.register_x = self.register_x.wrapping_add(1);
    self.update_negative_and_zero_flags(self.register_x);
  }

  fn update_negative_and_zero_flags(&mut self, result: u8) {
    self.udpate_zero_flag(result);
    self.update_negative_flag(result);
  }

  pub fn update_negative_flag(&mut self, last_operation: u8) {
    if last_operation & 0x80 == 0x80 {
      self.processor_status |= NEGATIVE_FLAG;
    } else {
      self.processor_status &= 0xFF - NEGATIVE_FLAG;
    }
  }

  pub fn udpate_zero_flag(&mut self, last_operation: u8) {
    if last_operation == 0 {
      self.processor_status |= ZERO_FLAG;
    } else {
      self.processor_status &= 0xFF - ZERO_FLAG;
    }
  }

  pub fn run(&mut self, program: Vec<u8>) {
    self.program_counter = 0;
    loop {
      // http://www.obelisk.me.uk/6502/reference.html
      let counter = self.program_counter as usize;
      let opcode = program[counter];
      self.program_counter += 1;

      match opcode {
        0xa9 => {
          let param = opcode;
          self.program_counter += 1;
          self.lda(param);
        }

        0xaa => self.tax(),

        0xe8 => self.inx(),

        0x00 => return,

        _ => todo!(),
      }
    }
  }
}
