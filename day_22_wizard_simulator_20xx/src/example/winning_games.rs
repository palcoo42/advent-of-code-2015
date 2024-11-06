use super::game::Game;

#[derive(Debug, Default, Clone)]
pub struct WinningGames {
    games: Vec<Game>,
}

impl WinningGames {
    pub fn new(game: Game) -> Self {
        Self { games: vec![game] }
    }

    pub fn replace_games(&mut self, game: Game) {
        self.games = vec![game];
    }

    pub fn add_game(&mut self, game: Game) {
        self.games.push(game);
    }

    pub fn get_spent_mana(&self) -> Option<u32> {
        // All winning games has the sme lowest mana
        self.games.first().map(|g| g.get_spent_mana())
    }

    pub fn get_games(&self) -> &Vec<Game> {
        &self.games
    }
}
