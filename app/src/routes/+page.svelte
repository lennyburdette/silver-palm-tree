<script lang="ts">
  import { browser } from '$app/environment';
  import { onMount } from 'svelte';
  import init, { parse_selection_and_apply_to } from 'wasm-bridge';

  let initted = false;
  onMount(() => init().then(() => initted = true));

  let selection = 'foo\nbar\nbaz {\n  quux\n}';
  let response = JSON.stringify({ foo: "1", bar: "2", baz: { "quux": "3" } }, null, 2);
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
</script>

<div class="fixed inset-0 flex flex-col">
  <div class="h-12 p-3 bg-slate-700 text-white">JSONSelection Playground</div>
  <main class="grow grid grid-cols-2 grid-rows-2 inset-0 gap-3 bg-slate-50 p-3">
    <div><textarea bind:value={selection} class="bg-white w-full h-full p-3 font-mono"></textarea></div>
    <div><textarea bind:value={response} class="bg-white w-full h-full p-3 font-mono"></textarea></div>
    <div><pre class="bg-white w-full h-full p-3">{JSON.stringify(result, null, 2)}</pre></div>
    <div><pre class="bg-white w-full h-full p-3">{JSON.stringify(errors, null, 2)}</pre></div>
  </main>
</div>