use crate::octet::Octet;
use crate::symbol::Symbol;
use crate::util::get_both_indices;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum SymbolOps {
    AddAssign {
        dest: usize,
        src: usize,
    },
    MulAssign {
        dest: usize,
        scalar: Octet,
    },
    Fma {
        dest: usize,
        src: usize,
        scalar: Octet,
    },
    Reorder {
        order: Vec<usize>,
    },
}

pub fn perform_op(op: &SymbolOps, symbols: &mut Vec<Symbol>) {
    match op {
        SymbolOps::AddAssign { dest, src } => {
            let (dest, temp) = get_both_indices(symbols, *dest, *src);
            *dest += temp;
        }
        SymbolOps::MulAssign { dest, scalar } => {
            symbols[*dest].mulassign_scalar(scalar);
        }
        SymbolOps::Fma { dest, src, scalar } => {
            let (dest, temp) = get_both_indices(symbols, *dest, *src);
            dest.fused_addassign_mul_scalar(temp, scalar);
        }
        SymbolOps::Reorder { order } => {
            /* TODO: Reorder is the last step of the algorithm. It should be
             *       possible to move reorder to be the first step and use when
             *       creating D (place all rows in correct position before
             *       calculations). This will however force an update on all
             *       row-numbers used in all other "Operations". */
            let mut temp_symbols: Vec<Option<Symbol>> = symbols.drain(..).map(Some).collect();
            for row_index in order.iter() {
                symbols.push(temp_symbols[*row_index].take().unwrap());
            }
        }
    }
}

