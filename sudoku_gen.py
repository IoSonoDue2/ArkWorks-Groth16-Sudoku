

#this pythin script is used to generate sudoku puzzles and solutions
import numpy as np

def generate_sudoku(size=9):
    """
    Generate a completed Sudoku puzzle of a given size.
    The default size is 9x9.
    """
    # Initialize an empty size x size array
    sudoku = np.zeros((size, size), dtype=int)

    # Helper function to check if a number can be placed at a certain position
    def can_place(row, col, num):
        # Check if the number is already in the row or column
        if num in sudoku[row] or num in sudoku[:, col]:
            return False

        # Determine the start of the subgrid
        start_row, start_col = row - row % int(pow(size,1/2)), col - col % int(pow(size,1/2))
        
       
        # Check if the number is in the subgrid
        for r in range(int(pow(size,1/2))):
            for c in range(int(pow(size,1/2))):
                if sudoku[start_row + r][start_col + c] == num:
                    return False

        return True

    # Recursive function to solve the sudoku
    def solve():
        # Find the first empty cell
        for row in range(size):
            for col in range(size):
                if sudoku[row][col] == 0:
                    # Try all possible numbers
                    #create an array of disordeed integer form 1 to size-1
                    nume = [i for i in range(1, size + 1)]
                    

                    # Shuffle the array
                    #np.random.shuffle(nume)

                    #print(row, col , "  :  ", nume)
                    for num in nume:
                        if can_place(row, col, num):
                            sudoku[row][col] = num
                            if solve():
                                return True
                            sudoku[row][col] = 0
                    return False
                            
                    
        return True

    # Start solving from the first cell
    solve()
    return sudoku

# Generate a complete sudoku puzzle
sudoku_puzzle = generate_sudoku(16)
print(sudoku_puzzle)
