<script>
    import { invoke } from '@tauri-apps/api/tauri';

    let searchInput = "";
    let searchResult = "";

    function emitSearchInputChangeEvent(currentInputString) {
        invoke('get_search_results', {
            searchTerm: searchInput
        }).then(
            (res) => { searchResult = res; }
        );
    }
</script>

<div class="search-container">
    <div class="hyprspace-search-bar">
        <input 
        type="text"
        placeholder="Search ... "
        bind:value={ searchInput }
        on:input ={ () => emitSearchInputChangeEvent( searchInput ) }
        />
    </div>
    <div class="hyprspace-search-results">
        {@html searchResult }
    </div>
</div>