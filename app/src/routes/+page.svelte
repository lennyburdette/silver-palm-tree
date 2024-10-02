<script lang="ts">
  import { browser } from "$app/environment";
  import { onMount } from "svelte";
  import init, { parse_selection_and_apply_to } from "wasm-bridge";
  import { createModel, initMonaco, monaco } from "$lib";
  import type { editor } from "monaco-editor";
  import { examples } from "$lib/examples";
  import { auto } from "$lib/auto";

  function loadInitialExample() {
    const exampleId = window.location.hash.substring(1);
    const example = examples.find((ex) => ex.id === exampleId);
    return example ?? examples[0];
  }

  let selectedExample: (typeof examples)[0] | null = null;
  let selection = "";
  let response = "";
  let vars = "";

  let initted = false;
  onMount(() => init().then(() => (initted = true)));

  let result = {};
  let errors: string[] = [];
  $: if (browser && initted) {
    console.log(selection);
    if (!selectedExample) {
      selectedExample = selectedExample ?? loadInitialExample();
      selection = selectedExample.selection;
      response = selectedExample.response;
      vars = selectedExample.vars;
    }

    let out = parse_selection_and_apply_to(selection, response, vars);
    if (out.errors) {
      errors = out.errors;
    }
    if (out.value) {
      result = out.value;
    }
  }

  let selectionModel: editor.ITextModel | undefined;
  let responseModel: editor.ITextModel | undefined;
  let varsModel: editor.ITextModel | undefined;
  let resultModel: editor.ITextModel | undefined;

  let hasMonaco = false;
  onMount(() =>
    initMonaco().then(() => {
      hasMonaco = true;

      selectionModel = createModel(selection);
      responseModel = createModel(response, "json");
      resultModel = createModel(JSON.stringify(result, null, 2), "json");
      varsModel = createModel(vars, "json");

      selectionModel.onDidChangeContent(() => {
        selection = selectionModel?.getValue() || "";
      });
      responseModel.onDidChangeContent(() => {
        response = responseModel?.getValue() || "";
      });
      varsModel.onDidChangeContent(() => {
        vars = varsModel?.getValue() || "";
      });
    })
  );

  $: resultModel?.setValue(JSON.stringify(result, null, 2));

  import { afterUpdate } from "svelte";

  afterUpdate(() => {
    const handleHashChange = () => {
      const exampleId = window.location.hash.substring(1);
      const example = examples.find((ex) => ex.id === exampleId);
      if (example) {
        selectedExample = example;
        selectExample(example);
      }
    };

    window.addEventListener("hashchange", handleHashChange);

    return () => {
      window.removeEventListener("hashchange", handleHashChange);
    };
  });

  function selectExample(example: (typeof examples)[0]) {
    if (example) {
      window.location.hash = example.id;
      selectionModel?.setValue(example.selection);
      responseModel?.setValue(example.response);
      varsModel?.setValue(example.vars);
    }
  }
</script>

<div class="wrapper">
  <div
    style="grid-area: header"
    class="h-12 px-3 bg-slate-700 text-white flex justify-between items-center"
  >
    <h1>JSONSelection Playground</h1>
    <div class="space-x-2">
      <button
        class="text-sm border border-slate-300 rounded px-2 py-1 hover:bg-slate-600 hover:border-slate-200"
        on:click={() => {
          selection = auto(response);
          selectionModel?.setValue(selection);
        }}>Generate Selection</button
      >
      <select
        class="bg-slate-700 text-white text-sm border border-slate-300 rounded px-2 py-1"
        on:change={(e) =>
          selectExample(examples[parseInt(e.currentTarget.value)])}
      >
        {#each examples as example, i}
          <option value={i}>{example.title}</option>
        {/each}
      </select>
    </div>
  </div>
  {#if hasMonaco}
    <div class="with-title" style="grid-area: selection">
      <h2>Selection</h2>
      <div
        use:monaco={{ model: selectionModel, minimap: { enabled: false } }}
      ></div>
    </div>

    <div
      class="with-title border-l border-gray-200"
      style="grid-area: response"
    >
      <h2>JSON Response</h2>
      <div
        use:monaco={{ model: responseModel, minimap: { enabled: false } }}
      ></div>
    </div>

    <div class="with-title border-l border-gray-200" style="grid-area: vars">
      <h2>Variables</h2>
      <div
        class="border-t border-gray-200"
        use:monaco={{ model: varsModel, minimap: { enabled: false } }}
      ></div>
    </div>

    <div class="with-title border-t border-gray-200" style="grid-area: result">
      <h2>Result</h2>
      <div
        use:monaco={{
          model: resultModel,
          minimap: { enabled: false },
          readOnly: true,
        }}
      ></div>
    </div>
  {:else}
    <div style="grid-area: selection"></div>
    <div style="grid-area: response"></div>
    <div style="grid-area: vars"></div>
    <div style="grid-area: result"></div>
  {/if}

  <div
    class="with-title border-t border-l border-gray-200"
    style="grid-area: errors"
  >
    <h2>Errors</h2>
    <div>
      {#each errors as error}
        <div class="bg-red-100 text-red-800 p-3 text-sm mb-0.5">{error}</div>
      {/each}
    </div>
  </div>
</div>

<style>
  .wrapper {
    height: 100vh;
    display: grid;
    grid-template-areas:
      "header header"
      "selection result"
      "selection result"
      "response result"
      "response result"
      "vars errors"
      "vars errors";

    grid-template-rows: 50px 1fr 1fr 1fr 1fr 1fr 1fr;
    grid-template-columns: 1fr 1fr;
  }

  .with-title {
    height: 100%;
    display: grid;
    grid-template-rows: 30px 1fr;
  }

  .with-title h2 {
    @apply flex items-center px-2 uppercase text-xs tracking-widest font-semibold text-slate-500 border-b bg-slate-100;
  }
</style>
