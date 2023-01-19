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

    import { emit, listen } from "@tauri-apps/api/event";
    import { appWindow } from "@tauri-apps/api/window";

    /**
     * This function is used in the handling of shortcuts.
     *
     * @var event - KeyDown Event
     */
    function handleKeyDownEvent(event) {
        let key = event.key;
        let code = event.code;

        console.log(event.key);
        console.log(event.code);

        if (code == "Escape") {
            minimizeWindow();
        }

        return event;
    }

    /**
     * This function will be called if the user presses the Esc - Key.
     * On pressing the Esc - Key the window will minimize, waiting to be re-opened.
     * Once the window minimizes an event will be emitted that will prompt the backend
     * into reloading the application and command buffer.
     *
     * This is useful, if a new Application has been installed. Just close and re-open the application
     * to gain access to it.
     * By Minimizing the Application we reduce start-up time.
     * The Application will be started once and then simply remain in the Background until called again with a shortcut
     * defined in the WM - Settings
     */
    function minimizeWindow() {
        appWindow
            .hide()
            .then(
                function (value) {
                    console.log('minimize');
                },
                function (error) {
                    emit("error-message", {
                        error: error.toString(),
                        message:
                            "An Error Occurred in component: Menu.svelte ==> Check the Logs. Error was: " +
                            error.toString(),
                        component_of_origin: "Menu",
                    });
                }
            )
            .catch((err) => {
                emit("error-message", {
                    error: err.toString(),
                    message:
                        "An Error occurred in component: Menu.svelte ==> Check the Logs. Error was: " +
                        err.toString(),
                    component_of_origin: "Menu",
                });
            });
    }
    
</script>

<!-- Configuration of Window -->
<svelte:window on:keydown={handleKeyDownEvent} />

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
