<script lang="ts">
	import { onMount } from 'svelte';
	import Join from '$lib/menus/Join.svelte';
	import Answer from '$lib/menus/Answer.svelte';
	import AnswerWait from '$lib/menus/AnswerWait.svelte';
	import Reveal from '$lib/menus/Reveal.svelte';
	import Board from '$lib/menus/Board.svelte';
	import WaitingRoom from '$lib/menus/WaitingRoom.svelte';

	import { deleteGame, getGame } from '$lib/functions/requests';
	import Button from '$lib/Button.svelte';
	import type { Game } from '$lib/datatypes/game';

	let game_state: string | null;

	let production_url: string = 'https://term-on-the-road.onrender.com/api/v1/game/';
	let test_url: string = 'http://0.0.0.0:8172/api/v1/game/';

	function setGameState(new_state: string) {
		localStorage.setItem('game_state', new_state);
		game_state = new_state;
	}

	onMount(() => {
		if (!localStorage.getItem('game_state')) {
			setGameState('join');
		} else {
			game_state = localStorage.getItem('game_state');
		}
		if (window.location.href == 'http://localhost:5173/') {
			localStorage.setItem('base_server_path', test_url);
		} else {
			localStorage.setItem('base_server_path', production_url);
		}
		if (localStorage.getItem('game_name')) {
			const response: Promise<Response> = getGame(localStorage.getItem('game_name'));
			response.then((response) => {
				if (!response.ok) {
					setGameState('join');
				}
			});
		}
	});

	function onEndGame() {
		if (confirm('Do you want to end the game for everybody?')) {
			const response: Promise<Response> = deleteGame(localStorage.getItem('game_name'));
			setGameState('join');
		}
	}
</script>

<main>
	{#if game_state == 'join'}
		<Join {setGameState} />
	{:else if game_state == 'waiting_room'}
		<WaitingRoom {setGameState} />
	{:else if game_state == 'answer'}
		<Answer
			{setGameState}
			name={localStorage.getItem('name')}
			game_name={localStorage.getItem('game_name')}
		/>
	{:else if game_state == 'answer_wait'}
		<AnswerWait {setGameState} game_name={localStorage.getItem('game_name')} />
	{:else if game_state == 'reveal'}
		<Reveal {setGameState} game_name={localStorage.getItem('game_name')} />
	{/if}
	{#if game_state != 'join'}
		<Board />	
	{/if} 

	<div style="padding: 50px;"></div>
	{#if game_state != 'join'}
		<div>
			<Button text="End Game" onClick={onEndGame} />
		</div>
	{/if}
</main>

<style>
	@import '../app.css';
</style>
