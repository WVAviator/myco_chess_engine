use bishop::BishopMoveGen;
use king::KingMoveGen;
use knight::KnightMoveGen;
use pawn::PawnMoveGen;
use rook::RookMoveGen;
use simulate::Simulate;

use crate::cgame::{
    game::{Game, Turn},
    moves::LongAlgebraicMove,
};

mod bishop;
mod king;
mod knight;
mod pawn;
mod rook;
mod simulate;

pub trait MoveGen:
    PawnMoveGen + KingMoveGen + BishopMoveGen + RookMoveGen + KnightMoveGen + Simulate
{
    fn generate_vision(&self, turn: &Turn) -> Result<u64, anyhow::Error>;
    fn generate_pseudolegal_moves(&self) -> Result<Vec<LongAlgebraicMove>, anyhow::Error>;
    fn generate_legal_moves(&self) -> Result<Vec<LongAlgebraicMove>, anyhow::Error>;
}

impl MoveGen for Game {
    fn generate_vision(&self, turn: &Turn) -> Result<u64, anyhow::Error> {
        let mut vision = 0;
        vision |= self.generate_king_vision(turn)?;
        vision |= self.generate_pawn_vision(turn)?;
        vision |= self.generate_rook_vision(turn)?;
        vision |= self.generate_bishop_vision(turn)?;
        vision |= self.generate_knight_vision(turn)?;

        Ok(vision)
    }
    fn generate_pseudolegal_moves(&self) -> Result<Vec<LongAlgebraicMove>, anyhow::Error> {
        let mut moves = Vec::new();

        moves.extend(self.generate_pseudolegal_king_moves()?);
        moves.extend(self.generate_pseudolegal_bishop_moves()?);
        moves.extend(self.generate_pseudolegal_rook_moves()?);
        moves.extend(self.generate_psuedolegal_pawn_moves()?);
        moves.extend(self.generate_psuedolegal_knight_moves()?);

        Ok(moves)
    }
    fn generate_legal_moves(&self) -> Result<Vec<LongAlgebraicMove>, anyhow::Error> {
        self.generate_pseudolegal_moves().map(|legal_moves| {
            legal_moves
                .into_iter()
                .filter(|lmove| self.check_move_legality(lmove).unwrap_or(false))
                .collect()
        })
    }
}
