<script lang="ts">
	import Button from '$lib/Button.svelte';
	import InputField from '$lib/InputField.svelte';
	import { Player } from '$lib/datatypes/player';
	import { Game } from '$lib/datatypes/game';
	import { onMount } from 'svelte';
	import { getGame, postAnswer } from '$lib/functions/requests';
	import { sleep } from '$lib/functions/helper';

	export let setGameState: (new_state: string) => void;
	export let game_name: string | null;
	export let game: Game;

	$: players = game.players;
	$: current_question = game.rounds[game.rounds.length - 1].question;
	$: round_count = game.rounds.length;
	$: waiting_for = players.filter(
		(player) =>
			!game.rounds[game.rounds.length - 1].answers.some((answer) => answer.player === player.name)
	);
</script>

<main>
	<h2>
		Round: {round_count}
	</h2>
	<h3>Waiting on players...</h3>
	{#each waiting_for as player}
		<div>
			{player}
		</div>
	{/each}
	<div>
		{current_question}
	</div>
	<div>

	</div>
	<div>
		<Button
			text={'reveal next question'}
			onClick={() => {
				setGameState('answer');
			}}
		/>
	</div>
</main>

<style>
	@import '../../app.css';
</style>
