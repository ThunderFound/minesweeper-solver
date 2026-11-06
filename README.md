### Minesweeper solver

Solver mainly does only 2 steps, RREF and substitution.

For the RREF (Reduced Row Echelon Form) step I used this amazing blog by Robert Massaioli: https://massaioli.wordpress.com/2013/01/12/solving-minesweeper-with-matricies/<br>
First, the solver builds an augmented matrix from the field, where 1 = mine and 0 = safe. After that, it reduces the matrix to RREF and then computes min and max bounds for each row:<br>
min bound is equal to the sum of all coefficients that are less than 0<br>
max bound is equal to the sum of all coefficients that are greater than 0<br>
If the RHS (right-hand side) is equal to the min bound, then all negative coefficients are mines and all positive coefficients are safe.<br>
If the RHS is equal to the max bound, then all negative coefficients are safe and all positive coefficients are mines.

This step is repeated until the solver couldn't find any new mines/safe tiles.<br>
Next, the solved does back-substitution/backtracking on the matrix. For each unrevealed tile that has any significance on the board (has neighboring tiles that are already revealed) it tries to force that tile being a mine (1) or safe (0).<br>
It tries substituting this value into each row, while finding values of other tiles. If it finds a contradiction while substituting, then the tile cannot have this value. Otherwise, try substituting the other value.

These 2 steps are repeated until there are no unknown tiles left or it couldn't find any more tiles.

Solver doesn't check boards for validity yet