console.log('test');
let addon = require('./dist');
const assert = require('assert');

try {
console.log("Step 0")
console.log('addon.hello(2)', addon.hello(2));
assert.equal(addon.hello(2),"hello world 2"); 
console.log("Step 1")

assert.throws( () => addon.hello("hello"),{
    message: 'invalid type, expected: number, actual: string'
});
console.log("Step 2")


assert.throws(() => addon.hello(),{
    message: 'expected argument of type: i32'
});       
console.log("Step 3")

assert.equal(addon.sum(1,2),3);
console.log("Step 4")

assert.throws( () => addon.minMax(10,0),{
    message: 'first arg is greater',   
});
console.log("Step 5")

assert.equal(addon.minMax(1,2),3);
console.log("Step 6")

assert.equal(addon.multiply(2,5),10);
console.log("Step 7")

assert.equal(addon.sum2(10),10);
console.log("Step 8")

assert.equal(addon.sum2(5,100),105);
console.log("Step 9")

console.log("function tests succeed");

} catch (error) {
    console.error(error);
}