import loadWasm, {greet} from "./wasm/rustlog.js";

await loadWasm();
greet("Bill");
