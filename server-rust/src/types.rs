mod traits;

use reqwest::header::{OccupiedEntry, VacantEntry};
use serde::{Deserialize, Serialize};
use core::str;
#[cfg(test)]
use std::iter::FromIterator;
use std::{
    collections::{HashMap, HashSet},
    error, fmt,
};

use crate::Opt;

pub(crate) type Result<T> = std::result::Result<T, Error>;
pub(crate) type Player = String;
pub(crate) type GameId = String;
pub(crate) type Prompt = String;

#[derive(Serialize, Debug)]
pub(crate) enum Error {
    GameConflict,
    GameNotFound,
    PlayerConflict,
    PlayerNotFound,
    RoundNotInStartState,
    RoundNotInCollectingAnswersState,
    RoundNotInCollectingGuessesState,
    GuessedPlayerNotFound,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GameConflict => write!(f, "game conflict"),
            Self::GameNotFound => write!(f, "game not found"),
            Self::PlayerConflict => write!(f, "player conflict"),
            Self::PlayerNotFound => write!(f, "player not found"),
            Self::RoundNotInStartState => write!(f, "round not in start state"),
            Self::RoundNotInCollectingAnswersState => {
                write!(f, "round not in collecting answer state")
            }
            Self::RoundNotInCollectingGuessesState => {
                write!(f, "round not in collecting guess state")
            }
            Self::GuessedPlayerNotFound => write!(f, "guessed player not found"),
        }
    }
}

impl error::Error for Error {}

#[derive(Deserialize, Serialize)]
pub(crate) struct BadRequest {
    error: String,
    message: String,
}

impl BadRequest {
    fn new(error: Error) -> Self {
        Self {
            error: format!("{:?}", error),
            message: format!("{}", error),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub(crate) struct PromptData {
    pub(crate) prompt: Prompt,
}

#[cfg(test)]
impl PromptData {
    pub(crate) fn new(prompt: &str) -> Self {
        Self {
            prompt: Prompt::from(prompt),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub(crate) struct PlayerData {
    /// The player with which the request is associated
    pub(crate) player: Player,
}

#[cfg(test)]
impl PlayerData {
    pub(crate) fn new(player: &str) -> Self {
        Self {
            player: Player::from(player),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub(crate) struct Answer {
    /// The player who gave the answer
    player: Player,
    /// The answer to the question for the round
    pub answer: String,
}

#[cfg(test)]
impl Answer {
    pub(crate) fn new(player: &str, answer: &str) -> Self {
        Self {
            player: Player::from(player),
            answer: String::from(answer),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct Guess {
    /// The player making the guess
    pub player: Player,
    /// The list of guessed answers, one per player
    pub answers: HashSet<Answer>,
}

#[cfg(test)]
impl Guess {
    pub(crate) fn new(player: &str, guess: Vec<Answer>) -> Self {
        Self {
            player: Player::from(player),
            answers: HashSet::from_iter(guess),
        }
    }
}

#[derive(PartialEq)]
pub(crate) enum RoundState {
    Start,
    CollectingAnswers,
    CollectingGuesses,
    Complete,
}

#[derive(Clone, Deserialize, Serialize)]
pub(crate) struct Round {
    /// The question for the round
    player_one_question: String,
    player_two_question: String,
    /// The list of answers given, one per player
    player_one_answer: Option<String>,
    player_two_answer: Option<String>,
}

impl Round {
    fn new(player_one_question: String, player_two_question: String) -> Self {
        Round {
            player_one_question,
            player_two_question,
            player_one_answer: None,
            player_two_answer: None,
        }
    }

    fn state(&self) -> RoundState {
        if self.player_one_answer.is_none() && self.player_two_answer.is_none() {
            RoundState::Start
        } else if self.player_one_answer.is_some() && self.player_two_answer.is_some() {
            RoundState::Complete
        } else {
            RoundState::CollectingAnswers
        }
    }

    // fn change_question(&mut self, new_question: String) -> () {
    //     self.question = new_question;
    // }
}

#[derive(Clone, Default, Deserialize, Serialize)]
pub(crate) struct Board {
    pub(crate) board: HashMap<char, i32>,
    pub(crate) player_one_captured: HashSet<char>,
    pub(crate) player_two_captured: HashSet<char>,
}

impl Board {
    pub(crate) fn move_board(&mut self, player_one_answer: String, player_two_answer: String) -> () {
        for letter in player_one_answer.chars() {
            match self.board.entry(letter) {
                std::collections::hash_map::Entry::Occupied(mut entry) => {
                    if self.board.contains_key(&letter) {
                        entry.insert(entry.get() + 1);
                    }
                },
                std::collections::hash_map::Entry::Vacant(_) => (),
            }
        } 

        for letter in player_two_answer.chars() {
            match self.board.entry(letter) {
                std::collections::hash_map::Entry::Occupied(mut entry) => {
                    if self.board.contains_key(&letter) {
                        entry.insert(entry.get() - 1);
                    }
                },
                std::collections::hash_map::Entry::Vacant(_) => (),
            }
        }

        for (key, value) in self.board {
            if value > 3 {
                self.player_one_captured.insert(key);
                self.board.remove(&key);
            } else if value < -3 {
                self.player_two_captured.insert(key);
                self.board.remove(&key);
            }
        }

    }
}

#[derive(Clone, Default, Deserialize, Serialize)]
pub(crate) struct Game {
    /// The list of players in the game
    pub(crate) player_one: Option<String>,
    pub(crate) player_two: Option<String>,
    /// The list of rounds in the game with the most recent round being the last item in the list
    pub(crate) rounds: Vec<Round>,
    /// 
    pub(crate) board: Board,
}

impl Game {
    pub(crate) fn add_player(&mut self, player: Player) -> Result<()> {
        // Only allow adding players at the start of a round
        if self.current_round_state() != RoundState::Start {
            return Err(Error::RoundNotInStartState);
        }
        if self.player_one.is_none() {
            self.player_one = Some(player);
            Ok(())
        } else if self.player_two.is_none() {
            self.player_two = Some(player);
            Ok(())
        } else {
            Err(Error::PlayerConflict)
        }
    }

    pub(crate) fn answer(&mut self, answer: Answer) -> Result<()> {
        let player = &answer.player;
        // Confirm the player exists
        if self.player_one.unwrap().eq(player) || self.player_two.unwrap().eq(player) {
            return Err(Error::PlayerNotFound);
        }
        // Confirm we are collecting answers for the current round
        let state = self.current_round_state();
        if state != RoundState::Start && self.current_round_state() != RoundState::CollectingAnswers
        {
            return Err(Error::RoundNotInCollectingAnswersState);
        }
        // Add or replace the answer
        let round = self.current_round_mut();
        if player.eq(&self.player_one.unwrap())  {
           round.player_one_answer = Some(answer.answer); 
        } else {
            round.player_two_answer = Some(answer.answer);
        }
        Ok(())
    }

    pub(crate) fn add_round_if_complete(&mut self, question_one: String, question_two: String) {
        if self.current_round_state() == RoundState::Complete {
            self.board.move_board(self.current_round().player_one_answer.unwrap(), self.current_round().player_two_answer.unwrap());
            self.add_round(question_one, question_two);
        }
    }

    fn add_round(&mut self, question_one: String, question_two: String) {
        self.rounds.push(Round::new(question_one, question_two));
    }

    pub(crate) fn current_round(&self) -> &Round {
        let index = self.rounds.len() - 1;
        &self.rounds[index]
    }

    fn current_round_mut(&mut self) -> &mut Round {
        let index = self.rounds.len() - 1;
        &mut self.rounds[index]
    }

    fn current_round_state(&self) -> RoundState {
        let round = self.current_round();
        round.state()
    }

    // pub fn change_question(&mut self, new_question: String) -> () {
    //     self.rounds
    //         .last_mut()
    //         .unwrap()
    //         .change_question(new_question);
    // }
}

#[derive(Default)]
pub(crate) struct Games(HashMap<String, Game>);

impl Games {
    #[allow(clippy::map_entry)]
    pub(crate) fn create(
        &mut self,
        game_id: String,
        initial_player: Player,
        initial_question: String,
    ) -> Result<()> {
        if self.0.contains_key(&game_id) {
            Err(Error::GameConflict)
        } else {
            let mut game = Game::default();
            game.add_round(initial_question, initial_question);
            game.add_player(initial_player)?;
            self.0.insert(game_id, game);
            Ok(())
        }
    }

    pub(crate) fn get(&mut self, game_id: &str) -> Result<&mut Game> {
        self.0.get_mut(game_id).ok_or(Error::GameNotFound)
    }

    pub(crate) fn delete(&mut self, game_id: &str) {
        self.0.remove(game_id);
    }
}
