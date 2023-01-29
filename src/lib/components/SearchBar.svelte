<script>
    import { invoke, convertFileSrc } from "@tauri-apps/api/tauri";
    import { emit } from "@tauri-apps/api/event";
    import { appWindow } from "@tauri-apps/api/window";

    let searchInput = "";
    let application_row = undefined;
    let exe_row = undefined;
    let buttons_array = undefined;
    let currentButton = 0;

    class Application {
        constructor(name, exe, path_to_icon) {
            this.name = name;
            this.exe = exe;
            this.path_to_icon = findFittingIcon(path_to_icon);
        }
    }

    class Executable {
        constructor(full_path, name) {
            this.full_path = full_path;
            this.name = name;
        }
    }

    class SearchResult {
        constructor(applications, executables) {
            let applications_array = [];
            let executables_array = [];

            applications.forEach((element) => {
                applications_array.push(
                    new Application(
                        element.name,
                        element.exe,
                        element.path_to_icon
                    )
                );
            });

            executables.forEach((element) => {
                executables_array.push(
                    new Executable(element.full_path, element.name)
                );
            });

            this.applications = applications_array;
            this.executables = executables_array;
        }
    }

    function findFittingIcon(iconSrc) {
        if (iconSrc === undefined || iconSrc === "") {
            return "/placeholder.svg";
        }

        return convertFileSrc(iconSrc);
    }

    function removeExistingChildrenFromResults(resultsDiv) {
        while (resultsDiv.firstChild) {
            resultsDiv.removeChild(resultsDiv.lastChild);
        }
    }

    function buildSearchResultObject(result) {
        return new SearchResult(result.applications, result.executables);
    }

    function appendResultBoxesToDocument(
        resultsDiv,
        applicationsElement,
        executablesElement
    ) {
        resultsDiv.appendChild(applicationsElement);
        resultsDiv.appendChild(executablesElement);
    }

    function buildApplicationsElement(applicationsObject) {
        let appContainer = document.createElement("div");
        appContainer.classList.add("hyprspace-apps");
        appContainer.id = "hyprspace-apps";

        applicationsObject.forEach((element) => {
            appContainer.appendChild(buildApplicationsButton(element));
        });

        return appContainer;
    }

    function buildApplicationsButton(application) {
        let appIcon = document.createElement("img");
        appIcon.src = application.path_to_icon;
        appIcon.alt = "Icon for " + application.name;
        appIcon.classList.add("hyprspace-app-icon");
        appIcon.id = application.name + "-icon";

        let appName = document.createElement("div");
        appName.innerHTML = application.name;
        appName.classList.add("hyprspace-app-name");
        appName.id = application.name + "-name";

        let exePath = document.createElement("div");
        exePath.innerHTML = application.exe;
        exePath.classList.add("hyprspace-app-exe");
        exePath.id = application.name + "-exe";

        let appButton = document.createElement("button");
        appButton.type = "button";
        appButton.classList.add("hyprspace-app-button");
        appButton.id = application.name + "-button";
        appButton.addEventListener("click", function () {
            emitExecutionRequestEvent(application.exe, "application");
        });

        /**
         * button
         *  -> Icon
         *  -> Name
         *      -> Exe
         */

        appButton.appendChild(appIcon);
        appButton.appendChild(appName);
        appName.appendChild(exePath);

        return appButton;
    }

    function buildExecutablesElement(executablesObject) {
        let exeContainer = document.createElement("div");
        exeContainer.classList.add("hyprspace-executables");
        exeContainer.id = "hyprspace-executables";

        executablesObject.forEach((element) => {
            exeContainer.appendChild(buildExecutableButton(element));
        });

        return exeContainer;
    }

    function buildExecutableButton(executable) {
        let icon = document.createElement("img");
        icon.src = "/placeholder.svg";
        icon.alt = "Icon for " + executable.name;
        icon.classList.add("hyprspace-exe-icon");
        icon.id = executable.name + "-icon";

        let exeName = document.createElement("div");
        exeName.innerHTML = executable.name;
        exeName.classList.add("hyprspace-exe-name");
        exeName.id = executable.name + "-name";

        let exePath = document.createElement("div");
        exePath.innerHTML = executable.full_path;
        exePath.classList.add("hyprspace-exe-fullpath");
        exePath.id = executable.name + "-fullpath";

        let exeButton = document.createElement("button");
        exeButton.type = "button";
        exeButton.classList.add("hyprspace-exe-button");
        exeButton.id = executable.name + "-button";
        exeButton.addEventListener("click", function () {
            emitExecutionRequestEvent(executable.full_path, "executable");
        });

        /**
         * Button
         *  -> Icon
         *  -> Name
         *      -> Full Path
         */

        exeButton.appendChild(icon);
        exeButton.appendChild(exeName);
        exeName.appendChild(exePath);

        return exeButton;
    }

    function toggleBetweenEmptyAndContains(resultsDiv, isEmpty) {
        if (!isEmpty) {
            resultsDiv.classList.remove("contains-results");
            resultsDiv.classList.add("empty");
        } else {
            resultsDiv.classList.remove("empty");
            resultsDiv.classList.add("contains-results");
        }
    }

    function buildSearchResultBoxes(result) {
        let resultsDiv = document.getElementById("search-result-box");

        removeExistingChildrenFromResults(resultsDiv);

        if (
            (result.applications === undefined ||
                result.applications.length === 0) &&
            (result.executables === undefined ||
                result.applications.length === 0)
        ) {
            application_row = undefined;
            exe_row = undefined;
            buttons_array = undefined;
            toggleBetweenEmptyAndContains(resultsDiv, false);
        } else {
            let resultObject = buildSearchResultObject(result);
            toggleBetweenEmptyAndContains(resultsDiv, true);

            let executablesElement = buildExecutablesElement(
                resultObject.executables
            );
            let applicationsElement = buildApplicationsElement(
                resultObject.applications
            );

            application_row = Array.from(applicationsElement.children);
            exe_row = Array.from(executablesElement.children);
            buttons_array = application_row.concat(exe_row);

            appendResultBoxesToDocument(
                resultsDiv,
                applicationsElement,
                executablesElement
            );
        }
    }

    function minimizeWindow() {
        let resultsDiv = document.getElementById("search-result-box");
        searchInput = "";
        application_row = undefined;
        exe_row = undefined;
        buttons_array = undefined;
        removeExistingChildrenFromResults(resultsDiv);
        appWindow
            .hide()
            .then(
                function (p) {
                    invoke("get_search_results", {
                        searchTerm: "",
                        reloadCache: true,
                    });
                },
                function (error) {
                    emit("error-message", {
                        error: error.toString(),
                        message:
                            "An Error Occurred in component: SearchBar.svelte ==> Check the Logs. Error was: " +
                            error.toString(),
                        component_of_origin: "App",
                    });
                }
            )
            .catch((err) => {
                emit("error-message", {
                    error: err.toString(),
                    message:
                        "An Error occurred in component: SearchBar.svelte ==> Check the Logs. Error was: " +
                        err.toString(),
                    component_of_origin: "App",
                });
            });
    }

    function emitExecutionRequestEvent(exe, type) {
        invoke("handle_execution_request", {
            exePath: exe,
            typeOfExe: type,
        });
        let resultsDiv = document.getElementById("search-result-box");
        removeExistingChildrenFromResults(resultsDiv);
        searchInput = "";
        minimizeWindow();
    }

    function emitSearchInputChangeEvent(currentInputString) {
        invoke("get_search_results", {
            searchTerm: searchInput,
            reloadCache: false,
        }).then((res) => {
            buildSearchResultBoxes(res);
        });
    }

    function initial_focus(el) {
        el.focus();
    }

    function handleKeyDownEvent(event) {
        let key = event.key;
        let code = event.code;

        console.log(key);

        if (key === "Escape" || code === "Escape") {
            minimizeWindow();
        } else if (buttons_array !== undefined && (key === "ArrowDown" || code === "ArrowDown")) {
            moveSelectionDown();
        } else if(buttons_array !== undefined && (key === "ArrowUp" || code === "ArrowUp")) {
            moveSelectionUp();
        } else if(key === "Backspace" || code === "Backspace") {
            if(searchInput.length > 0) {
                searchInput = searchInput.substring(0, searchInput.length - 1);
            }
        } 
    }

    function moveSelectionUp() {
        currentButton -= 1;
        let currentlyFocused = document.activeElement;
        if (currentlyFocused.id === "hyprspace-search-bar-input") {
            buttons_array[buttons_array.length - 1].focus();
            currentButton = buttons_array.length - 1;
        } else if (currentButton >= 0) {
            buttons_array[currentButton].focus();
        } else {
            document.getElementById("hyprspace-search-bar-input").focus();
        }
    }

    function moveSelectionDown() {
        currentButton += 1;
        let currentlyFocused = document.activeElement;
        if (currentlyFocused.id === "hyprspace-search-bar-input") {
            buttons_array[0].focus();
            currentButton = 0;
        } else if (currentButton < buttons_array.length) {
            buttons_array[currentButton].focus();
        } else {
            document.getElementById("hyprspace-search-bar-input").focus();
        }
    }
</script>

<!-- Configuration of Window -->
<svelte:window on:keydown={handleKeyDownEvent} />

<div class="search-container">
    <div class="hyprspace-search-bar">
        <input
            type="text"
            placeholder="Search ... "
            class="hyprspace-search-bar-input"
            id="hyprspace-search-bar-input"
            bind:value={searchInput}
            on:input={() => emitSearchInputChangeEvent(searchInput)}
            use:initial_focus
        />
    </div>
    <div class="hyprspace-search-results empty" id="search-result-box" />
</div>
