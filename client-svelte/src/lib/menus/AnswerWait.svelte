<script lang="ts">
	import { onMount } from 'svelte';
	import { getGame, } from '$lib/functions/requests';
	import { sleep } from '$lib/functions/helper';

	export let setGameState: (new_state: string) => void;
	export let game_name: string | null;

	let player_one: String;
	let player_two: String;
	let player_one_question: String;
	let player_two_question: String;
	let round_count: number;
	let round_over: boolean = false;

	async function readGame() {
		getGame(localStorage.getItem('game_name'))
			.then((response) => response.json())
			.then((data) => {
				player_one = data.player_one;
				player_two = data.player_two;
				player_one_question = data.rounds[data.rounds.length - 1].player_one_question;
				player_two_question = data.rounds[data.rounds.length - 1].player_two_question;
				round_count = data.rounds.length;
				if (
					data.rounds[data.rounds.length - 1].player_one_answer == null &&
					data.rounds[data.rounds.length - 1].player_two_answer == null
				) {
					round_over = true;
					setGameState('reveal');
				}
			});
	}

	let get_game_interval_ms: number = 1000;
	async function getGameLoop() {
		if (localStorage.getItem('game_state') == 'answer_wait') {
			readGame();
			await sleep(get_game_interval_ms);
			getGameLoop();
		}
	}

	onMount(() => {
		getGameLoop();
	});
</script>

<main>
	<h2>
		Round: {round_count}
	</h2>
	<h3>Waiting on the other team...</h3>
	<!-- <div>
		<Button
			text={'reveal next question'}
			onClick={() => {
				setGameState('answer');
			}}
		/>
	</div> -->
</main>

<style>
	@import '../../app.css';
</style>
