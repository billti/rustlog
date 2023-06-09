import loadWasm, {greet, initLogging, doSomething, initTelemetry} from "./wasm/rustlog.js";

function setLogger() {
    // There's little value in using an internal function that closes over a
    // variable here, I just want to test that the Wasm holds a reference to
    // it correctly when later logging calls occur.
    const prefix = document.location.host;
    function logger(msg) {console.log(`${prefix} [${new Date().toISOString()}] ${msg}`);}
    initLogging(logger, 4);
}

await loadWasm();
setLogger();
initTelemetry((msg) => console.log(`[telemetry] ${msg}`));

const button = document.querySelector('#run-button')
    .addEventListener('click', () => doSomething());
