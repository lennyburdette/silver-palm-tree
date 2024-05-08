// place files you want to import through the `$lib` alias in this folder.

import type { Uri, editor } from "monaco-editor";

import EditorWorker from "monaco-editor/esm/vs/editor/editor.worker.js?worker";
import JsonWorker from "monaco-editor/esm/vs/language/json/json.worker?worker";

let pkg: typeof import("monaco-editor") | null = null;
export async function initMonaco() {
  pkg = await import("monaco-editor");

  window.MonacoEnvironment = {
    async getWorker(_workerId: string, label: string) {
      if (label === "json") {
        return new JsonWorker();
      }
      // if (label === "css" || label === "scss" || label === "less") {
      //   return new Worker(new URL("css.worker.js", import.meta.url).href);
      // }
      // if (label === "html" || label === "handlebars" || label === "razor") {
      //   return new Worker(new URL("html.worker.js", import.meta.url).href);
      // }
      // if (label === "typescript" || label === "javascript") {
      //   return new Worker(new URL("ts.worker.js", import.meta.url).href);
      // }

      return new EditorWorker();
    },
  };

  return pkg;
}

export function monaco(
  el: HTMLElement,
  options: editor.IStandaloneEditorConstructionOptions
) {
  if (pkg) {
    let e = pkg.editor.create(el, options);
    return {
      destroy() {
        e?.dispose();
      },
    };
  }
}

export function createModel(
  value: string,
  language?: string | undefined,
  uri?: Uri | undefined
) {
  return pkg!.editor.createModel(value, language, uri);
}
