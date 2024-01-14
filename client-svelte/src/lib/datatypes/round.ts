export class Round {
    player_one_question: String;
    player_two_question: String;
    player_one_answer: String;
    player_two_answer: String;
    
    constructor(q1: String, q2: String, a1: String, a2: String) {
        this.player_one_question = q1;
        this.player_two_question= q2;
        this.player_one_answer = a1;
        this.player_two_answer = a2;
    }
}