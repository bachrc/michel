<script lang="ts">
    export let entry: Entry;
</script>

<div>
    <h1>{entry.title}</h1>
    <h3>{entry.description}</h3>
</div>

<style>
    div {
        display: flex;
        flex-direction: column;
        padding: 5px;
        border: 1px solid mediumpurple;
        border-radius: 2px;
        box-sizing: border-box;
    }

    h1 {
        font-weight: bold;
    }

    h3 {
        font-size: 0.6em;
    }
</style>