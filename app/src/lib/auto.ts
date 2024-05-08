export function auto(input: any) {
  try {
    let out = JSON.parse(input);
    out = r(out);
    out = out ? p(out, "") : "";
    return out;
  } catch (e) {
    return "";
  }
}

type O = { [key: string]: { alias?: string; children?: O }[] };

function r(input: any): O | null {
  if (Array.isArray(input)) {
    return input.map(r).reduce((acc, o) => ({ ...acc, ...o }));
  } else if (input && typeof input === "object") {
    return Object.fromEntries(
      Object.entries(input).map(([k, v]) => {
        let alias = k.includes("_")
          ? k.replace(/_([a-z])/g, (_, a) => a.toUpperCase())
          : null;
        const children = r(v);
        return [
          k,
          { ...(alias ? { alias } : {}), ...(children ? { children } : {}) },
        ];
      })
    ) as O;
  } else {
    return null;
  }
}

function p(output: O, indent = ""): string {
  return Object.entries(output)
    .map(([key, value]) => {
      let fullKey = "alias" in value ? `${value.alias}: ${key}` : key;
      let children = "children" in value ? (value.children as O) : {};

      if (Object.keys(children).length) {
        return [
          `${indent}${fullKey} {`,
          p(children, indent + "  "),
          `${indent}}`,
        ].join("\n");
      } else {
        return `${indent}${fullKey}`;
      }
    })
    .join("\n");
}
