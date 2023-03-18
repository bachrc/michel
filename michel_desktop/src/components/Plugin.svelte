<script lang="ts">
    import {invoke} from "@tauri-apps/api/tauri";

    export let plugin: PluginInfo;

    let indexing: boolean = false;

    function index() {
        indexing = true
        console.log("lezgo")
        console.log(plugin.identifier)
        invoke("run_plugin_index", {"identifier" : plugin.identifier})
            .then(() => indexing = false)
    }
</script>

<div class="box">
    <div class="name">
        <h1>{plugin.name}</h1>
        <p>{plugin.description}</p>
    </div>

    <div class="operations">
        {#if plugin.can_index}
            {#if indexing}
                <span>Indexing...</span>
            {:else}
                <button on:click={index}>Index</button>
            {/if}
        {/if}
    </div>
</div>

<style>
    .box {
        box-shadow: 5px 5px 2px 0px rgba(0,0,0,0.75);
        -webkit-box-shadow: 5px 5px 2px 0px rgba(0,0,0,0.5);
        -moz-box-shadow: 5px 5px 2px 0px rgba(0,0,0,0.5);

        display: flex;
        align-items: center;
        flex-direction: row;
        padding: 10px;
        border: 1px solid #322949;
        width: 100%;
    }

    .name {
        display: flex;
        flex-direction: column;
        flex-grow: 1;
    }

    .name h1 {
        font-size: 1.1em;
        font-weight: bold;
    }

    .operations {
        display: flex;
        flex-direction: row;
    }

</style>