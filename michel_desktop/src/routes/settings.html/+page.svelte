<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri'

    interface Plugin {
        name: string
    }

    let plugins: Plugin[] = [];

    invoke('get_plugins_list').then((fetched_plugins: Plugin[]) => {
        plugins = fetched_plugins;
    })
</script>

<div>
    <header>
        <h1>Settings for MICHEL</h1>
    </header>
    {#each plugins as plugin}
        <p>{plugin.name}</p>
    {/each}
</div>