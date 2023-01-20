<!--  
    The Menu contains the following:
    - A Searchbar for Applications
    - A Results window for search results
    - A FavBar for favourite Applications
    - System information (such as packages, available updates etc)

    Style is a Tryptichon
-->
<script>
    import FavBar from "./components/FavBar.svelte";
    import Monitor from "./components/Monitor.svelte";
    import Packages from "./components/Packages.svelte";
    import SearchBar from "./components/SearchBar.svelte";
    import SysInfo from "./components/SysInfo.svelte";
</script>



<div class="hyprspace-menu">
    <canvas class="hyprspace-backdrop" />
    <div class="hyprspace-search">
        <SearchBar />
    </div>
    <div class="hyprspace-fav-menu">
        <FavBar />
    </div>
    <div class="hyprspace-system">
        <SysInfo />
        <Packages />
        <Monitor />
    </div>
</div>
