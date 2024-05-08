<script lang="ts">
  import { browser } from "$app/environment";
  import { onMount } from "svelte";
  import init, { parse_selection_and_apply_to } from "wasm-bridge";
  import { createModel, initMonaco, monaco } from "$lib";
  import type { editor } from "monaco-editor";
  import { examples } from "$lib/examples";

  let selectedExample = examples[0];
  let selection = selectedExample.selection;
  let response = selectedExample.response;

  let initted = false;
  onMount(() => init().then(() => (initted = true)));

  let result = {};
  let errors: string[] = [];
  $: if (browser && initted) {
    let out = parse_selection_and_apply_to(selection, response);
    if (out.errors) {
      errors = out.errors;
    }
    if (out.value) {
      result = out.value;
    }
  }

  let selectionModel: editor.ITextModel | undefined;
  let responseModel: editor.ITextModel | undefined;
  let resultModel: editor.ITextModel | undefined;

  let hasMonaco = false;
  onMount(() =>
    initMonaco().then(() => {
      hasMonaco = true;

      selectionModel = createModel(selection);
      responseModel = createModel(response, "json");
      resultModel = createModel(JSON.stringify(result, null, 2), "json");

      selectionModel.onDidChangeContent(() => {
        selection = selectionModel?.getValue() || "";
      });
      responseModel.onDidChangeContent(() => {
        response = responseModel?.getValue() || "";
      });
    })
  );

  $: resultModel?.setValue(JSON.stringify(result, null, 2));
</script>

<div class="wrapper">
  <div
    style="grid-area: header"
    class="h-12 p-3 bg-slate-700 text-white flex justify-between"
  >
    <h1>JSONSelection Playground</h1>
    <select
      class="bg-slate-700 text-white"
      bind:value={selectedExample}
      on:change={() => {
        selection = selectedExample.selection;
        response = selectedExample.response;
        selectionModel?.setValue(selection);
        responseModel?.setValue(response);
      }}
    >
      {#each examples as example}
        <option value={example}>{example.title}</option>
      {/each}
    </select>
  </div>
  {#if hasMonaco}
    <div
      style="grid-area: selection"
      use:monaco={{ model: selectionModel, minimap: { enabled: false } }}
    ></div>

    <div
      style="grid-area: response"
      use:monaco={{ model: responseModel, minimap: { enabled: false } }}
    ></div>

    <div
      style="grid-area: result"
      class="border-t border-gray-200"
      use:monaco={{
        model: resultModel,
        minimap: { enabled: false },
        readOnly: true,
      }}
    ></div>
  {:else}
    <div style="grid-area: selection"></div>
    <div style="grid-area: response"></div>
    <div style="grid-area: result"></div>
  {/if}

  <div style="grid-area: errors" class="border-t border-gray-200">
    {#each errors as error}
      <div class="bg-red-100 text-red-800 p-3 text-sm mb-0.5">{error}</div>
    {/each}
  </div>
</div>

<style>
  .wrapper {
    height: 100vh;
    display: grid;
    grid-template-areas:
      "header header"
      "selection response"
      "result errors";

    grid-template-rows: 50px calc(50% - 25px) calc(50% - 25px);
    grid-template-columns: 1fr 1fr;
  }
</style>
