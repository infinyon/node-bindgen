let addon = require('./dist');
const assert = require('assert');

assert.equal(addon.hello(2),"hello world 2"); 


assert.throws( () => addon.hello("hello"),{
    message: 'invalid type, expected: number, actual: string'
});



assert.throws(() => addon.hello(),{
    message: '1 args expected but 0 is present'
});       
console.log("function tests succeed");

assert.equal(addon.sum(1,2),3);

assert.throws( () => addon.minMax(10,0),{
    message: 'first arg is greater',   
  });

assert.equal(addon.minMax(1,2),3);
