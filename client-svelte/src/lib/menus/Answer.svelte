<script lang="ts">
	import Button from '$lib/Button.svelte';
	import InputField from '$lib/InputField.svelte';
	import { getGame, postAnswer } from '$lib/functions/requests';
	import { onMount } from 'svelte';
	import { sleep } from '$lib/functions/helper';

	export let setGameState: (new_state: string) => void;
	export let name: string | null;
	export let game_name: string | null;

	let player_one: String;
	let player_two: String;
	let player_one_question: String;
	let player_two_question: String;
	let round_count: number;

	let answer: string = '';
	let prompt: string = '';

	async function readGame() {
		getGame(localStorage.getItem('game_name'))
			.then((response) => response.json())
			.then((data) => {
				player_one = data.player_one;
				player_two = data.player_two;
				player_one_question = data.rounds[data.rounds.length - 1].player_one_question;
				player_two_question = data.rounds[data.rounds.length - 1].player_two_question;
				round_count = data.rounds.length;
			});
	}

	let get_game_interval_ms: number = 1000;
	async function getGameLoop() {
		if (localStorage.getItem('game_state') == 'answer') {
			readGame();
			await sleep(get_game_interval_ms);
			getGameLoop();
		}
	}

	function onSubmitClick() {
		if (answer == '') {
			console.log('you need a non-empty answer');
			return;
		}
		const response: Promise<Response> = postAnswer(game_name, name, answer);
		response.then((response) => {
			if (response.ok) {
				setGameState('answer_wait');
			}
		});
	}

	let time: number = 30;
	async function startTimer() {
		time -= 1;
		if (time < 1) {
			onTimerEnd();
		} else if (localStorage.getItem('game_state') == 'answer') {
			await sleep(1000);
			startTimer();
		}
	}

	function onTimerEnd() {
		if (localStorage.getItem('game_state') == 'answer') {
			const response: Promise<Response> = postAnswer(game_name, name, answer);
			response.then((response) => {
				if (response.ok) {
					setGameState('answer_wait');
				}
			});
		}
	}

	onMount(() => {
		getGameLoop();
		startTimer();
	});
</script>

<main>
	<h2>
		Round: {round_count}
	</h2>
	<div>{time}</div>
	{#if name == player_one}
		<div>
			{player_one_question}
		</div>
	{:else}
		<div>
			{player_two_question}
		</div>
	{/if}
	<div>
		<InputField bind:value={answer} text="enter your answer" />
	</div>
	<div>
		<Button text="Submit" onClick={onSubmitClick} />
	</div>
</main>

<style>
	@import '../../app.css';
</style>
