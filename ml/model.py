import torch
import torch.nn as nn

class MycoCNNModel(nn.Module):
    def __init__(self):
        super(MycoCNNModel, self).__init__()

        self.conv1 = nn.Conv2d(7, 32, kernel_size=3, padding=1)  # 7 channels -> 32 filters
        self.relu = nn.ReLU()
        self.conv2 = nn.Conv2d(32, 64, kernel_size=3, padding=1)  # 32 filters -> 64 filters
        self.pool = nn.MaxPool2d(kernel_size=2)  # Downsample by 2

        self.fc1 = nn.Linear(64 * 4 * 4, 128)  # From 4x4 spatial downsampling
        self.fc2 = nn.Linear(128, 1)  # Output evaluation in centipawns

    def forward(self, x):
        x = self.relu(self.conv1(x))
        x = self.pool(self.relu(self.conv2(x)))
        x = x.view(x.size(0), -1)  # Flatten for fully connected layers
        x = self.relu(self.fc1(x))
        output = self.fc2(x)
        return output
