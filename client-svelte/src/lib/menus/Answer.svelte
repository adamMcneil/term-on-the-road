<script lang="ts">
	import Button from '$lib/Button.svelte';
	import InputField from '$lib/InputField.svelte';
	import { getGame, postAnswer, postChangeQuestion, postChatGptQuestion } from '$lib/functions/requests';
	import type { Game } from '$lib/datatypes/game';
	import { onMount } from 'svelte'
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
		readGame();
		await sleep(get_game_interval_ms);
		getGameLoop();
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

	function onChangeQuestion() {
		const response: Promise<Response> = postChangeQuestion(game_name);
	}

	function onMrGptQuestion() {
		if (prompt == '') {
			return;
		}
		const response: Promise<Response> = postChatGptQuestion(game_name, prompt);
	}

	onMount(() => {
		getGameLoop();
	})

</script>

<main>
	<div>
		hello
	</div>
	<h2>
		Round: {round_count}
	</h2>
	<!-- <div>
		<Button text="Change Question" onClick={onChangeQuestion} />
	</div> -->
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

	<!-- <div>
		<InputField bind:value={prompt} text="enter Mr. GPT prompt" />
	</div>
	<div>
		<Button text="Get Mr. GPT question" onClick={onMrGptQuestion} />
	</div> -->
</main>

<style>
	@import '../../app.css';
</style>
