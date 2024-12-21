import numpy as np
import pandas as pd
from sklearn.model_selection import train_test_split
import torch
import torch.nn as nn
import torch.optim as optim
from torch.utils.data import DataLoader, Dataset


def fen_to_sparse_array(fen):
    """
    Converts a FEN string to a 12x64 sparse array.
    
    :param fen: FEN string representing a chess position
    :return: numpy array of shape (12, 64)
    """
    
    piece_to_index = {
        'P': 0, 'N': 1, 'B': 2, 'R': 3, 'Q': 4, 'K': 5,  # White pieces
        'p': 6, 'n': 7, 'b': 8, 'r': 9, 'q': 10, 'k': 11  # Black pieces
    }
    
    board_fen, side_to_move, castling, en_passant, _, _ = fen.split(" ")
    output_array = np.zeros((13, 64), dtype=int)
    
    squares = board_fen.split('/')
    square_index = 0
    
    for rank in squares:
        for char in rank:
            if char.isdigit():  # Empty squares
                square_index += int(char)
            elif char in piece_to_index:  # Pieces
                piece_plane = piece_to_index[char]
                output_array[piece_plane, square_index] = 1
                square_index += 1
            else:
                raise ValueError(f"Invalid character in FEN: {char}")
    
    en_passant_array = np.zeros(64, dtype=int)
    if en_passant != '-':
        file = ord(en_passant[0]) - ord('a')  # Convert file to 0-7
        rank = int(en_passant[1]) - 1  # Convert rank to 0-7
        en_passant_index = rank * 8 + file
        en_passant_array[en_passant_index] = 1    
        
        # Flatten the board representation
    board_flat = output_array.flatten()  # Shape (832,)
    
    # Process side to move (1 scalar)
    side_to_move_feature = np.array([1 if side_to_move == 'w' else 0], dtype=int)
    
    # Process castling rights (4 scalars)
    castling_array = np.array([
        int('K' in castling),  # White kingside
        int('Q' in castling),  # White queenside
        int('k' in castling),  # Black kingside
        int('q' in castling)   # Black queenside
    ], dtype=int)
    
    # Concatenate all features into a single vector
    output_array = np.concatenate([board_flat, side_to_move_feature, castling_array, en_passant_array])
    
    return output_array


def process_csv_to_training_data(csv_path):
    """
    Reads a CSV file with FEN and evaluation columns and processes it into training data.
    
    :param csv_path: Path to the CSV file
    :return: A tuple of (X, y) where:
             - X is a numpy array of input features for each FEN
             - y is a numpy array of evaluations
    """
    df = pd.read_csv(csv_path, nrows=100000)
    
    inputs = []
    evaluations = []

    df['Evaluation'] = df['Evaluation'].apply(lambda x: float(x.lstrip('\ufeff').lstrip('#').lstrip('+')) / 24000)
    print(df['Evaluation'].abs().max())
    
    print("Converting FEN strings to sparse arrays for processing.")
    for index, row in df.iterrows():
        fen = row['FEN']
        evaluation = row['Evaluation']
        
        try:
            ml_input = fen_to_sparse_array(fen)
            inputs.append(ml_input)
            evaluations.append(evaluation)
        except ValueError as e:
            print(f"Error processing FEN at index {index}: {fen} - {e}")
    
    print("FEN strings parsed.")
    
    X = np.array(inputs, dtype=float)
    y = np.array(evaluations, dtype=float)
    
    return X, y


class FenDataset(Dataset):
    def __init__(self, features, labels):
        self.features = torch.tensor(features, dtype=torch.float32)
        self.labels = torch.tensor(labels, dtype=torch.float32)

    def __len__(self):
        return len(self.labels)

    def __getitem__(self, idx):
        return self.features[idx], self.labels[idx]

class ChessEvaluationModel(nn.Module):
    def __init__(self, input_size):
        super(ChessEvaluationModel, self).__init__()
        self.fc1 = nn.Linear(input_size, 128)
        self.relu = nn.ReLU()
        self.fc2 = nn.Linear(128, 64)
        self.fc3 = nn.Linear(64, 1)

    def forward(self, x):
        x = self.relu(self.fc1(x))
        x = self.relu(self.fc2(x))
        x = self.fc3(x)
        return x


X, y = process_csv_to_training_data("./chessData.csv")

X_train, X_val, y_train, y_val = train_test_split(X, y, test_size=0.2, random_state=42)

train_dataset = FenDataset(X_train, y_train)
val_dataset = FenDataset(X_val, y_val)

train_loader = DataLoader(train_dataset, batch_size=32, shuffle=True)
val_loader = DataLoader(val_dataset, batch_size=32, shuffle=False)

input_size = X.shape[1]
model = ChessEvaluationModel(input_size)
criterion = nn.MSELoss()
optimizer = optim.Adam(model.parameters(), lr=0.001)

num_epochs = 24
for epoch in range(num_epochs):
    model.train()
    epoch_loss = 0.0
    for batch_features, batch_labels in train_loader:
        optimizer.zero_grad()
        predictions = model(batch_features)
        loss = criterion(predictions.squeeze(), batch_labels)
        loss.backward()
        optimizer.step()
        epoch_loss += loss.item()

    print(f"Epoch {epoch+1}/{num_epochs}, Loss: {epoch_loss:.4f}")

model.eval()
val_loss = 0.0
with torch.no_grad():
    for val_features, val_labels in val_loader:
        val_predictions = model(val_features)
        loss = criterion(val_predictions.squeeze(), val_labels)
        val_loss += loss.item()

val_loss /= len(val_loader)
print(f"Validation Loss: {val_loss:.4f}")

torch.save(model.state_dict(), "chess_eval_model.pt")
print("Model saved as chess_eval_model.pt")