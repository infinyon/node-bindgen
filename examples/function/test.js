const crypto = require('crypto');

let addon =require('./dist');
const assert = require('assert');

assert.strictEqual(addon.hello(2),"hello world 2");

assert.throws( () => addon.hello("hello"),{
    message: 'invalid type, expected: number, actual: string'
});

assert.throws(() => addon.hello(),{
    message: 'expected argument of type: i32 at: 0'
});       

assert.strictEqual(addon.sum(1,2),3);

assert.throws( () => addon.minMax(10,0),{
    message: 'first arg is greater',
  });

assert.strictEqual(addon.minMax(1,2),3);

assert.strictEqual(addon.multiply(2,5),10);

assert.strictEqual(addon.sum2(10),10);
assert.strictEqual(addon.sum2(5,100),105);

const stringShort = _generateForCustomCharacters(5);
const stringMedium = _generateForCustomCharacters(100);
const stringLong = _generateForCustomCharacters(2000);
const strings = new Set([stringShort, stringMedium, stringLong]);

assert.strictEqual(addon.string(stringShort), stringShort);
assert.strictEqual(addon.string(stringMedium), stringMedium);
assert.strictEqual(addon.string(stringLong), stringLong);

for(const string1 in strings) {
    for(const string2 in strings) {
        assert.strictEqual(addon.string(string1), string2);
    }
}

console.log("function tests succeed");

/*
 * attribution: https://github.com/sindresorhus/crypto-random-string
 * MIT License
 * Copyright (c) Sindre Sorhus <sindresorhus@gmail.com> (sindresorhus.com)
 */
function _generateForCustomCharacters(length, characters) {
    characters = characters || '!"#$%&\'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~'.split('');
	// Generating entropy is faster than complex math operations, so we use the simplest way
	const characterCount = characters.length;
	const maxValidSelector = (Math.floor(0x10000 / characterCount) * characterCount) - 1; // Using values above this will ruin distribution when using modular division
	const entropyLength = 2 * Math.ceil(1.1 * length); // Generating a bit more than required so chances we need more than one pass will be really low
	let string = '';
	let stringLength = 0;

	while (stringLength < length) { // In case we had many bad values, which may happen for character sets of size above 0x8000 but close to it
		const entropy = crypto.randomBytes(entropyLength);
		let entropyPosition = 0;

		while (entropyPosition < entropyLength && stringLength < length) {
			const entropyValue = entropy.readUInt16LE(entropyPosition);
			entropyPosition += 2;
			if (entropyValue > maxValidSelector) { // Skip values which will ruin distribution when using modular division
				continue;
			}

			string += characters[entropyValue % characterCount];
			stringLength++;
		}
	}

	return string;
}
