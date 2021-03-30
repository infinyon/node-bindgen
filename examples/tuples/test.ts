const assert = require('assert');

type MyTuple = [string, number[]];
interface TupleTester {
    capitalizeAndSquare(value: MyTuple): MyTuple
}


// Test that a well-formed tuple works as expected
const addon: TupleTester = require('./dist');
const item = addon.capitalizeAndSquare(["hello", [3, 5, 7]]);
const [capitalized, squared]: MyTuple = item;
assert.equal(capitalized, "HELLO");
assert.equal(squared[0], 9);
assert.equal(squared[1], 25);
assert.equal(squared[2], 49);


// Test that giving a tuple of the wrong size will fail
const addonUntyped: any = require('./dist');
assert.throws(
    () => { addonUntyped.capitalizeAndSquare(["hello"]) },
    { message: "2Tuple must have exactly length 2" },
);


// Test that giving a tuple with the wrong types will fail
assert.throws(
    () => {
        // This fails because the first item of the tuple should be a string
        addonUntyped.capitalizeAndSquare([ 5, [1, 2, 3] ]);
    },
    { message: "invalid type, expected: string, actual: number" },
);
