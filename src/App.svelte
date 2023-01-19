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

</script>

<main class="container">
  <Menu />
</main>
