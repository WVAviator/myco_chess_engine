from model import MycoCNNModel
from training import train_on_pgn
import os
import torch
import torch.nn as nn
import argparse

MODEL_PATH = "myco_training_model.pt"
SCRIPTED_MODEL_PATH = "../resources/myco_eval_model.pt"

def main():
    parser = argparse.ArgumentParser(description="Train a chess evaluation model on a PGN file.")
    parser.add_argument(
        "pgn_file",
        type=str,
        help="Path to the PGN file containing chess games."
    )
    args = parser.parse_args()

    pgn_file = args.pgn_file

    model = load_model(MycoCNNModel)

    train_on_pgn(pgn_file, model)


def save_model(model: nn.Module):
    model.eval()
    torch.save(model.state_dict(), MODEL_PATH)
    scripted_model = torch.jit.script(model)
    scripted_model.save(SCRIPTED_MODEL_PATH)
    print(f"Model saved to {MODEL_PATH} and scripted model saved to {SCRIPTED_MODEL_PATH}")

def load_model(model_class):
    model = model_class()
    if os.path.exists(MODEL_PATH):
        model.load_state_dict(torch.load(MODEL_PATH))
        print(f"Model loaded from {MODEL_PATH}")
    else:
        print("No saved model found. Starting with a new model.")
    model.train()
    return model


if __name__ == "__main__":
    main()