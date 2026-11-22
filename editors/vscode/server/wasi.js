// A minimal WASI (preview 1) host for Node: enough for rhind, which, run
// by the extension, only reads standard input and writes standard output and
// standard error, both when it runs a page and when it is the language
// server. It needs no other library.
"use strict";

const fs = require("fs");

const ERRNO_BADF = 8;
const ERRNO_NOENT = 44;
const ERRNO_NOSYS = 52;
const FILETYPE_UNKNOWN = 0;
const FILETYPE_CHARACTER_DEVICE = 2;

const pause = new Int32Array(new SharedArrayBuffer(4));
const sleep = (ms) => Atomics.wait(pause, 0, 0, ms);

// Standard input may be a pipe that is not ready yet: wait and try again.
function readInput(buffer) {
  for (;;) {
    try {
      return fs.readSync(0, buffer, 0, buffer.length, null);
    } catch (error) {
      if (error.code === "EOF") return 0;
      if (error.code !== "EAGAIN") throw error;
      sleep(5);
    }
  }
}

function writeOutput(fd, bytes) {
  let done = 0;
  while (done < bytes.length) {
    try {
      done += fs.writeSync(fd, bytes, done, bytes.length - done);
    } catch (error) {
      if (error.code !== "EAGAIN") throw error;
      sleep(5);
    }
  }
}

/**
 * Runs a WASI command to completion and returns its exit code.
 *
 * @param {string} wasmPath   the compiled program
 * @param {object} [options]
 * @param {string[]} [options.args]      arguments after the program name
 * @param {Uint8Array} [options.prefix]  bytes the program reads before
 *        anything from the real standard input
 * @param {boolean} [options.keyboard]   false if the prefix is all there is
 *        to read: the real standard input is left alone, and the program is
 *        told that its input is not a console
 * @param {(fd: number, bytes: Uint8Array) => void} [options.output]
 *        where standard output (1) and standard error (2) go; by default,
 *        this process's own
 */
function run(wasmPath, { args = [], prefix = new Uint8Array(0), keyboard = true, output = writeOutput } = {}) {
  const module = new WebAssembly.Module(fs.readFileSync(wasmPath));
  let memory;
  let position = 0;
  const view = () => new DataView(memory.buffer);
  const bytes = () => new Uint8Array(memory.buffer);
  const argv = ["rhind", ...args].map((arg) => Buffer.from(arg + "\0"));

  const wasi = {
    args_sizes_get(countPtr, sizePtr) {
      view().setUint32(countPtr, argv.length, true);
      view().setUint32(sizePtr, argv.reduce((n, a) => n + a.length, 0), true);
      return 0;
    },
    args_get(argvPtr, bufPtr) {
      for (const arg of argv) {
        view().setUint32(argvPtr, bufPtr, true);
        bytes().set(arg, bufPtr);
        argvPtr += 4;
        bufPtr += arg.length;
      }
      return 0;
    },
    environ_sizes_get(countPtr, sizePtr) {
      view().setUint32(countPtr, 0, true);
      view().setUint32(sizePtr, 0, true);
      return 0;
    },
    environ_get() {
      return 0;
    },
    random_get(ptr, length) {
      require("crypto").randomFillSync(bytes(), ptr, length);
      return 0;
    },
    fd_read(fd, iovsPtr, iovsLen, nreadPtr) {
      if (fd !== 0) return ERRNO_BADF;
      let total = 0;
      for (let i = 0; i < iovsLen && total === 0; i++) {
        const ptr = view().getUint32(iovsPtr + i * 8, true);
        const len = view().getUint32(iovsPtr + i * 8 + 4, true);
        if (len === 0) continue;
        if (position < prefix.length) {
          const chunk = prefix.subarray(position, position + len);
          bytes().set(chunk, ptr);
          position += chunk.length;
          total = chunk.length;
        } else if (!keyboard) {
          break;
        } else {
          const buffer = Buffer.alloc(len);
          total = readInput(buffer);
          bytes().set(buffer.subarray(0, total), ptr);
          if (total === 0) break;
        }
      }
      view().setUint32(nreadPtr, total, true);
      return 0;
    },
    fd_write(fd, iovsPtr, iovsLen, nwrittenPtr) {
      if (fd !== 1 && fd !== 2) return ERRNO_BADF;
      let total = 0;
      for (let i = 0; i < iovsLen; i++) {
        const ptr = view().getUint32(iovsPtr + i * 8, true);
        const len = view().getUint32(iovsPtr + i * 8 + 4, true);
        if (len > 0) output(fd, bytes().slice(ptr, ptr + len));
        total += len;
      }
      view().setUint32(nwrittenPtr, total, true);
      return 0;
    },
    fd_fdstat_get(fd, statPtr) {
      if (fd > 2) return ERRNO_BADF;
      bytes().fill(0, statPtr, statPtr + 24);
      // Standard input is a console unless it is only the page being fed in.
      view().setUint8(statPtr, fd === 0 && !keyboard ? FILETYPE_UNKNOWN : FILETYPE_CHARACTER_DEVICE);
      return 0;
    },
    fd_close() {
      return 0;
    },
    clock_time_get(id, precision, timePtr) {
      view().setBigUint64(timePtr, BigInt(Date.now()) * 1000000n, true);
      return 0;
    },
    fd_filestat_get() {
      return ERRNO_BADF;
    },
    // No files: there are no preopened directories and nothing can be opened.
    fd_prestat_get() {
      return ERRNO_BADF;
    },
    fd_prestat_dir_name() {
      return ERRNO_BADF;
    },
    path_open() {
      return ERRNO_NOENT;
    },
    proc_exit(code) {
      throw new Exit(code);
    },
  };

  // Which calls a build imports depends on the toolchain that made it. One
  // this host has no answer for is given a stub that says "not supported".
  for (const { module: from, name } of WebAssembly.Module.imports(module)) {
    if (from === "wasi_snapshot_preview1" && !(name in wasi)) wasi[name] = () => ERRNO_NOSYS;
  }
  const instance = new WebAssembly.Instance(module, { wasi_snapshot_preview1: wasi });
  memory = instance.exports.memory;
  try {
    instance.exports._start();
    return 0;
  } catch (error) {
    if (error instanceof Exit) return error.code;
    throw error;
  }
}

class Exit extends Error {
  constructor(code) {
    super(`exit ${code}`);
    this.code = code;
  }
}

module.exports = { run };
