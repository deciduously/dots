(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[0],{

/***/ "../pkg/dots.js":
/*!**********************!*\
  !*** ../pkg/dots.js ***!
  \**********************/
/*! exports provided: Game, GameConfig, GameInstance, __wbg_now_b5cbb83d2079a9cc, __wbg_random_0167f35ba217be9a, __wbg_new_59cb74e423758ede, __wbg_stack_558ba5917b466edd, __wbg_error_4bb6c2a97407129a, __wbindgen_object_drop_ref, __wbindgen_throw */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var _dots_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./dots_bg.wasm */ \"../pkg/dots_bg.wasm\");\n/* harmony import */ var _dots_bg_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./dots_bg.js */ \"../pkg/dots_bg.js\");\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"Game\", function() { return _dots_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"Game\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"GameConfig\", function() { return _dots_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"GameConfig\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"GameInstance\", function() { return _dots_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"GameInstance\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_now_b5cbb83d2079a9cc\", function() { return _dots_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_now_b5cbb83d2079a9cc\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_random_0167f35ba217be9a\", function() { return _dots_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_random_0167f35ba217be9a\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_new_59cb74e423758ede\", function() { return _dots_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_new_59cb74e423758ede\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_stack_558ba5917b466edd\", function() { return _dots_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_stack_558ba5917b466edd\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_error_4bb6c2a97407129a\", function() { return _dots_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_error_4bb6c2a97407129a\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_object_drop_ref\", function() { return _dots_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_object_drop_ref\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_throw\", function() { return _dots_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_throw\"]; });\n\n\n\n\n//# sourceURL=webpack:///../pkg/dots.js?");

/***/ }),

/***/ "../pkg/dots_bg.js":
/*!*************************!*\
  !*** ../pkg/dots_bg.js ***!
  \*************************/
/*! exports provided: Game, GameConfig, GameInstance, __wbg_now_b5cbb83d2079a9cc, __wbg_random_0167f35ba217be9a, __wbg_new_59cb74e423758ede, __wbg_stack_558ba5917b466edd, __wbg_error_4bb6c2a97407129a, __wbindgen_object_drop_ref, __wbindgen_throw */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* WEBPACK VAR INJECTION */(function(module) {/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"Game\", function() { return Game; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"GameConfig\", function() { return GameConfig; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"GameInstance\", function() { return GameInstance; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_now_b5cbb83d2079a9cc\", function() { return __wbg_now_b5cbb83d2079a9cc; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_random_0167f35ba217be9a\", function() { return __wbg_random_0167f35ba217be9a; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_new_59cb74e423758ede\", function() { return __wbg_new_59cb74e423758ede; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_stack_558ba5917b466edd\", function() { return __wbg_stack_558ba5917b466edd; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_error_4bb6c2a97407129a\", function() { return __wbg_error_4bb6c2a97407129a; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_object_drop_ref\", function() { return __wbindgen_object_drop_ref; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_throw\", function() { return __wbindgen_throw; });\n/* harmony import */ var _dots_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./dots_bg.wasm */ \"../pkg/dots_bg.wasm\");\n\n\nconst heap = new Array(32).fill(undefined);\n\nheap.push(undefined, null, true, false);\n\nfunction getObject(idx) { return heap[idx]; }\n\nlet heap_next = heap.length;\n\nfunction dropObject(idx) {\n    if (idx < 36) return;\n    heap[idx] = heap_next;\n    heap_next = idx;\n}\n\nfunction takeObject(idx) {\n    const ret = getObject(idx);\n    dropObject(idx);\n    return ret;\n}\n\nconst lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;\n\nlet cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });\n\ncachedTextDecoder.decode();\n\nlet cachegetUint8Memory0 = null;\nfunction getUint8Memory0() {\n    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== _dots_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetUint8Memory0 = new Uint8Array(_dots_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetUint8Memory0;\n}\n\nfunction getStringFromWasm0(ptr, len) {\n    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));\n}\n\nfunction notDefined(what) { return () => { throw new Error(`${what} is not defined`); }; }\n\nfunction addHeapObject(obj) {\n    if (heap_next === heap.length) heap.push(heap.length + 1);\n    const idx = heap_next;\n    heap_next = heap[idx];\n\n    heap[idx] = obj;\n    return idx;\n}\n\nlet WASM_VECTOR_LEN = 0;\n\nconst lTextEncoder = typeof TextEncoder === 'undefined' ? (0, module.require)('util').TextEncoder : TextEncoder;\n\nlet cachedTextEncoder = new lTextEncoder('utf-8');\n\nconst encodeString = (typeof cachedTextEncoder.encodeInto === 'function'\n    ? function (arg, view) {\n    return cachedTextEncoder.encodeInto(arg, view);\n}\n    : function (arg, view) {\n    const buf = cachedTextEncoder.encode(arg);\n    view.set(buf);\n    return {\n        read: arg.length,\n        written: buf.length\n    };\n});\n\nfunction passStringToWasm0(arg, malloc, realloc) {\n\n    if (realloc === undefined) {\n        const buf = cachedTextEncoder.encode(arg);\n        const ptr = malloc(buf.length);\n        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);\n        WASM_VECTOR_LEN = buf.length;\n        return ptr;\n    }\n\n    let len = arg.length;\n    let ptr = malloc(len);\n\n    const mem = getUint8Memory0();\n\n    let offset = 0;\n\n    for (; offset < len; offset++) {\n        const code = arg.charCodeAt(offset);\n        if (code > 0x7F) break;\n        mem[ptr + offset] = code;\n    }\n\n    if (offset !== len) {\n        if (offset !== 0) {\n            arg = arg.slice(offset);\n        }\n        ptr = realloc(ptr, len, len = offset + arg.length * 3);\n        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);\n        const ret = encodeString(arg, view);\n\n        offset += ret.written;\n    }\n\n    WASM_VECTOR_LEN = offset;\n    return ptr;\n}\n\nlet cachegetInt32Memory0 = null;\nfunction getInt32Memory0() {\n    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== _dots_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetInt32Memory0 = new Int32Array(_dots_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetInt32Memory0;\n}\n/**\n*/\nclass Game {\n\n    static __wrap(ptr) {\n        const obj = Object.create(Game.prototype);\n        obj.ptr = ptr;\n\n        return obj;\n    }\n\n    free() {\n        const ptr = this.ptr;\n        this.ptr = 0;\n\n        _dots_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbg_game_free\"](ptr);\n    }\n    /**\n    */\n    constructor() {\n        var ret = _dots_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"game_new\"]();\n        return Game.__wrap(ret);\n    }\n    /**\n    * @returns {number}\n    */\n    height() {\n        var ret = _dots_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"game_height\"](this.ptr);\n        return ret;\n    }\n    /**\n    * @returns {number}\n    */\n    width() {\n        var ret = _dots_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"game_width\"](this.ptr);\n        return ret;\n    }\n    /**\n    */\n    tick() {\n        _dots_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"game_tick\"](this.ptr);\n    }\n    /**\n    * @param {number} x\n    * @param {number} y\n    */\n    handle_click(x, y) {\n        _dots_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"game_handle_click\"](this.ptr, x, y);\n    }\n    /**\n    * @returns {number}\n    */\n    header() {\n        var ret = _dots_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"game_header\"](this.ptr);\n        return ret;\n    }\n    /**\n    * @returns {number}\n    */\n    pack() {\n        var ret = _dots_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"game_pack\"](this.ptr);\n        return ret;\n    }\n    /**\n    * @returns {number}\n    */\n    score() {\n        var ret = _dots_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"game_score\"](this.ptr);\n        return ret;\n    }\n}\n/**\n*/\nclass GameConfig {\n\n    free() {\n        const ptr = this.ptr;\n        this.ptr = 0;\n\n        _dots_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbg_gameconfig_free\"](ptr);\n    }\n}\n/**\n*/\nclass GameInstance {\n\n    free() {\n        const ptr = this.ptr;\n        this.ptr = 0;\n\n        _dots_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbg_gameinstance_free\"](ptr);\n    }\n}\n\nconst __wbg_now_b5cbb83d2079a9cc = typeof Date.now == 'function' ? Date.now : notDefined('Date.now');\n\nconst __wbg_random_0167f35ba217be9a = typeof Math.random == 'function' ? Math.random : notDefined('Math.random');\n\nconst __wbg_new_59cb74e423758ede = function() {\n    var ret = new Error();\n    return addHeapObject(ret);\n};\n\nconst __wbg_stack_558ba5917b466edd = function(arg0, arg1) {\n    var ret = getObject(arg1).stack;\n    var ptr0 = passStringToWasm0(ret, _dots_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"], _dots_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_realloc\"]);\n    var len0 = WASM_VECTOR_LEN;\n    getInt32Memory0()[arg0 / 4 + 1] = len0;\n    getInt32Memory0()[arg0 / 4 + 0] = ptr0;\n};\n\nconst __wbg_error_4bb6c2a97407129a = function(arg0, arg1) {\n    try {\n        console.error(getStringFromWasm0(arg0, arg1));\n    } finally {\n        _dots_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_free\"](arg0, arg1);\n    }\n};\n\nconst __wbindgen_object_drop_ref = function(arg0) {\n    takeObject(arg0);\n};\n\nconst __wbindgen_throw = function(arg0, arg1) {\n    throw new Error(getStringFromWasm0(arg0, arg1));\n};\n\n\n/* WEBPACK VAR INJECTION */}.call(this, __webpack_require__(/*! ./../www/node_modules/webpack/buildin/harmony-module.js */ \"./node_modules/webpack/buildin/harmony-module.js\")(module)))\n\n//# sourceURL=webpack:///../pkg/dots_bg.js?");

/***/ }),

/***/ "../pkg/dots_bg.wasm":
/*!***************************!*\
  !*** ../pkg/dots_bg.wasm ***!
  \***************************/
/*! exports provided: memory, __wbg_gameconfig_free, __wbg_gameinstance_free, __wbg_game_free, game_new, game_height, game_width, game_tick, game_handle_click, game_header, game_pack, game_score, __wbindgen_free, __wbindgen_malloc, __wbindgen_realloc */
/***/ (function(module, exports, __webpack_require__) {

eval("\"use strict\";\n// Instantiate WebAssembly module\nvar wasmExports = __webpack_require__.w[module.i];\n__webpack_require__.r(exports);\n// export exports from WebAssembly module\nfor(var name in wasmExports) if(name != \"__webpack_init__\") exports[name] = wasmExports[name];\n// exec imports from WebAssembly module (for esm order)\n/* harmony import */ var m0 = __webpack_require__(/*! ./dots_bg.js */ \"../pkg/dots_bg.js\");\n\n\n// exec wasm module\nwasmExports[\"__webpack_init__\"]()\n\n//# sourceURL=webpack:///../pkg/dots_bg.wasm?");

/***/ }),

/***/ "./index.js":
/*!******************!*\
  !*** ./index.js ***!
  \******************/
/*! no exports provided */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var dots__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! dots */ \"../pkg/dots.js\");\n/* harmony import */ var dots_dots_bg__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! dots/dots_bg */ \"../pkg/dots_bg.wasm\");\n\n\n\nconst finalRadius = 50.0;\n\nvar game = new dots__WEBPACK_IMPORTED_MODULE_0__[\"Game\"]()\n\n// So the idea here is to put all thje drawing fns into like the example to call from Rust\n// then in Rust, create a draw: thing that does all of it - get the attempts stuff outta the DOM you idiot\n// and then back here call module.draw() in our event loop.\n\n// I think this means we can stop doing packing at all ?!\n\n// Set up the score\n// TODO REMOVE - you're bringing this back into the module\n//const attempts = document.getElementById('current-attempts')\n//const totalAttempts = document.getElementById('total-attempts')\n\n//const updateScore = () => {\n//  let scorePtr = game.score()\n//  let score = new Uint32Array(memory.buffer, scorePtr, 2)\n//  let newTotalAttempts = score[0]\n//  let newCurrentAttempts = score[1]\n//  attempts.innerHTML = newCurrentAttempts\n//  totalAttempts.innerHTML = newTotalAttempts\n//}\n\n// The following pattern is borrowed from https://github.com/aochagavia/rocket_wasm/blob/master/html/index.html\n\n// Returns an object containing the dot canvas resource\nfunction dotCanvas() {\n  let res = {\n    dot: document.createElement('canvas'),\n  }\n\n  res.dot.width = finalRadius // the biggest it will ever be\n  res.dot.height = finalRadius\n  let dCtx = res.dot.getContext('2d')\n  dCtx.beginPath()\n  // use an arc from 0 to 2pi to draw a full circle\n  // the actual size is handled down in imports()\n  dCtx.arc(10, 10, 25, 0, 2 * Math.PI, false)\n  //dCtx.fillStyle = color // see if you cna do it in imports()\n  dCtx.fill()\n  dCtx.stroke()\n\n}\n\n// set up the render context\nconst canvas = document.getElementById('dots-canvas')\nconst height = game.height()\nconst width = game.width()\ncanvas.height = height\ncanvas.width = width\nconst ctx = canvas.getContext('2d')\nctx.globalAlpha = 0.8 // everything's a little transparent\n\n// Canvas click handler\ncanvas.addEventListener('click', event => {\n  // translate from page coords to canvas coords\n  // shamelessly lifted from the RustWasm book\n  // https://rustwasm.github.io/book/game-of-life/interactivity.html\n  const boundingRect = canvas.getBoundingClientRect()\n\n  const scaleX = canvas.width / boundingRect.width\n  const scaleY = canvas.height / boundingRect.height\n\n  const canvasX = (event.clientX - boundingRect.left) * scaleX\n  const canvasY = (event.clientY - boundingRect.top) * scaleY\n\n  game.handle_click(canvasX, canvasY)\n})\n\n// define the main loop, updated 60 times per second\nconst renderLoop = () => {\n  // Start with a blank slate\n  ctx.clearRect(0, 0, width, height)\n\n  // tick us forward and grab the packed version\n  game.tick()\n  const headerPtr = game.header()\n\n  // read header\n  // level_number | level_state | total_dots | win_threshold | captured_dots\n  const levelData = new Uint8Array(dots_dots_bg__WEBPACK_IMPORTED_MODULE_1__[\"memory\"].buffer, headerPtr, 5)\n  const level = levelData[0]\n  const levelState = levelData[1]\n  const totalDots = levelData[2]\n  const winThreshold = levelData[3]\n  const capturedDots = levelData[4]\n\n  // LevelState:\n  // Begin = 0,\n  // Waiting = 1,\n  // Clicked = 2,\n  // Won = 3,\n  // Lost = 4\n\n  switch (levelState) {\n    case 0: {\n      drawBeginLevel(level, winThreshold, totalDots)\n      window.requestAnimationFrame(renderLoop)\n      break\n    }\n    case 1:\n    case 2: {\n      // get dots\n      const dataLength = totalDots * 7\n      const dotsPtr = game.pack()\n      const dots = new Float32Array(dots_dots_bg__WEBPACK_IMPORTED_MODULE_1__[\"memory\"].buffer, dotsPtr, dataLength)\n\n      drawGame(dots, level, totalDots, winThreshold, capturedDots, levelState)\n      window.requestAnimationFrame(renderLoop)\n      break\n    }\n    case 3: {\n      drawNextLevel(level)\n      window.requestAnimationFrame(renderLoop)\n      break\n    }\n    case 4: {\n      drawRestartLevel(level)\n      window.requestAnimationFrame(renderLoop)\n      break\n    }\n    default: { }\n  }\n}\n\n// DRAW FNS\n\nconst drawLevelButton = (text, colorStr) => {\n  ctx.beginPath()\n  ctx.rect(325, 275, 150, 50)\n  ctx.stroke()\n  ctx.font = '14px serif'\n  ctx.fillStyle = colorStr\n  ctx.fillText(text, 330, 305)\n}\n\nconst drawBeginLevel = (level, winThreshold) => {\n  drawLevelButton('Level ' + level + ' - capture ' + winThreshold, 'purple')\n}\n\nconst drawRestartLevel = (level) => {\n  drawLevelButton('Too bad! Retry ' + level, 'red')\n}\n\nconst drawNextLevel = (level) => {\n  const nextText = (level >= 12) ? 'Game over!  Restart game?' : 'Nice job! Level ' + (level + 1)\n  drawLevelButton(nextText, 'green')\n}\n\nconst drawGame = (dots, level, totalDots, winThreshold, capturedDots, levelState) => {\n  drawProgressCounter(capturedDots, totalDots, winThreshold, levelState)\n  drawLevelNumber(level)\n\n  let dotsLength = dots.length\n  for (let idx = 0; idx < dotsLength; idx += 7) {\n    drawDot(dots.slice(idx, idx + 7))\n  }\n}\n\nconst drawProgressCounter = (capturedDots, totalDots, winThreshold, levelState) => {\n  const won = capturedDots >= winThreshold\n  const levelDots = (levelState === 1) ? totalDots : totalDots - 1\n  ctx.font = '22px serif'\n  ctx.fillStyle = won ? 'green' : 'red'\n  ctx.fillText(capturedDots + '/' + levelDots + ' - goal: ' + winThreshold, 10, 42)\n}\n\nconst drawLevelNumber = level => {\n  ctx.font = '20px serif'\n  ctx.fillStyle = 'blue'\n  ctx.fillText('level ' + level, 10, 70)\n}\n\nconst drawDot = packedDot => {\n  // x | y | radius | DotState | r | g | b\n  if (packedDot[3] !== 5.0) {\n    const posX = packedDot[0]\n    const posY = packedDot[1]\n    const radius = packedDot[2]\n    const color = colorString(packedDot[4], packedDot[5], packedDot[6])\n\n    ctx.beginPath()\n    // use an arc from 0 to 2pi to draw a full circle\n    ctx.arc(posX, posY, radius, 0, 2 * Math.PI, false)\n    ctx.fillStyle = color\n    ctx.fill()\n    ctx.stroke()\n  }\n}\n\nconst colorString = (r, g, b) => '#' + Math.floor(r).toString(16) + Math.floor(g).toString(16) + Math.floor(b).toString(16)\n\n// INIT\n\nwindow.requestAnimationFrame(renderLoop)\n\n\n//# sourceURL=webpack:///./index.js?");

/***/ }),

/***/ "./node_modules/webpack/buildin/harmony-module.js":
/*!*******************************************!*\
  !*** (webpack)/buildin/harmony-module.js ***!
  \*******************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("module.exports = function(originalModule) {\n\tif (!originalModule.webpackPolyfill) {\n\t\tvar module = Object.create(originalModule);\n\t\t// module.parent = undefined by default\n\t\tif (!module.children) module.children = [];\n\t\tObject.defineProperty(module, \"loaded\", {\n\t\t\tenumerable: true,\n\t\t\tget: function() {\n\t\t\t\treturn module.l;\n\t\t\t}\n\t\t});\n\t\tObject.defineProperty(module, \"id\", {\n\t\t\tenumerable: true,\n\t\t\tget: function() {\n\t\t\t\treturn module.i;\n\t\t\t}\n\t\t});\n\t\tObject.defineProperty(module, \"exports\", {\n\t\t\tenumerable: true\n\t\t});\n\t\tmodule.webpackPolyfill = 1;\n\t}\n\treturn module;\n};\n\n\n//# sourceURL=webpack:///(webpack)/buildin/harmony-module.js?");

/***/ })

}]);