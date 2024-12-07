mod constructors;
mod consumptors;
mod getters;
mod setters;


use crate::Byte;

//Bit manipulation helpers

#[inline]
const fn get_bit_mask(index: usize) -> Byte {
    1 << (index % 8)
}

#[inline]
const fn get_bit_indeces(index: usize) -> (usize, usize) {
    (index / 8, index % 8)
}

#[inline]
const fn get_bit_in_byte(byte: Byte, index: usize) -> bool {
    byte & get_bit_mask(index) != 0
}

#[inline]
const fn set_bit_in_byte(byte: Byte, index: usize, bit: bool) -> Byte {
    if bit {
        byte | get_bit_mask(index)
    } else {
        byte & !get_bit_mask(index)
    }
}
