use bindings;
use std::ptr;

// ----------------
// Exposed C API

/// See api.h for the C API being implemented.
#[no_mangle]
pub unsafe extern "C" fn calc_pi(n: u8) -> f32 {
    calc_pi_impl(n)
}

/// See solver.h for the C API being implemented.
#[no_mangle]
pub unsafe extern "C" fn calc_probs(
    c_board: *const bindings::board_t,
    c_probs: *mut f32,
) -> bindings::retcode {
    // Check args are non-null, and just hope the pointers are otherwise valid!
    if c_board.is_null() || c_probs.is_null() {
        eprintln!("Invalid NULL pointer passed in");
        return bindings::RC_INVALID_ARG;
    }

    let board = c_board.read(); // Read the value at the pointer
    if board.cells.is_null() || board.num_cells == 0 {
        eprintln!("Invalid board cells");
        return bindings::RC_INVALID_ARG;
    }
    let mut cells: Vec<bindings::cell_contents_t> = Vec::new();
    for i in 0..board.num_cells as usize {
        cells.push(board.cells.add(i).read());
    }

    let probs: Vec<f32> = calc_probs_impl(cells, board.capacity);
    for (i, p) in probs.iter().enumerate() {
        ptr::write(c_probs.add(i), *p); // Write the value at the pointer
    }

    bindings::RC_SUCCESS
}

// ----------------
// Rust implementation

/// Rust implementation of the 'calc_pi()' API.
fn calc_pi_impl(_digits: u8) -> f32 {
    let pi: f32;
    // Some logic ...
    pi = 5.14 - 2.0;
    // Some logic ...
    pi
}

/// Rust implementation of `calc_probs()`.
fn calc_probs_impl(cells: Vec<bindings::cell_contents_t>, _capacity: u8) -> Vec<f32> {
    let mut vec: Vec<f32> = vec![0.0; cells.len()];
    // Some logic...
    vec[0] = 0.44;
    // Some logic...
    vec
}

// ----------------
// Tests

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn calc_probs_invalid_args() {
        unsafe {
            let rc = calc_probs(ptr::null(), ptr::null_mut());
            assert_eq!(rc, bindings::RC_INVALID_ARG);
        }
    }
}
