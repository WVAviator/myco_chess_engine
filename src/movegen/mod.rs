use bishop::BishopMoveGen;
use king::KingMoveGen;
use knight::KnightMoveGen;
use pawn::PawnMoveGen;
use rook::RookMoveGen;

use crate::cgame::{game::Game, moves::LongAlgebraicMove};

mod bishop;
mod king;
mod knight;
mod pawn;
mod rook;

pub trait MoveGen: PawnMoveGen + KingMoveGen + BishopMoveGen + RookMoveGen + KnightMoveGen {
    fn generate_pseudolegal_moves(&self) -> Result<Vec<LongAlgebraicMove>, anyhow::Error>;
}

impl MoveGen for Game {
    fn generate_pseudolegal_moves(&self) -> Result<Vec<LongAlgebraicMove>, anyhow::Error> {
        let mut moves = Vec::new();

        moves.extend(self.generate_pseudolegal_king_moves()?);
        moves.extend(self.generate_pseudolegal_bishop_moves()?);
        moves.extend(self.generate_pseudolegal_rook_moves()?);
        moves.extend(self.generate_psuedolegal_pawn_moves()?);
        moves.extend(self.generate_psuedolegal_knight_moves()?);

        Ok(moves)
    }
}
