<script lang="ts">
	import Button from '$lib/Button.svelte';
	import { sleep } from '$lib/functions/helper';
	import { getGame } from '$lib/functions/requests';
	import { onMount } from 'svelte';

	export let setGameState: (new_state: string) => void;
	let player_one: String;
	let player_two: String;

	async function readGame() {
		getGame(localStorage.getItem('game_name'))
			.then((response) => response.json())
			.then((data) => {
				player_one = data.player_one;
				player_two = data.player_two;
			});
	}

	let get_game_interval_ms: number = 1000;
	async function getGameLoop() {
		if (localStorage.getItem('game_state') == 'waiting_room') {
			readGame();
			await sleep(get_game_interval_ms);
			getGameLoop();
		}
	}

	onMount(() => {
		getGameLoop();
	});

	function onStart() {
		if (player_two != null){
			setGameState('answer');
		}
	}
</script>

<main>
	<h1>Waiting Room</h1>
	<div>
		Orange Team: {player_one}
	</div>
	<div>
		Purple Team:
		{#if player_two != null}
			{player_two}
		{/if}
	</div>
	<div>
		<Button text="Start" onClick={onStart} />
	</div>
</main>

<style>
	@import '../../app.css';
</style>
