<script>
  import { onMount } from "svelte";
  import init, { greet, sum } from "../public/excel.js";

  let message = "";
  let result = 0;
  let cellValues = Array.from({ length: 10 }, () => Array(10).fill(""));

  onMount(async () => {
    try {
      // Initialize the Wasm module
      await init();
      // Call the greet function
      message = greet();
    } catch (error) {
      console.error("Failed to load Wasm module:", error);
    }
  });

  function parseCellAddress(cell) {
    const match = cell.match(/([A-Z]+)(\d+)/);
    if (!match) return null;

    const col = match[1];
    const row = parseInt(match[2], 10) - 1;

    let colIndex = 0;
    for (let i = 0; i < col.length; i++) {
      colIndex = colIndex * 26 + (col.charCodeAt(i) - 'A'.charCodeAt(0) + 1);
    }
    colIndex -= 1; // Zero-based index

    return { row, col: colIndex };
  }

  function parseCellRange(range) {
    const [start, end] = range.split(':').map(cell => cell.trim());

    const startAddr = parseCellAddress(start);
    const endAddr = end ? parseCellAddress(end) : startAddr;

    if (!startAddr || !endAddr) return [];

    const cells = [];
    for (let row = startAddr.row; row <= endAddr.row; row++) {
      for (let col = startAddr.col; col <= endAddr.col; col++) {
        cells.push({ row, col });
      }
    }
    return cells;
  }

  function evaluateSumFormula(formula) {
    const cellReferences = formula.replace('=SUM(', '').replace(')', '').trim().split(',');

    const cells = cellReferences.flatMap(ref => {
      if (ref.includes(':')) {
        return parseCellRange(ref);
      } else {
        const cell = parseCellAddress(ref);
        return cell ? [cell] : [];
      }
    });

    const values = cells.map(({ row, col }) => {
      return parseFloat(cellValues[row][col]) || 0;
    });

    return sum(values);
  }

  function handleInput(event) {
    const target = event.target;
    const value = target.innerText.trim();
    const { row, col } = target.dataset;

    cellValues[row][col] = value;

    if (value.startsWith('=SUM(') && value.endsWith(')')) {
      const result = evaluateSumFormula(value);
      target.innerText = result;
    } else {
      target.innerText = value;
    }
  }

  function handleFocus(event) {
    const target = event.target;
    const { row, col } = target.dataset;
    const value = cellValues[row][col];
    target.innerText = value;
  }

  function handleKeyDown(event) {
    const target = event.target;
    const { row, col } = target.dataset;

    if (event.key === "Enter" && !event.metaKey) {
      event.preventDefault();
      const nextRow = parseInt(row) + 1;
      if (nextRow < 10) {
        const nextCell = document.querySelector(`[data-row='${nextRow}'][data-col='${col}']`);
        if (nextCell) {
          nextCell.focus();
        }
      } else {
        target.blur();
      }
    } else if (event.key === "Enter" && event.metaKey) {
      document.execCommand('insertLineBreak');
    }
  }

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

<main>
  <p>{message}</p>
  <p>SUM: {result}</p>
  <table>
    {#each rows as row}
      <tr>
        {#each cols as col}
          <td
            id={`cell-${row}-${col}`}
            contenteditable="true"
            data-row={row}
            data-col={col}
            on:blur={handleInput}
            on:focus={handleFocus}
            on:keydown={handleKeyDown}
          ></td>
        {/each}
      </tr>
    {/each}
  </table>
</main>
