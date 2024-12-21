class MycoEvaluationModel(nn.Module):
  def __init__(self, max_eval):
    super(MycoEvaluationModel, self).__init__()
    self.max_eval = max_eval
    
    self.fc1 = nn.Linear(837, 128)
    self.relu = nn.ReLU()
    self.fc2 = nn.Linear(128, 64)
    self.fc3 = nn.Linear(64, 1)


  def one_hot_encode_fen(self, fen):
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
          file = ord(en_passant[0]) - ord('a')
          rank = int(en_passant[1]) - 1
          en_passant_index = rank * 8 + file
          en_passant_array[en_passant_index] = 1    
          
      board_flat = output_array.flatten()
      side_to_move_feature = np.array([1 if side_to_move == 'w' else 0], dtype=int)
      castling_array = np.array([
          int('K' in castling),
          int('Q' in castling),
          int('k' in castling),
          int('q' in castling)
      ], dtype=int)
      
      output_array = np.concatenate([board_flat, side_to_move_feature, castling_array, en_passant_array])

      print(output_array)
      print(len(output_array))
      
      return output_array
  
      def forward(self, fen):
        """
        Processes a FEN string and predicts the evaluation in centipawns.
        """
        input_array = self.fen_to_sparse_array(fen)
        
        input_tensor = torch.tensor(input_array, dtype=torch.float32).unsqueeze(0)
        input_tensor[:, :-1] = input_tensor[:, :-1] / self.max_eval

        x = self.relu(self.fc1(input_tensor))
        x = self.relu(self.fc2(x))
        output = self.fc3(x)

        return output * self.max_eval

class MycoCNNModel(nn.Module):
    def __init__(self):
        super(ChessEvaluationCNNCompact, self).__init__()

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
