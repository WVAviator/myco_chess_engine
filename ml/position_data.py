from typing import NamedTuple, List
import math
import numpy as np
import torch
from torch.utils.data import DataLoader, Dataset
import chess.pgn

from board_to_np_array import board_to_np_array


class PositionData(NamedTuple):
    tensor: np.ndarray
    evaluation: float

class ChessDataset(Dataset):
    def __init__(self, position_data):
        self.data = position_data
    
    def __len__(self):
        return len(self.data)
    
    def __getitem__(self, idx):
        tensor = self.data[idx].tensor.copy()
        evaluation = self.data[idx].evaluation
        return torch.tensor(tensor, dtype=torch.float32), torch.tensor(evaluation, dtype=torch.float32)



def compute_evaluation(move_number: int, total_moves: int, winner: str) -> float:
    normalized_move = move_number / total_moves
    # Cubic Bézier-inspired easing function: 3x^2 - 2x^3
    evaluation = 3 * (normalized_move ** 2) - 2 * (normalized_move ** 3)
    return evaluation if winner == "white" else -evaluation


def generate_training_data(game: chess.pgn.Game) -> List[PositionData]:
    positions = []
    board = game.board()
    total_moves = len(list(game.mainline_moves()))
    winner = game.headers.get("Result")
    winner = "white" if winner == "1-0" else "black" if winner == "0-1" else None
    if not winner:
        return []  # Skip draws or invalid results
    
    for i, move in enumerate(game.mainline_moves(), start=1):
        board.push(move)
        tensor = board_to_np_array(board)
        evaluation = compute_evaluation(i, total_moves, winner)
        if board.turn == chess.BLACK:  # If it's White's turn, Black made the last move
          evaluation = -evaluation
        positions.append(PositionData(tensor=tensor, evaluation=evaluation))
    
    return positions