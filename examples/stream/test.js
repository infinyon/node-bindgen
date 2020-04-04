const assert = require('assert');
let addon = require('./dist');


const EventEmitter = require('events').EventEmitter;
const emitter = new EventEmitter();

let sum = 0;

emitter.on('data', (evt) => {
    console.log("received event",evt);
    sum = sum + evt;
})

let factory = new addon.StreamFactory();

// test for error 
assert.throws( () => factory.stream(20,emitter.emit.bind(emitter)),{
  message: 'count: 20 should be less than or equal to 10'
});


factory.stream(10,emitter.emit.bind(emitter));

console.log("stream started");

// wait for stream to finish, since stream produce event at every 100ms we should wait at least 1 second
setTimeout(() => {
  console.log("timer finished");
  assert.equal(sum,45);
}, 3000); // Made a bit larger so it reliably works on Travis
