#ifndef API_H
#define API_H

#include <stdint.h>
#include "errors.h"


/* Basic dummy API to calculate pi. */

// Function to calculate pi correct to n decimal places.
float calc_pi(uint8_t n);


/* Mock complex API to manipulate 'board state'. */

// Enumeration of possible cell contents.
typedef enum {
    CELL_EMPTY,
    CELL_HALF_FULL,
    CELL_FULL,
    CELL_UNKNOWN,
} cell_contents_t;

// A struct representing the state of a board.
typedef struct board {
    cell_contents_t *cells;
    uint8_t          num_cells;
    uint8_t          capacity;
} board_t;

/*
 * Function to calculate probabilities of a given board.
 *
 * Argument: board
 *   IN    - The board state to calculate probabilities for.
 *
 * Argument: probs
 *   INOUT - An array to fill in with the probabilities for each cell.
 *           The memory must be allocated (and is owned) by the caller, with the
 *           array being of a length corresponding to the number of cells.
 *
 * Return:
 *   Return code.
 */
retcode calc_probs(const board_t *board,
                   float         *probs);


#endif //API_H
