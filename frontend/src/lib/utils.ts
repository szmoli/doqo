import type { DoqoSymbol } from "./bindings/DoqoSymbol";
import type { DoqoRegistry } from "./bindings/DoqoRegistry";

export function symbolName(symbol: DoqoSymbol): string {
  return symbol.fqid.split("::").pop() ?? symbol.fqid
}

export function hasChildren(symbol: DoqoSymbol): boolean {
  return symbol.children.length > 0;
}

export function hasParent(symbol: DoqoSymbol): boolean {
  return symbol.parent ? true : false;
}

export function fqidToPath(fqid: string): string {
  return fqid.replaceAll("::", "/");
}

export function pathToFqid(path: string): string {
  return path.replace(/\/$/, "").replaceAll("/", "::");
}

export function source(symbol: DoqoSymbol, registry: DoqoRegistry): string {
  const source = registry.sources[symbol.span.source_id];
  
  const encoder = new TextEncoder();
  const decoder = new TextDecoder();
  const bytes = encoder.encode(source.content);
  
  const slicedBytes = bytes.slice(symbol.span.start, symbol.span.end);
  
  const source_slice = decoder.decode(slicedBytes);
  
  console.debug(`Symbol: ${symbol.fqid}\nSource (${symbol.span.source_id}: ${symbol.span.start}-${symbol.span.end}):\n${source_slice}`);
  return source_slice;
}