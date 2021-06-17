use std::fmt::{Display, Write};

pub struct Tetromino {
    value: u64,
    size: u8,
}

struct Form(u16);

impl Form {
    const fn check_block_exists(form: &Form, size: u8, x: u8, y: u8) -> bool {
        (((form.0 >> (y * size)) << x) & 0b1000) == 0b1000
    }

    const fn get_clockwised_block(from: &Form, size: u8, x: u8, y: u8) -> u16 {
        let exist = Form::check_block_exists(from, size, x, y);
        let mut clockwised = 0u16;

        if exist {
            clockwised |= (0b1000 >> y) >> (x * size);
        }

        if x == size - 1 {
            if y == size - 1 {
                clockwised
            } else {
                clockwised | Form::get_counterclockwised_block(from, size, 0, y + 1)
            }
        } else {
            clockwised | Form::get_counterclockwised_block(from, size, x + 1, y)
        }
    }

    const fn get_counterclockwised_block(from: &Form, size: u8, x: u8, y: u8) -> u16 {
        let exist = Form::check_block_exists(from, size, x, y);
        let mut counterclockwised = 0u16;

        if exist {
            counterclockwised |= (1 << y) << (x * size);
        }

        if x == size - 1 {
            if y == size - 1 {
                counterclockwised
            } else {
                counterclockwised | Form::get_counterclockwised_block(from, size, 0, y + 1)
            }
        } else {
            counterclockwised | Form::get_counterclockwised_block(from, size, x + 1, y)
        }
    }

    const fn clockwised(&self, size: u8) -> Self {
        let mut form = Form(0);
        form.0 = Form::get_clockwised_block(self, size, 0, 0);
        form
    }

    const fn counterclockwised(&self, size: u8) -> Self {
        let mut form = Form(0);
        form.0 = Form::get_counterclockwised_block(self, size, 0, 0);
        form
    }
}

// 4x4 Block data
impl Tetromino {
    pub fn check(&self, x: u8, y: u8) -> bool {
        if x >= self.size || y >= self.size {
            panic!("x and y should be less than or equal to size {}", self.size);
        }

        ((self.value << x) >> (y * self.size)) & 0b1000 == 0b1000
    }

    const fn from_form(mut form: Form, size: u8) -> Self {
        let mut tetromino = Tetromino { size, value: 0 };

        tetromino.value |= form.0 as u64;
        form = form.clockwised(size);
        tetromino.value |= (form.0 as u64) << size * size;
        form = form.clockwised(size);
        tetromino.value |= (form.0 as u64) << size * size * 2;
        form = form.clockwised(size);
        tetromino.value |= (form.0 as u64) << size * size * 3;

        tetromino
    }

    pub fn turn_clockwise(&mut self) {
        let next = (self.value & 0xffff_0000_0000_0000) >> self.size * 12;
        self.value <<= self.size * 4;
        self.value += next;
    }

    pub fn turn_counterclockwise(&mut self) {
        let current = self.value & 0xffff;
        self.value >>= self.size * 4;
        self.value |= current << self.size * 12;
    }

    pub const fn i() -> Self {
        Tetromino::from_form(Form(0x4444), 4)
    }

    pub const fn o() -> Self {
        Tetromino::from_form(Form(0xFFFF), 2)
    }

    pub const fn z() -> Self {
        Tetromino::from_form(Form(0xC600), 4)
    }

    pub const fn s() -> Self {
        Tetromino::from_form(Form(0x3600), 4)
    }

    pub const fn j() -> Self {
        Tetromino::from_form(Form(0x2260), 4)
    }

    pub const fn l() -> Self {
        Tetromino::from_form(Form(0x4460), 4)
    }

    pub const fn t() -> Self {
        Tetromino::from_form(Form(0x01D0), 3)
    }
}

impl Display for Tetromino {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in (0..self.size).rev() {
            for x in 0..self.size {
                f.write_char(if self.check(x, y) { '1' } else { '0' })?;
            }

            if y != 0 {
                f.write_char('\n')?;
            }
        }

        Ok(())
    }
}
