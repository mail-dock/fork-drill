import assert from 'node:assert/strict';
import test from 'node:test';
import { fib } from './index.js';
test('fib', () => { assert.equal(fib(20), 6765); });
