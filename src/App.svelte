<script>
  import Menu from "./lib/Menu.svelte";

  import { invoke, convertFileSrc } from "@tauri-apps/api/tauri";

  /**
   * Here we get the path to the style.css file located in the .config directory
   * and the we dynamically append it to the DOM
  */
  invoke('get_style_sheet_path').then(
    (path) => {
      let linkElem = document.createElement('link');
      linkElem.rel = "stylesheet";
      linkElem.href = convertFileSrc(path);

      document.head.appendChild(linkElem);
    }
  )


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
                    emit("reload-apps", {
                      reload: true,
                      message: "reload of applications requested by frontend",
                      component_of_origin: "App"
                    })
                },
                function (error) {
                    emit("error-message", {
                        error: error.toString(),
                        message:
                            "An Error Occurred in component: Menu.svelte ==> Check the Logs. Error was: " +
                            error.toString(),
                        component_of_origin: "App",
                    });
                }
            )
            .catch((err) => {
                emit("error-message", {
                    error: err.toString(),
                    message:
                        "An Error occurred in component: Menu.svelte ==> Check the Logs. Error was: " +
                        err.toString(),
                    component_of_origin: "App",
                });
            });
    }
    

</script>

<!-- Configuration of Window -->
<svelte:window on:keydown={handleKeyDownEvent} />

<main class="container">
  <Menu />
</main>
