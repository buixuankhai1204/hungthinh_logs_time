<!-- src/routes/index.svelte (or any other page where the modal is needed) -->
<script lang="ts">
    import {invoke} from "@tauri-apps/api/core";
    import {onMount} from "svelte";

    let showNewTeamModal = false;
    let name = "";
    let size = 0;
    let current_size = 0;
    let teams: Team[] = [];
    async function fetchTeams() {
        const response: Team[] = await invoke('get_all_teams');
        teams = response;
    }

    async function addTeam() {
        const data: Team = await invoke('create_new_team', {
            team: {
                id: 2,
                name: name,
                size: size,
                current_size
            }
        })
        if (data) {
            teams = [...teams, data]
            alert('Team added successfully!');
            showNewTeamModal = false; // Close the modal after successful submission
        } else {
            alert('Failed to add team.');
        }
    }
    onMount(fetchTeams);

</script>

<style>
    .modal {
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        background-color: rgba(0, 0, 0, 0.5);
        display: flex;
        justify-content: center;
        align-items: center;
    }

    .modal-content {
        background-color: white;
        padding: 2rem;
        border-radius: 8px;
        width: 90%;
        max-width: 500px;
        position: relative;
    }

    .form-group {
        margin-bottom: 1rem;
    }

    .form-group label {
        display: block;
        margin-bottom: 0.5rem;
    }

    .form-group input {
        width: 100%;
        padding: 0.5rem;
        border-radius: 4px;
        border: 1px solid #ddd;
    }

    .submit-btn {
        padding: 0.5rem 1rem;
        background-color: #007bff;
        color: white;
        border: none;
        border-radius: 4px;
        cursor: pointer;
    }

    .close-btn {
        position: absolute;
        top: 10px;
        right: 10px;
        background: none;
        border: none;
        font-size: 1.5rem;
        cursor: pointer;
    }

    .team-table {
        margin-top: 2rem;
        width: 100%;
        border-collapse: collapse;
        font-family: 'Helvetica Neue', Helvetica, Arial, sans-serif;
    }

    .team-table thead {
        background-color: #121212;
        color: #fff;
        text-align: left;
    }

    .team-table th,
    .team-table td {
        padding: 1rem;
        border-bottom: 1px solid #333;
    }

    .team-table tbody tr {
        transition: background-color 0.3s ease;
    }

    .team-table tbody tr:hover {
        background-color: #282828;
    }

    .team-table th {
        font-weight: bold;
        font-size: 14px;
        text-transform: uppercase;
        letter-spacing: 0.1em;
    }

    .team-table td {
        color: #b3b3b3;
        font-size: 14px;
    }

    .team-table td:first-child {
        font-weight: bold;
        color: white;
    }

    .add-team-button {
        margin-top: 1rem;
        padding: 0.5rem 1rem;
        background-color: #1db954;
        color: white;
        border: none;
        border-radius: 50px;
        cursor: pointer;
        transition: background-color 0.3s ease;
    }

    .add-team-button:hover {
        background-color: #1ed760;
    }

    .add-team-button:focus {
        outline: none;
    }
</style>

<!-- Trigger button to open modal -->
<button class="add-team-button" on:click={() => (showNewTeamModal = true)}>Add New Team</button>

<!-- Display all teams in a table -->
<div>
    <table class="team-table">
        <thead>
        <tr>
            <th>Name</th>
            <th>Size</th>
            <th>Current Size</th>
        </tr>
        </thead>
        <tbody>
        {#if teams.length > 0}
            {#each teams as team}
                <tr>
                    <td>{team.name}</td>
                    <td>{team.size}</td>
                    <td>{team.current_size}</td>
                </tr>
            {/each}
        {:else}
            <tr>
                <td colspan="3">No teams found.</td>
            </tr>
        {/if}
        </tbody>
    </table>
</div>

<!-- Modal Popup -->
{#if showNewTeamModal}
    <div class="modal">
        <div class="modal-content">
            <button class="close-btn" on:click={() => (showNewTeamModal = false)}>&times;</button>
            <h2>Add New Team</h2>

            <div class="form-group">
                <label for="name">Team Name:</label>
                <input id="name" type="text" bind:value={name} placeholder="Enter team name" required/>
            </div>

            <div class="form-group">
                <label for="size">Team Size:</label>
                <input id="size" type="number" bind:value={size} placeholder="Enter team size" required/>
            </div>

            <div class="form-group">
                <label for="current_size">Current Size:</label>
                <input id="current_size" type="number" bind:value={current_size} placeholder="Enter current size"
                       required/>
            </div>

            <button class="submit-btn" on:click={addTeam}>Submit</button>
        </div>
    </div>
{/if}
