<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri'
    import Plugin from "../../components/Plugin.svelte";

    let plugins: PluginInfo[] = [];

    invoke('get_plugins_list').then((fetched_plugins: PluginInfo[]) => {
        plugins = fetched_plugins;
    })
</script>

<div class="main">
    <header>
        <h1>Settings for MICHEL</h1>
    </header>
    <hr/>
    <h2>Plugins</h2>
    <div class="plugin-list">
        {#each plugins as plugin}
            <Plugin plugin="{plugin}"/>
        {/each}
    </div>
</div>

<style>
    .main {
        background: #c6adf8;
        height: 100vh;
        padding: 10px;
        box-sizing: border-box;
    }

    h1 {
        font-size: 1.75em;
        font-weight: bold;
    }

    h2 {
        font-weight: bold;
        font-size: 1.5em;
        padding-bottom: 0.5em;
    }

    .plugin-list {
        display: flex;
        gap: 5px;
    }
</style>