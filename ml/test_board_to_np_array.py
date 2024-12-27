import unittest
import chess
import numpy as np
from board_to_np_array import board_to_np_array

class TestBoardToNpArray(unittest.TestCase):
    def test_initial_position(self):
        board = chess.Board()
        array = board_to_np_array(board)
        self.assertTrue(np.all(array[0, 6, :] == 1))
        self.assertTrue(np.all(array[0, 1, :] == -1))

        self.assertEqual(array[5, 7, 4], 1)
        self.assertEqual(array[5, 0, 4], -1)
        self.assertTrue(np.all(array[6, :, :] == 0))
    
    def test_single_move(self):
        board = chess.Board()
        board.push_san("e4")
        array = board_to_np_array(board)
        self.assertEqual(array[0, 1, 4], 0)
        self.assertEqual(array[0, 3, 4], -1)
    
    def test_en_passant_black(self):
        board = chess.Board()
        board.push_san("e4")
        board.push_san("d5")
        array = board_to_np_array(board)
        self.assertEqual(array[6, 2, 3], -1)

    def test_en_passant_white(self):
        board = chess.Board()
        board.push_san("e4")
        array = board_to_np_array(board)
        self.assertEqual(array[6, 2, 4], -1)

if __name__ == "__main__":
    unittest.main()
