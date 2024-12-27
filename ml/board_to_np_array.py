import numpy as np
import chess

def board_to_np_array(board: chess.Board) -> np.ndarray:
    """
    Convert a python-chess board to a 7x8x8 numpy array representation.
    """
    # Initialize an empty 7x8x8 array
    board_array = np.zeros((7, 8, 8), dtype=np.int8)
    
    # Map piece types to layers
    piece_map = {
        chess.PAWN: 0,
        chess.ROOK: 1,
        chess.KNIGHT: 2,
        chess.BISHOP: 3,
        chess.QUEEN: 4,
        chess.KING: 5,
    }
    
    # Populate layers 1-6
    for square, piece in board.piece_map().items():
        row, col = divmod(square, 8)
        layer = piece_map[piece.piece_type]
        board_array[layer, row, col] = 1 if piece.color == chess.WHITE else -1
    
    # Populate layer 7 (en passant)
    if board.ep_square is not None:
        row, col = divmod(board.ep_square, 8)
        board_array[6, row, col] = -1  # Mark the en passant target

    board_array = np.flip(board_array, axis=(1))

    if board.turn == chess.BLACK:
        board_array = np.flip(board_array, axis=(1))
        board_array[:6] *= -1

    return board_array
