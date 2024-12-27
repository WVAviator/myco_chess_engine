import chess.pgn
from typing import Generator, List

def read_pgn_in_batches(filepath: str, batch_size: int) -> Generator[List[chess.pgn.Game], None, None]:
    with open(filepath, "r") as pgn_file:
        batch = []
        while True:
            game = chess.pgn.read_game(pgn_file)
            if game is None:
                if batch:
                    yield batch  # Yield any remaining games in the last batch
                break
            batch.append(game)
            if len(batch) == batch_size:
                yield batch
                batch = []  # Start a new batch