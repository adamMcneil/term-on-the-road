import { Round } from "./round";
import { Board } from "./board";

export class Game {
    player_one!: String;
    player_two!: String;
    rounds!: Array<Round>;
    board!: Board;

    // constructor(players: Array<Player>, rounds: Array<Round>) {
    //     this.players = players;
    //     this.rounds = rounds;
    // }

    // constructor(names: Array<string>, rounds: Array<object>) {
    //     this.players = new Array();
    //     names.forEach(name => {
    //        this.players.push(new Player(name)); 
    //     });
        
    //     this.rounds = new Array();
    //     rounds.forEach(round => {
    //         this.rounds.push(new Round(round.question))
    //     })
    // }
}