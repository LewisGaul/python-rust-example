import pathlib
import subprocess
import cffi


RUST_PROJ_PATH = pathlib.Path(".")
HEADER_PATH = RUST_PROJ_PATH / "include" / "api.h"
LIB_PATH = RUST_PROJ_PATH / "target" / "debug" / "libexample.so"


def _read_header(hdr):
    """Run the C preprocessor over a header file."""
    return subprocess.run(
        ["cc", "-E", hdr], stdout=subprocess.PIPE, universal_newlines=True
    ).stdout


ffi = cffi.FFI()                          # Initialise
ffi.cdef(_read_header(str(HEADER_PATH)))  # Read in the header file
lib = ffi.dlopen(str(LIB_PATH))           # Open the dynamic library
pi = lib.calc_pi(2)                       # Call the Rust function
print("Calculated pi to 2 decimal places:", pi)

# More complicated C API call
num_cells = 10
board_p = ffi.new("board_t *")                     # Single board struct
cells_p = ffi.new("cell_contents_t[]", num_cells)  # Array of cells
board_p.cells = cells_p                            # Fill in the struct
board_p.num_cells = num_cells
probs_p = ffi.new("float[]", num_cells)            # Allocate OUT array

rc = lib.calc_probs(board_p, probs_p)              # Call the Rust function
if rc != lib.RC_SUCCESS:
    raise RuntimeError(f"Return code: {rc}")

probs = [probs_p[i] for i in range(num_cells)]
print("Probs:", probs)

rc = lib.calc_probs(ffi.NULL, ffi.NULL)            # Invalid args
print("Invalid arg return code:", rc)
