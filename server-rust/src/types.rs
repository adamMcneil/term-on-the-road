mod traits;

use parking_lot::Mutex;
use rocket::State;
use serde::{Deserialize, Serialize};
use core::str;
#[cfg(test)]
use std::iter::FromIterator;
use std::{
    collections::{HashMap, HashSet, BTreeMap, BTreeSet},
    error, fmt, hash::Hash,
};

use crate::question_lookup::QuestionLookup;


pub(crate) type Result<T> = std::result::Result<T, Error>;
pub(crate) type Player = String;
pub(crate) type Prompt = String;

#[derive(Serialize, Debug)]
pub(crate) enum Error {
    GameConflict,
    GameNotFound,
    PlayerConflict,
    PlayerNotFound,
    RoundNotInStartState,
    RoundNotInCollectingAnswersState,
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
    Complete,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub(crate) struct Round {
    /// The question for the round
    player_one_question: String,
    player_two_question: String,
    /// The list of answers given, one per player
    pub player_one_answer: Option<String>,
    pub player_two_answer: Option<String>,
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

#[derive(Clone, Default, Deserialize, Serialize, Debug)]
pub(crate) struct Board {
    pub(crate) board: BTreeMap<char, i32>,
    pub(crate) player_one_captured: BTreeSet<char>,
    pub(crate) player_two_captured: BTreeSet<char>,
}

fn make_start_map() -> BTreeMap<char, i32> {
    let mut letter_to_number: BTreeMap<char, i32> = BTreeMap::new();
    let alphabet = "bcdfghjklmnpqrstvwxyz";
    for letter in alphabet.chars() {
        letter_to_number.insert(letter, 0);
    }
    letter_to_number
}

impl Board {
    pub(crate) fn new() -> Self {
        Self {
            board: make_start_map(),
            player_one_captured: BTreeSet::new(),
            player_two_captured: BTreeSet::new(),
        }
    }

    pub(crate) fn move_board(&mut self, player_one_answer: String, player_two_answer: String) -> () {
        for letter in player_one_answer.chars() {
            if self.board.contains_key(&letter) && !self.player_one_captured.contains(&letter) && !self.player_two_captured.contains(&letter) {
                match self.board.entry(letter) {
                    std::collections::btree_map::Entry::Occupied(mut entry) => {
                            entry.insert(entry.get() + 1);
                    },
                    std::collections::btree_map::Entry::Vacant(_) => (),
                }
            }
        } 

        for letter in player_two_answer.chars() {
            if self.board.contains_key(&letter) && !self.player_one_captured.contains(&letter) && !self.player_two_captured.contains(&letter) {
                match self.board.entry(letter) {
                    std::collections::btree_map::Entry::Occupied(mut entry) => {
                            entry.insert(entry.get() - 1);
                    },
                    std::collections::btree_map::Entry::Vacant(_) => (),
                }
            }
        }

        for (key, value) in &self.board {
            if !self.player_one_captured.contains(key) && !self.player_two_captured.contains(key) {
                if value > &2 {
                    self.player_one_captured.insert(*key);
                } else if value < &-2 {
                    self.player_two_captured.insert(*key);
                }
            }
        }

    }
}

#[derive(Clone, Default, Deserialize, Serialize, Debug)]
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

    pub(crate) fn answer(&mut self, answer: Answer, questions: State<'_, Mutex<QuestionLookup>>) -> Result<()> {
        let player = &answer.player;
        // Confirm the player exists
        let (player_one, player_two) = match (self.player_one.clone(), self.player_two.clone()) {
            (Some(player_one), Some(player_two)) => (player_one, player_two),
            _ => {
                return Err(Error::PlayerNotFound);
            }
        };
        if &player_one != player && &player_two != player {
            return Err(Error::PlayerNotFound);
        }
        // Confirm we are collecting answers for the current round
        let state = self.current_round_state();
        if state != RoundState::Start && self.current_round_state() != RoundState::CollectingAnswers
        {
            return Err(Error::RoundNotInCollectingAnswersState);
        }

        let current_round = self.current_round_mut();
        // Add or replace the answer
        if player == &player_one  {
           current_round.player_one_answer = Some(answer.answer); 
        } else {
            current_round.player_two_answer = Some(answer.answer);
        }

        if current_round.player_one_answer.is_some() && current_round.player_two_answer.is_some() {
            let x = questions.lock().get();
            let y = questions.lock().get();
            self.add_round_if_complete(x, y);
        }
        
        Ok(())
    }

    pub(crate) fn add_round_if_complete(&mut self, question_one: String, question_two: String) {
        if self.current_round_state() == RoundState::Complete {
            self.board.move_board(self.current_round().player_one_answer.as_ref().unwrap().to_string(),self.current_round().player_two_answer.as_ref().unwrap().to_string());
            self.add_round(question_one, question_two);
        }
    }

    fn add_round(&mut self, question_one: String, question_two: String) {
        self.rounds.push(Round::new(question_one, question_two));
    }

    // pub(crate) fn previous_round(&self) -> Option<&Round> {
    //     let index = self.rounds.len() - 2;
    //     self.rounds.get(index)
    // }

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

    fn init_board(&mut self) -> () {
        self.board = Board::new();
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
        initial_question_two: String,
    ) -> Result<()> {
        if self.0.contains_key(&game_id) {
            Err(Error::GameConflict)
        } else {
            let mut game = Game::default();
            game.add_round(initial_question, initial_question_two);
            game.add_player(initial_player)?;
            game.init_board();
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
