import numpy as np
import pandas as pd
from sklearn.model_selection import train_test_split
import torch
import torch.nn as nn
import torch.optim as optim
from torch.utils.data import DataLoader, Dataset
from fen_processing import fen_to_cnn_input_compact
from model import MycoCNNModel


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
            ml_input = fen_to_cnn_input_compact(fen)
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

X, y = process_csv_to_training_data("./chessData.csv")

X_train, X_val, y_train, y_val = train_test_split(X, y, test_size=0.2, random_state=42)

train_dataset = FenDataset(X_train, y_train)
val_dataset = FenDataset(X_val, y_val)

train_loader = DataLoader(train_dataset, batch_size=32, shuffle=True)
val_loader = DataLoader(val_dataset, batch_size=32, shuffle=False)

model = MycoCNNModel()
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