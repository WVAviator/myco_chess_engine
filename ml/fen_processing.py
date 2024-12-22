import numpy as np

def fen_to_cnn_input_compact(fen):
    """
    Converts a FEN string to a compact 7x8x8 array suitable for a CNN.
    Flips the board if it's Black's turn.
    
    :param fen: A string representing a chess position in FEN format.
    :return: A numpy array of shape (7, 8, 8).
    """
    piece_to_index = {
        'P': 0, 'N': 1, 'B': 2, 'R': 3, 'Q': 4, 'K': 5,
        'p': 0, 'n': 1, 'b': 2, 'r': 3, 'q': 4, 'k': 5
    }
    board_fen, turn, _, en_passant, _, _ = fen.split(' ')
    cnn_input = np.zeros((7, 8, 8), dtype=np.float32)

    for row_idx, rank in enumerate(board_fen.split('/')):
        col_idx = 0
        for char in rank:
            if char.isdigit():
                col_idx += int(char)
            elif char in piece_to_index:
                plane = piece_to_index[char]
                cnn_input[plane, row_idx, col_idx] = 1 if char.isupper() else -1
                col_idx += 1

    if en_passant != '-':
        file = ord(en_passant[0]) - ord('a')
        rank = int(en_passant[1]) - 1
        cnn_input[6, rank, file] = -1

    if turn == 'b':
        cnn_input = np.flip(cnn_input, axis=1)
        cnn_input[:6] *= -1

    return cnn_input

print(fen_to_cnn_input_compact("8/8/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"))