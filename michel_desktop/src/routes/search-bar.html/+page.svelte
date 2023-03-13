<script>
    //import { onFocus } from 'svelte'
    let usernameInput

    function focusOnInput() {
        usernameInput.focus();
    }
</script>

<div class="panel" on:focus={focusOnInput}>
    <header>
        MICHEL
    </header>
    <nav>
        <input
            name="search-content"
            bind:this={usernameInput}
            class="search-bar"
        />
    </nav>
    <main>
        pouet
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
        flex-grow: 1;
    }

</style>