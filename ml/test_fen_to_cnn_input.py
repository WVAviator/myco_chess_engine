
import unittest
import numpy as np
from fen_processing import fen_to_cnn_input_compact

class TestFenToCnnInputCompact(unittest.TestCase):

    def test_white_turn(self):
        fen = "8/8/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
        result = fen_to_cnn_input_compact(fen)
        self.assertEqual(result.shape, (7, 8, 8))
        self.assertEqual(result[0, 6, 0], 1)
        self.assertEqual(result[5, 7, 4], 1)

    def test_black_turn(self):
        fen = "rnbqkbnr/pppppppp/8/8/8/8/8/8 b KQkq - 0 1"
        result = fen_to_cnn_input_compact(fen)
        self.assertEqual(result.shape, (7, 8, 8))
        self.assertEqual(result[0, 6, 0], 1)
        self.assertEqual(result[5, 7, 4], 1)

    def test_en_passant(self):
        fen = "8/8/8/8/8/8/8/8 w - e3 0 1"
        result = fen_to_cnn_input_compact(fen)
        self.assertEqual(result[6, 2, 4], -1)

    def test_empty_squares(self):
        fen = "8/8/8/8/8/8/8/8 w - - 0 1"
        result = fen_to_cnn_input_compact(fen)
        self.assertTrue(np.all(result == 0))

if __name__ == '__main__':
    unittest.main()
