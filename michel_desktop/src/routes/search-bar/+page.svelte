<script lang="ts">
    import {invoke} from "@tauri-apps/api/tauri";

    let entries: Entry[] = [];
    let timer;
    let value = "";

    function on_key_pressed(e) {
        clearTimeout(timer);

        timer = setTimeout(fetch_entries, 1000);
    }

    function fetch_entries() {
        invoke("fetch_entries_for_input", {input: value})
            .then((returned_entries: Entry[]) => {
                entries = returned_entries;
            })
    }

</script>

<div class="panel" >
    <header>
        MICHEL
    </header>
    <nav>
        <input
            name="search-content"
            bind:value
            on:keyup={on_key_pressed}
            class="search-bar"
        />
    </nav>
    <main>
        {#each entries as entry}
            <span>{entry.title}</span>
        {/each}
    </main>

</div>

<style>

    .panel {
        background: rgb(227,174,238);
        background: linear-gradient(133deg, rgba(227,174,238,1) 0%, rgba(141,127,245,1) 100%);
        box-sizing: border-box;
        display: flex;
        flex-direction: column;
        align-items: stretch;
        height: 100vh;
        padding: 8px;
    }

    nav {
    }

    input {
        box-sizing: border-box;
        width: 100%;
        height: 2.2em;
        border: 1px solid rgba(56, 37, 123, 0.3);
        border-radius: 8px;
        padding: 8px;
        font-size: 2em;
        caret-color: #5e4d85;
    }

    main {
        display: flex;
        flex-direction: column;
    }

</style>