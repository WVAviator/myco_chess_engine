from position_data import PositionData, ChessDataset, generate_training_data
from read_pgn import read_pgn_in_batches

import torch
from torch.utils.data import DataLoader, Dataset

def train_on_pgn(filepath, model, batch_size=500, num_epochs=24, device='cuda' if torch.cuda.is_available() else 'cpu'):
    optimizer = torch.optim.Adam(model.parameters(), lr=1e-4)
    criterion = torch.nn.L1Loss()
    model.to(device)
    
    for game_batch in read_pgn_in_batches(filepath, batch_size):
        print(f"Processing batch of {len(game_batch)} games")

        position_data = []
        for game in game_batch:
            position_data.extend(generate_training_data(game))
        
        dataset = ChessDataset(position_data)
        dataloader = DataLoader(dataset, batch_size=128, shuffle=True)
        
        for epoch in range(num_epochs):
            model.train()
            running_loss = 0.0
            for inputs, targets in dataloader:
                inputs, targets = inputs.to(device), targets.to(device)
                optimizer.zero_grad()
                outputs = model(inputs)
                loss = criterion(outputs.squeeze(), targets)
                loss.backward()
                optimizer.step()
                running_loss += loss.item()
            print(f"Epoch {epoch+1}/{num_epochs}, Loss: {running_loss/len(dataloader)}")
