<script lang="ts">
	import type { Board } from '$lib/datatypes/board';
	import { sleep } from '$lib/functions/helper';
	import { getGame } from '$lib/functions/requests';
	import { onMount } from 'svelte';

	let player_one: String;
	let player_two: String;
	let player_one_question: String;
	let player_two_question: String;
	let round_count: number;
	let board: Board;
	let board_map: Map<String, number> = new Map();
	let team_one_captured_letters: Set<String>;
	let team_two_captured_letters: Set<String>;

	// $: board_map = board.board
	// $: team_one_captured_letters = board.player_one_captured
	// $: team_two_captured_letters = board.player_two_captured

	async function readGame() {
		getGame(localStorage.getItem('game_name'))
			.then((response) => response.json())
			.then((data) => {
				board = data.board;

				for (var property in data.board.board) {
					board_map = board_map.set(property, data.board.board[property]);
				}
				team_one_captured_letters = new Set([...data.board.player_one_captured]);
				team_two_captured_letters = new Set([...data.board.player_two_captured]);
                console.log(data.board)
				// console.log(Object.entries(data.board.board));
				// board_map = new Map(Object.entries(data.board.board));
				// board_map = data.board.board;
				// console.log(board_map);

				player_one = data.player_one;
				player_two = data.player_two;
				player_one_question = data.rounds[data.rounds.length - 1].player_one_question;
				player_two_question = data.rounds[data.rounds.length - 1].player_two_question;
				round_count = data.rounds.length;
			});
	}

	let get_game_interval_ms: number = 1000;
	async function getGameLoop() {
		if (localStorage.getItem('game_state') != 'join') {
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
	<div>
		<table class="center">
			{#if board !== undefined}
				{#each [...board_map] as [key, value]}
					<tr>
						<td class="team_orange">
							{#if team_one_captured_letters.has(key)}{key.toUpperCase()}{/if}</td
						>
						<td>
							{#if value == 2}{key.toUpperCase()}{/if}</td
						>
						<td
							>{#if value == 1}{key.toUpperCase()}{/if}</td
						>
						<td
							>{#if value == 0}{key.toUpperCase()}{/if}</td
						>
						<td
							>{#if value == -1}{key.toUpperCase()}{/if}</td
						>
						<td
							>{#if value == -2}{key.toUpperCase()}{/if}</td
						>
						<td class="team_purple">
							{#if team_two_captured_letters.has(key)}{key.toUpperCase()}{/if}</td
						>
					</tr>
				{/each}
			{/if}
		</table>
	</div>
</main>

<style>
	@import '../../app.css';
	table,
	th,
	td {
		border: 1px solid black;
	}
	.center {
		margin-left: auto;
		margin-right: auto;
	}
	td {
		padding: 10px;
		width: 10px;
		height: 20px;
	}
	.team_purple {
		background-color: purple;
	}
	.team_orange {
		background-color: orange;
	}
</style>
