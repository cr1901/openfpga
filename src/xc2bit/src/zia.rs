/*
Copyright (c) 2016-2017, Robert Ou <rqou@robertou.com> and contributors
All rights reserved.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice,
   this list of conditions and the following disclaimer.
2. Redistributions in binary form must reproduce the above copyright notice,
   this list of conditions and the following disclaimer in the documentation
   and/or other materials provided with the distribution.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/

//! Contains functions pertaining to the ZIA

use *;

/// Represents one output of the ZIA. The ZIA is divided into rows, and each row can independently select a choice
/// to connect to each function block. This represents one such output (as opposed to all outputs in a given row)
#[derive(Copy, Clone, Default)]
pub struct XC2ZIARowPiece {
    pub selected: XC2ZIAInput,
}

/// Represents one input to the ZIA. The ZIA has inputs from every part of the chip and can additionally output a
/// constant zero or one.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum XC2ZIAInput {
    Macrocell {
        fb: u32,
        ff: u32,
    },
    IBuf {
        ibuf: u32,
    },
    DedicatedInput,
    Zero,
    One,
}

impl Default for XC2ZIAInput {
    /// Returns a "default" ZIA selection, which is a constant one.
    fn default() -> XC2ZIAInput { XC2ZIAInput::One }
}

/// A map of the connections that exist within the ZIA for 32-macrocell parts
pub static ZIA_MAP_32: [[XC2ZIAInput; 6]; INPUTS_PER_ANDTERM] = [
    // Row 0
    [XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 10},
     XC2ZIAInput::IBuf{ibuf: 21},
     XC2ZIAInput::Macrocell{fb: 0, ff: 1},
     XC2ZIAInput::Macrocell{fb: 0, ff: 13},
     XC2ZIAInput::Macrocell{fb: 1, ff: 9}],

    [XC2ZIAInput::IBuf{ibuf: 1},
     XC2ZIAInput::IBuf{ibuf: 11},
     XC2ZIAInput::IBuf{ibuf: 22},
     XC2ZIAInput::Macrocell{fb: 0, ff: 8},
     XC2ZIAInput::Macrocell{fb: 0, ff: 15},
     XC2ZIAInput::Macrocell{fb: 1, ff: 12}],

    [XC2ZIAInput::IBuf{ibuf: 2},
     XC2ZIAInput::IBuf{ibuf: 12},
     XC2ZIAInput::IBuf{ibuf: 29},
     XC2ZIAInput::Macrocell{fb: 0, ff: 2},
     XC2ZIAInput::Macrocell{fb: 1, ff: 4},
     XC2ZIAInput::Macrocell{fb: 1, ff: 11}],

    [XC2ZIAInput::IBuf{ibuf: 3},
     XC2ZIAInput::IBuf{ibuf: 13},
     XC2ZIAInput::IBuf{ibuf: 25},
     XC2ZIAInput::Macrocell{fb: 0, ff: 9},
     XC2ZIAInput::Macrocell{fb: 0, ff: 14},
     XC2ZIAInput::Macrocell{fb: 1, ff: 6}],

    [XC2ZIAInput::IBuf{ibuf: 4},
     XC2ZIAInput::IBuf{ibuf: 14},
     XC2ZIAInput::IBuf{ibuf: 27},
     XC2ZIAInput::Macrocell{fb: 0, ff: 5},
     XC2ZIAInput::Macrocell{fb: 0, ff: 11},
     XC2ZIAInput::Macrocell{fb: 1, ff: 10}],
    // Row 5
    [XC2ZIAInput::IBuf{ibuf: 5},
     XC2ZIAInput::IBuf{ibuf: 15},
     XC2ZIAInput::IBuf{ibuf: 30},
     XC2ZIAInput::Macrocell{fb: 0, ff: 7},
     XC2ZIAInput::Macrocell{fb: 1, ff: 1},
     XC2ZIAInput::Macrocell{fb: 1, ff: 7}],

    [XC2ZIAInput::IBuf{ibuf: 6},
     XC2ZIAInput::DedicatedInput,
     XC2ZIAInput::IBuf{ibuf: 20},
     XC2ZIAInput::Macrocell{fb: 0, ff: 0},
     XC2ZIAInput::Macrocell{fb: 1, ff: 3},
     XC2ZIAInput::Macrocell{fb: 1, ff: 13}],

    [XC2ZIAInput::IBuf{ibuf: 7},
     XC2ZIAInput::IBuf{ibuf: 16},
     XC2ZIAInput::IBuf{ibuf: 26},
     XC2ZIAInput::IBuf{ibuf: 31},
     XC2ZIAInput::Macrocell{fb: 0, ff: 12},
     XC2ZIAInput::Macrocell{fb: 1, ff: 15}],

    [XC2ZIAInput::IBuf{ibuf: 8},
     XC2ZIAInput::IBuf{ibuf: 17},
     XC2ZIAInput::IBuf{ibuf: 24},
     XC2ZIAInput::Macrocell{fb: 0, ff: 6},
     XC2ZIAInput::Macrocell{fb: 0, ff: 10},
     XC2ZIAInput::Macrocell{fb: 1, ff: 8}],

    [XC2ZIAInput::IBuf{ibuf: 9},
     XC2ZIAInput::IBuf{ibuf: 18},
     XC2ZIAInput::IBuf{ibuf: 23},
     XC2ZIAInput::Macrocell{fb: 0, ff: 4},
     XC2ZIAInput::Macrocell{fb: 1, ff: 2},
     XC2ZIAInput::Macrocell{fb: 1, ff: 5}],
    // Row 10
    [XC2ZIAInput::IBuf{ibuf: 7},
     XC2ZIAInput::IBuf{ibuf: 19},
     XC2ZIAInput::IBuf{ibuf: 28},
     XC2ZIAInput::Macrocell{fb: 0, ff: 3},
     XC2ZIAInput::Macrocell{fb: 1, ff: 0},
     XC2ZIAInput::Macrocell{fb: 1, ff: 14}],

    [XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 11},
     XC2ZIAInput::IBuf{ibuf: 22},
     XC2ZIAInput::Macrocell{fb: 0, ff: 2},
     XC2ZIAInput::Macrocell{fb: 0, ff: 14},
     XC2ZIAInput::Macrocell{fb: 1, ff: 10}],

    [XC2ZIAInput::IBuf{ibuf: 1},
     XC2ZIAInput::IBuf{ibuf: 12},
     XC2ZIAInput::IBuf{ibuf: 28},
     XC2ZIAInput::Macrocell{fb: 0, ff: 4},
     XC2ZIAInput::Macrocell{fb: 1, ff: 1},
     XC2ZIAInput::Macrocell{fb: 1, ff: 15}],

    [XC2ZIAInput::IBuf{ibuf: 2},
     XC2ZIAInput::IBuf{ibuf: 18},
     XC2ZIAInput::IBuf{ibuf: 23},
     XC2ZIAInput::Macrocell{fb: 0, ff: 9},
     XC2ZIAInput::Macrocell{fb: 1, ff: 0},
     XC2ZIAInput::Macrocell{fb: 1, ff: 13}],

    [XC2ZIAInput::IBuf{ibuf: 3},
     XC2ZIAInput::IBuf{ibuf: 15},
     XC2ZIAInput::IBuf{ibuf: 30},
     XC2ZIAInput::Macrocell{fb: 0, ff: 3},
     XC2ZIAInput::Macrocell{fb: 0, ff: 11},
     XC2ZIAInput::Macrocell{fb: 1, ff: 12}],
    // Row 15
    [XC2ZIAInput::IBuf{ibuf: 4},
     XC2ZIAInput::IBuf{ibuf: 16},
     XC2ZIAInput::IBuf{ibuf: 21},
     XC2ZIAInput::Macrocell{fb: 0, ff: 0},
     XC2ZIAInput::Macrocell{fb: 0, ff: 15},
     XC2ZIAInput::Macrocell{fb: 1, ff: 7}],

    [XC2ZIAInput::IBuf{ibuf: 5},
     XC2ZIAInput::IBuf{ibuf: 19},
     XC2ZIAInput::IBuf{ibuf: 28},
     XC2ZIAInput::Macrocell{fb: 0, ff: 6},
     XC2ZIAInput::Macrocell{fb: 0, ff: 12},
     XC2ZIAInput::Macrocell{fb: 1, ff: 11}],

    [XC2ZIAInput::IBuf{ibuf: 6},
     XC2ZIAInput::IBuf{ibuf: 10},
     XC2ZIAInput::IBuf{ibuf: 21},
     XC2ZIAInput::Macrocell{fb: 0, ff: 8},
     XC2ZIAInput::Macrocell{fb: 1, ff: 2},
     XC2ZIAInput::Macrocell{fb: 1, ff: 8}],

    [XC2ZIAInput::IBuf{ibuf: 7},
     XC2ZIAInput::DedicatedInput,
     XC2ZIAInput::IBuf{ibuf: 20},
     XC2ZIAInput::Macrocell{fb: 0, ff: 1},
     XC2ZIAInput::Macrocell{fb: 1, ff: 4},
     XC2ZIAInput::Macrocell{fb: 1, ff: 14}],

    [XC2ZIAInput::IBuf{ibuf: 8},
     XC2ZIAInput::IBuf{ibuf: 14},
     XC2ZIAInput::IBuf{ibuf: 27},
     XC2ZIAInput::IBuf{ibuf: 31},
     XC2ZIAInput::Macrocell{fb: 0, ff: 13},
     XC2ZIAInput::Macrocell{fb: 1, ff: 6}],
    // Row 20
    [XC2ZIAInput::IBuf{ibuf: 9},
     XC2ZIAInput::IBuf{ibuf: 13},
     XC2ZIAInput::IBuf{ibuf: 25},
     XC2ZIAInput::Macrocell{fb: 0, ff: 7},
     XC2ZIAInput::Macrocell{fb: 0, ff: 10},
     XC2ZIAInput::Macrocell{fb: 1, ff: 9}],

    [XC2ZIAInput::IBuf{ibuf: 8},
     XC2ZIAInput::IBuf{ibuf: 17},
     XC2ZIAInput::IBuf{ibuf: 24},
     XC2ZIAInput::Macrocell{fb: 0, ff: 5},
     XC2ZIAInput::Macrocell{fb: 1, ff: 3},
     XC2ZIAInput::Macrocell{fb: 1, ff: 5}],

    [XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 12},
     XC2ZIAInput::IBuf{ibuf: 23},
     XC2ZIAInput::Macrocell{fb: 0, ff: 3},
     XC2ZIAInput::Macrocell{fb: 0, ff: 15},
     XC2ZIAInput::Macrocell{fb: 1, ff: 11}],

    [XC2ZIAInput::IBuf{ibuf: 1},
     XC2ZIAInput::IBuf{ibuf: 18},
     XC2ZIAInput::IBuf{ibuf: 25},
     XC2ZIAInput::Macrocell{fb: 0, ff: 6},
     XC2ZIAInput::Macrocell{fb: 1, ff: 4},
     XC2ZIAInput::Macrocell{fb: 1, ff: 5}],

    [XC2ZIAInput::IBuf{ibuf: 2},
     XC2ZIAInput::IBuf{ibuf: 13},
     XC2ZIAInput::IBuf{ibuf: 30},
     XC2ZIAInput::Macrocell{fb: 0, ff: 5},
     XC2ZIAInput::Macrocell{fb: 1, ff: 2},
     XC2ZIAInput::Macrocell{fb: 1, ff: 6}],
    // Row 25
    [XC2ZIAInput::IBuf{ibuf: 3},
     XC2ZIAInput::IBuf{ibuf: 19},
     XC2ZIAInput::IBuf{ibuf: 24},
     XC2ZIAInput::Macrocell{fb: 0, ff: 0},
     XC2ZIAInput::Macrocell{fb: 1, ff: 1},
     XC2ZIAInput::Macrocell{fb: 1, ff: 14}],

    [XC2ZIAInput::IBuf{ibuf: 4},
     XC2ZIAInput::DedicatedInput,
     XC2ZIAInput::IBuf{ibuf: 21},
     XC2ZIAInput::Macrocell{fb: 0, ff: 4},
     XC2ZIAInput::Macrocell{fb: 0, ff: 12},
     XC2ZIAInput::Macrocell{fb: 1, ff: 13}],

    [XC2ZIAInput::IBuf{ibuf: 5},
     XC2ZIAInput::IBuf{ibuf: 17},
     XC2ZIAInput::IBuf{ibuf: 27},
     XC2ZIAInput::Macrocell{fb: 0, ff: 1},
     XC2ZIAInput::Macrocell{fb: 1, ff: 0},
     XC2ZIAInput::Macrocell{fb: 1, ff: 8}],

    [XC2ZIAInput::IBuf{ibuf: 6},
     XC2ZIAInput::IBuf{ibuf: 11},
     XC2ZIAInput::IBuf{ibuf: 29},
     XC2ZIAInput::Macrocell{fb: 0, ff: 7},
     XC2ZIAInput::Macrocell{fb: 0, ff: 13},
     XC2ZIAInput::Macrocell{fb: 1, ff: 12}],

    [XC2ZIAInput::IBuf{ibuf: 7},
     XC2ZIAInput::IBuf{ibuf: 10},
     XC2ZIAInput::IBuf{ibuf: 22},
     XC2ZIAInput::Macrocell{fb: 0, ff: 9},
     XC2ZIAInput::Macrocell{fb: 1, ff: 3},
     XC2ZIAInput::Macrocell{fb: 1, ff: 9}],
    // Row 30
    [XC2ZIAInput::IBuf{ibuf: 8},
     XC2ZIAInput::IBuf{ibuf: 16},
     XC2ZIAInput::IBuf{ibuf: 20},
     XC2ZIAInput::Macrocell{fb: 0, ff: 2},
     XC2ZIAInput::Macrocell{fb: 0, ff: 11},
     XC2ZIAInput::Macrocell{fb: 1, ff: 15}],

    [XC2ZIAInput::IBuf{ibuf: 9},
     XC2ZIAInput::IBuf{ibuf: 15},
     XC2ZIAInput::IBuf{ibuf: 28},
     XC2ZIAInput::IBuf{ibuf: 31},
     XC2ZIAInput::Macrocell{fb: 0, ff: 14},
     XC2ZIAInput::Macrocell{fb: 1, ff: 7}],

    [XC2ZIAInput::IBuf{ibuf: 9},
     XC2ZIAInput::IBuf{ibuf: 14},
     XC2ZIAInput::IBuf{ibuf: 21},
     XC2ZIAInput::Macrocell{fb: 0, ff: 8},
     XC2ZIAInput::Macrocell{fb: 0, ff: 10},
     XC2ZIAInput::Macrocell{fb: 1, ff: 10}],

    [XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 13},
     XC2ZIAInput::IBuf{ibuf: 24},
     XC2ZIAInput::Macrocell{fb: 0, ff: 4},
     XC2ZIAInput::Macrocell{fb: 1, ff: 0},
     XC2ZIAInput::Macrocell{fb: 1, ff: 12}],

    [XC2ZIAInput::IBuf{ibuf: 1},
     XC2ZIAInput::IBuf{ibuf: 15},
     XC2ZIAInput::IBuf{ibuf: 27},
     XC2ZIAInput::Macrocell{fb: 0, ff: 9},
     XC2ZIAInput::Macrocell{fb: 0, ff: 10},
     XC2ZIAInput::Macrocell{fb: 1, ff: 11}],
    // Row 35
    [XC2ZIAInput::IBuf{ibuf: 2},
     XC2ZIAInput::IBuf{ibuf: 19},
     XC2ZIAInput::IBuf{ibuf: 21},
     XC2ZIAInput::Macrocell{fb: 0, ff: 7},
     XC2ZIAInput::Macrocell{fb: 0, ff: 11},
     XC2ZIAInput::Macrocell{fb: 1, ff: 5}],
     
    [XC2ZIAInput::IBuf{ibuf: 3},
     XC2ZIAInput::IBuf{ibuf: 14},
     XC2ZIAInput::IBuf{ibuf: 21},
     XC2ZIAInput::Macrocell{fb: 0, ff: 6},
     XC2ZIAInput::Macrocell{fb: 1, ff: 3},
     XC2ZIAInput::Macrocell{fb: 1, ff: 7}],
     
    [XC2ZIAInput::IBuf{ibuf: 4},
     XC2ZIAInput::IBuf{ibuf: 11},
     XC2ZIAInput::IBuf{ibuf: 25},
     XC2ZIAInput::Macrocell{fb: 0, ff: 1},
     XC2ZIAInput::Macrocell{fb: 1, ff: 2},
     XC2ZIAInput::Macrocell{fb: 1, ff: 15}],
     
    [XC2ZIAInput::IBuf{ibuf: 5},
     XC2ZIAInput::IBuf{ibuf: 16},
     XC2ZIAInput::IBuf{ibuf: 22},
     XC2ZIAInput::Macrocell{fb: 0, ff: 5},
     XC2ZIAInput::Macrocell{fb: 0, ff: 13},
     XC2ZIAInput::Macrocell{fb: 1, ff: 14}],
     
    [XC2ZIAInput::IBuf{ibuf: 6},
     XC2ZIAInput::IBuf{ibuf: 18},
     XC2ZIAInput::IBuf{ibuf: 28},
     XC2ZIAInput::Macrocell{fb: 0, ff: 2},
     XC2ZIAInput::Macrocell{fb: 1, ff: 1},
     XC2ZIAInput::Macrocell{fb: 1, ff: 9}],
];

/// A map of the connections that exist within the ZIA for 64-macrocell parts
pub static ZIA_MAP_64: [[XC2ZIAInput; 12]; INPUTS_PER_ANDTERM] = [
    // Row 0
    [XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0}],

    [XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0}],

    [XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0}],

    [XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0}],

    [XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0}],

    // Row 5
    [XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0}],

    [XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0}],

    [XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0}],

    [XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0}],

    [XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0}],

    // Row 10
    [XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0}],

    [XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0}],

    [XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0}],

    [XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0}],

    [XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0}],

    // Row 15
    [XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0}],

    [XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0}],

    [XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0}],

    [XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0}],

    [XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0}],

    // Row 20
    [XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0}],

    [XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0}],

    [XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0}],

    [XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0}],

    [XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0}],

    // Row 25
    [XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0}],

    [XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0}],

    [XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0}],

    [XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0}],

    [XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0}],

    // Row 30
    [XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0}],

    [XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0}],

    [XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0}],

    [XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0}],

    [XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0}],

    // Row 35
    [XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0}],

    [XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0}],

    [XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0}],

    [XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0}],

    [XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0},
     XC2ZIAInput::IBuf{ibuf: 0}],
];

/// Internal function that reads a piece of the ZIA corresponding to one FB and one row
pub fn read_32_zia_fb_row_logical(fuses: &[bool], block_idx: usize, row_idx: usize)
    -> Result<XC2ZIARowPiece, &'static str> {

    let mut zia_row_fuses = [false; 8];
    
    for i in 0..8 {
        zia_row_fuses[7 - i] = fuses[block_idx + row_idx * 8 + i];
    }

    // 7th bit is active-high unlike the rest
    zia_row_fuses[7] = !zia_row_fuses[7];

    let mut active_bit = 8;
    for i in 0..8 {
        // active low
        if !zia_row_fuses[i] {
            if active_bit != 8 {
                return Err("multiple ZIA inputs selected!");
            }

            active_bit = i;
        }
    }

    if active_bit == 8 {
        // FIXME: Is this an error?
        return Err("no ZIA inputs selected!");
    }

    Ok(XC2ZIARowPiece {
        selected: if active_bit == 6 {
            XC2ZIAInput::Zero
        } else if active_bit == 7 {
            XC2ZIAInput::One
        } else {
            ZIA_MAP_32[row_idx][active_bit]
        }
    })
}

/// Internal function that takes a ZIA row and choice and returns the bit encoding for it
pub fn encode_32_zia_choice(row: u32, choice: XC2ZIAInput) -> Option<[bool; 8]> {
    if choice == XC2ZIAInput::One {
        Some([true, true, true, true, true, true, true, true])
    } else if choice == XC2ZIAInput::Zero {
        Some([true, true, true, true, true, true, false, false])
    } else {
        let mut ret = [true; 8];
        // This bit is active-high unlike the rest
        ret[7] = false;

        let mut found_bit = 8;
        for i in 0..ZIA_MAP_32[row as usize].len() {
            if choice == ZIA_MAP_32[row as usize][i] {
                found_bit = i;
                break;
            }
        }

        if found_bit == 8 {
            // Didn't find it
            return None;
        }

        ret[found_bit] = false;

        Some(ret)
    }
}

const T: bool = true;
const F: bool = false;

/// Internal function that reads a piece of the ZIA corresponding to one FB and one row
pub fn read_64_zia_fb_row_logical(fuses: &[bool], block_idx: usize, row_idx: usize)
    -> Result<XC2ZIARowPiece, &'static str> {

    // This is an ugly workaround for the lack of stable slice patterns
    let zia_row_fuses = (
        fuses[block_idx + row_idx * 16 + 0],
        fuses[block_idx + row_idx * 16 + 1],
        fuses[block_idx + row_idx * 16 + 2],
        fuses[block_idx + row_idx * 16 + 3],
        fuses[block_idx + row_idx * 16 + 4],
        fuses[block_idx + row_idx * 16 + 5],
        fuses[block_idx + row_idx * 16 + 6],
        fuses[block_idx + row_idx * 16 + 7],
        fuses[block_idx + row_idx * 16 + 8],
        fuses[block_idx + row_idx * 16 + 9],
        fuses[block_idx + row_idx * 16 + 10],
        fuses[block_idx + row_idx * 16 + 11],
        fuses[block_idx + row_idx * 16 + 12],
        fuses[block_idx + row_idx * 16 + 13],
        fuses[block_idx + row_idx * 16 + 14],
        fuses[block_idx + row_idx * 16 + 15],
    );

    let selected_input = match zia_row_fuses {
        (T, T, T, T, T, T, T, F, T, T, T, T, F, T, T, F) => ZIA_MAP_64[row_idx][0],
        (T, T, T, T, T, T, T, F, T, T, T, T, F, T, F, T) => ZIA_MAP_64[row_idx][1],
        (T, T, T, T, T, T, T, F, T, T, T, T, F, F, T, T) => ZIA_MAP_64[row_idx][2],
        (T, T, T, T, T, T, T, F, T, T, T, F, F, T, T, T) => ZIA_MAP_64[row_idx][3],
        (T, T, T, T, T, T, T, F, T, T, F, T, F, T, T, T) => ZIA_MAP_64[row_idx][4],
        (T, T, T, T, T, T, T, F, T, F, T, T, F, T, T, T) => ZIA_MAP_64[row_idx][5],
        (T, T, T, F, T, T, F, F, T, T, T, T, T, T, T, T) => ZIA_MAP_64[row_idx][6],
        (T, T, T, F, T, F, T, F, T, T, T, T, T, T, T, T) => ZIA_MAP_64[row_idx][7],
        (T, T, T, F, F, T, T, F, T, T, T, T, T, T, T, T) => ZIA_MAP_64[row_idx][8],
        (T, T, F, F, T, T, T, F, T, T, T, T, T, T, T, T) => ZIA_MAP_64[row_idx][9],
        (T, F, T, F, T, T, T, F, T, T, T, T, T, T, T, T) => ZIA_MAP_64[row_idx][10],
        (F, T, T, F, T, T, T, F, T, T, T, T, T, T, T, T) => ZIA_MAP_64[row_idx][11],
        (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) => XC2ZIAInput::One,
        // TODO: This one isn't certain
        (T, T, T, T, T, T, T, F, F, T, T, T, T, T, T, T) => XC2ZIAInput::Zero,
        _ => return Err("unknown ZIA input choice"),
    };

    Ok(XC2ZIARowPiece {
        selected: selected_input
    })
}

/// Internal function that takes a ZIA row and choice and returns the bit encoding for it
pub fn encode_64_zia_choice(row: u32, choice: XC2ZIAInput) -> Option<[bool; 16]> {
    if choice == XC2ZIAInput::One {
        Some([T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T])
    } else if choice == XC2ZIAInput::Zero {
        // TODO: This one isn't certain
        Some([T, T, T, T, T, T, T, F, F, T, T, T, T, T, T, T])
    } else {
        let mut found_bit = 12;
        for i in 0..ZIA_MAP_64[row as usize].len() {
            if choice == ZIA_MAP_64[row as usize][i] {
                found_bit = i;
                break;
            }
        }

        if found_bit == 12 {
            // Didn't find it
            return None;
        }

        match found_bit {
            0  => Some([T, T, T, T, T, T, T, F, T, T, T, T, F, T, T, F]),
            1  => Some([T, T, T, T, T, T, T, F, T, T, T, T, F, T, F, T]),
            2  => Some([T, T, T, T, T, T, T, F, T, T, T, T, F, F, T, T]),
            3  => Some([T, T, T, T, T, T, T, F, T, T, T, F, F, T, T, T]),
            4  => Some([T, T, T, T, T, T, T, F, T, T, F, T, F, T, T, T]),
            5  => Some([T, T, T, T, T, T, T, F, T, F, T, T, F, T, T, T]),
            6  => Some([T, T, T, F, T, T, F, F, T, T, T, T, T, T, T, T]),
            7  => Some([T, T, T, F, T, F, T, F, T, T, T, T, T, T, T, T]),
            8  => Some([T, T, T, F, F, T, T, F, T, T, T, T, T, T, T, T]),
            9  => Some([T, T, F, F, T, T, T, F, T, T, T, T, T, T, T, T]),
            10 => Some([T, F, T, F, T, T, T, F, T, T, T, T, T, T, T, T]),
            11 => Some([F, T, T, F, T, T, T, F, T, T, T, T, T, T, T, T]),
            _ => unreachable!(),
        }
    }
}

/// Returns the width in bits of one row of the ZIA
pub fn zia_get_row_width(device: XC2Device) -> usize {
    match device {
        XC2Device::XC2C32 | XC2Device::XC2C32A => 8,
        XC2Device::XC2C64 | XC2Device::XC2C64A => 16,
        _ => unreachable!(),
    }
}
