export class Board {
    board: Map<String, number>;
    player_one_captured: Set<String>;
    player_two_captured: Set<String>;

    constructor(board: Map<String, number>, player_one_captured: Set<String>, player_two_captured: Set<String>) {
        this.board = board;
        this.player_one_captured = player_one_captured;
        this.player_two_captured = player_two_captured;
    }
}