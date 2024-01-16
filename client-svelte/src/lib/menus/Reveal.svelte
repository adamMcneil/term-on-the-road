<script lang="ts">
	import Button from '$lib/Button.svelte';
	import { onMount } from 'svelte';
	import { getGame } from '$lib/functions/requests';
	import { sleep } from '$lib/functions/helper';

	export let setGameState: (new_state: string) => void;
	export let game_name: string | null;

	let player_one: String;
	let player_two: String;
	let player_one_question: String;
	let player_two_question: String;
	let player_one_answer: String;
	let player_two_answer: String;
	let round_count: number;

	async function readGame() {
		getGame(localStorage.getItem('game_name'))
			.then((response) => response.json())
			.then((data) => {
				player_one = data.player_one;
				player_two = data.player_two;
				player_one_question = data.rounds[data.rounds.length - 2].player_one_question;
				player_two_question = data.rounds[data.rounds.length - 2].player_two_question;
				player_one_answer = data.rounds[data.rounds.length - 2].player_one_answer;
				player_two_question = data.rounds[data.rounds.length - 2].player_two_question;
				round_count = data.rounds.length;
			});
	}

	let get_game_interval_ms: number = 1000;
	async function getGameLoop() {
		if (localStorage.getItem('game_state') == 'reveal') {
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
		Round {round_count - 1} Summary
	</h2>
	<div class = "flex">
	<div>
		<div class="orange">
			Team {player_one}
		</div>
		<div>
			{player_one_question}: {player_one_answer}
		</div>
	</div>
	<div>
		<div class="purple">
			Team {player_two}
		</div>
		<div>
			{player_two_question}: {player_two_answer}
		</div>
	</div>
	</div>
	<div>
		<Button
			text={'Next Round'}
			onClick={() => {
				setGameState('answer');
			}}
		/>
	</div>
</main>

<style>
	@import '../../app.css';
	.flex {
		/* display: flex; */
		text-align: center;
	}
	.purple {
		color: purple;
	}
	.orange {
		color: orange;
	}
</style>
