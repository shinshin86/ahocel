<script>
import { onMount } from "svelte";
  import init, { greet } from "../public/excel.js";

  let message = "";

  onMount(async () => {
    try {
      // Initialize the Wasm module
      await init();
      // Call the exported function
      message = greet();
    } catch (error) {
      console.error("Failed to load Wasm module:", error);
    }
  });

  let rows = Array.from({ length: 10 }, (_, i) => i);
  let cols = Array.from({ length: 10 }, (_, i) => i);
</script>

<style>
  table {
      border-collapse: collapse;
      width: 100%;
      height: 100%;
  }
  td {
      border: 1px solid #ccc;
      width: 40px;
      height: 40px;
      text-align: center;
  }
</style>

<table>
  <p>{message}</p>
  {#each rows as row}
      <tr>
          {#each cols as col}
              <td contenteditable="true"></td>
          {/each}
      </tr>
  {/each}
</table>
