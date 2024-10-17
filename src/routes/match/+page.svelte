<script>
    import {invoke} from "@tauri-apps/api/core";

    let showModal = false;
    let stadium = '';
    let home = '';
    let s2 = '';
    let result = '';
    let is_win = false;

    let matchCreated = false;
    let match = null;
    let countdownTime = 5400; // 90 minutes = 5400 seconds
    let timerInterval;

    // Mock data for stadiums, teams
    const stadiums = ['Stadium A', 'Stadium B', 'Stadium C'];
    const teams = [1, 2, 3];

    // Function to handle form submission
    async function createMatch() {
        const data = await invoke('create_new_game', {
            game: {
                id: 4,
                stadium: stadium,
                s1: home,
                s2: s2,
                date: new Date(),
                result: result,
                is_win: is_win,
            }
        })
        if (data) {
            // console.log({
            //     stadium,
            //     home,
            //     s2,
            //     result,
            //     is_win
            // });
            matchCreated = true;
            match = {
                stadium,
                home,
                s2,
                result,
                is_win
            };
            alert('Match Created!');
            showModal = false;
            startCountdown();

        }
        // Close the modal after submission

    }

    function startCountdown() {
        if (timerInterval) clearInterval(timerInterval);
        timerInterval = setInterval(() => {
            countdownTime--;
            if (countdownTime <= 0) {
                clearInterval(timerInterval);
            }
        }, 1000);
    }

    // Helper to format countdown time
    function formatTime(seconds) {
        const mins = Math.floor(seconds / 60);
        const secs = seconds % 60;
        return `${mins}:${secs < 10 ? '0' : ''}${secs}`;
    }

    // Reset form after submission
    function resetForm() {
        stadium = '';
        home = '';
        s2 = '';
        result = '';
        is_win = false;
        matchCreated = false;
        countdownTime = 5400;
    }

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
        z-index: 999;
    }

    .modal-content {
        background-color: #121212;
        padding: 2rem;
        border-radius: 8px;
        width: 90%;
        max-width: 500px;
        position: relative;
    }

    .form-group {
        margin-bottom: 1.5rem;
    }

    .form-group label {
        color: black;
        display: block;
        margin-bottom: 0.5rem;
    }

    .form-group select,
    .form-group input[type="text"] {
        width: 100%;
        padding: 0.75rem;
        border-radius: 4px;
        background-color: #333;
        color: black;
        border: 1px solid #444;
    }

    .form-group select:focus,
    .form-group input[type="text"]:focus {
        outline: none;
        border-color: #1db954;
    }

    .form-group input[type="checkbox"] {
        margin-right: 0.5rem;
    }

    .submit-btn {
        padding: 0.75rem 1.5rem;
        background-color: #1db954;
        color: white;
        border: none;
        border-radius: 50px;
        cursor: pointer;
        transition: background-color 0.3s ease;
    }

    .submit-btn:hover {
        background-color: #1ed760;
    }

    .submit-btn:focus {
        outline: none;
    }

    .close-btn {
        position: absolute;
        top: 10px;
        right: 10px;
        background: none;
        border: none;
        font-size: 1.5rem;
        color: #fff;
        cursor: pointer;
    }

    .add-match-button {
        margin-top: 1rem;
        padding: 0.5rem 1rem;
        background-color: #1db954;
        color: white;
        border: none;
        border-radius: 50px;
        cursor: pointer;
        transition: background-color 0.3s ease;
    }

    .add-match-button:hover {
        background-color: #1ed760;
    }

    .add-match-button:focus {
        outline: none;
    }

    .match-info {
        margin-top: 2rem;
        background-color: #333;
        padding: 1.5rem;
        border-radius: 10px;
        color: white;
    }

    .match-info h3 {
        color: #1db954;
    }

    .match-info p {
        margin: 0.5rem 0;
        color: #b3b3b3;
    }

    .countdown {
        font-size: 1.5rem;
        font-weight: bold;
        color: #1db954;
        margin-top: 1rem;
    }

    .countdown-bar {
        background-color: #1db954;
        height: 10px;
        border-radius: 5px;
        margin-top: 0.5rem;
    }

</style>

<!-- Button to trigger modal -->
<button class="add-match-button" on:click={() => (showModal = true)}>Create New Match</button>

<!-- Modal Popup -->
{#if showModal}
    <div class="modal">
        <div class="modal-content">
            <button class="close-btn" on:click={() => (showModal = false)}>&times;</button>
            <h2 style="color: #fff;">Create New Match</h2>

            <!-- Stadium Selection -->
            <div class="form-group">
                <label for="stadium">Stadium:</label>
                <select id="stadium" bind:value={stadium}>
                    <option value="" disabled selected>Select Stadium</option>
                    {#each stadiums as stadium}
                        <option value={stadium}>{stadium}</option>
                    {/each}
                </select>
            </div>

            <!-- Home Team Selection -->
            <div class="form-group">
                <label for="home">Home Team:</label>
                <select id="home" bind:value={home}>
                    <option value="" disabled selected>Select Home Team</option>
                    {#each teams as team}
                        <option value={team}>{team}</option>
                    {/each}
                </select>
            </div>

            <!-- S2 Team Selection -->
            <div class="form-group">
                <label for="s2">Opponent Team:</label>
                <select id="s2" bind:value={s2}>
                    <option value="" disabled selected>Select Opponent Team</option>
                    {#each teams as team}
                        <option value={team}>{team}</option>
                    {/each}
                </select>
            </div>

            <!-- Result Input -->
            <div class="form-group">
                <label for="result">Result:</label>
                <input id="result" type="text" bind:value={result} placeholder="Enter result"/>
            </div>

            <!-- Win Checkbox -->
            <div class="form-group">
                <label>
                    <input type="checkbox" bind:checked={is_win}/>
                    Is Win
                </label>
            </div>

            <!-- Submit Button -->
            <button class="submit-btn" on:click={createMatch}>Create Match</button>
        </div>
    </div>
{/if}

<!-- Display match info and countdown if match is created -->
{#if matchCreated}
    <div class="match-info">
        <h3>Match Details</h3>
        <p><strong>Stadium:</strong> {match.stadium}</p>
        <p><strong>Home Team:</strong> {match.home}</p>
        <p><strong>Opponent Team:</strong> {match.s2}</p>
        <p><strong>Result:</strong> {match.result}</p>
        <p><strong>Is Win:</strong> {match.is_win ? 'Yes' : 'No'}</p>

        <div class="countdown">
            Time Remaining: {formatTime(countdownTime)}
        </div>
        <div class="countdown-bar">
            <div style="width: {Math.round((5400 - countdownTime) / 5400 * 100)}%; height: 100%; background-color: #1db954; transition: width 1s linear;"></div>
        </div>
    </div>
{/if}