<!-- src/routes/__layout.svelte -->
<script>
	let pages = [
		{ name: "Match", route: "/match", icon: "🏠" },
		{ name: "Team", route: "/team", icon: "🔍" },
		{ name: "History", route: "/history", icon: "🔍" },
		{ name: "Point", route: "/point", icon: "🔔" },
		{ name: "Profile", route: "/profile", icon: "👤" }
	];
</script>

<style>
	.app-layout {
		display: flex;
		flex-direction: column;
		height: 100vh;
		background: black;
		color: white;
	}
	.content {
		flex-grow: 1;
		padding: 1rem;
		overflow: auto;
	}
	.bottom-nav {
		display: flex;
		justify-content: space-around;
		align-items: center;
		background-color: black;
		border-top: 1px solid #ddd;
		height: 60px;
	}
	.nav-item {
		text-align: center;
		flex-grow: 1;
	}
	.nav-item a {
		text-decoration: none;
		font-size: 1.2rem;
		color: white;
	}

</style>

<div class="app-layout">
	<div class="content">
		<slot></slot> <!-- The main content of each page goes here -->
	</div>
	<nav class="bottom-nav">
		{#each pages as page}
			<div class="nav-item">
				<a href="{page.route}">
					<span>{page.icon}</span><br />
					<span>{page.name}</span>
				</a>
			</div>
		{/each}
	</nav>
</div>
